use edm::{csdl::Key, number::Number, primitive::PrimitiveValue, value::Value};
use mini_moka::sync::Cache;
use pjl_odata::DbSpecifics;
use pjl_tab::Table;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;
use tokio_postgres::{connect, types::ToSql, Client, NoTls};
use tracing::*;

pub struct Database {
    client: Client,
    primary_keys: Cache<String, Vec<KeyPart>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyPart {
    field: String,
}
impl std::fmt::Display for KeyPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.field)
    }
}
struct PostgresQuery {
    fld: Option<String>,
    params: Vec<Value>,
    arm: Vec<String>,
    arms: Vec<String>,
}

impl PostgresQuery {
    fn new() -> Self {
        Self {
            params: vec![],
            arm: vec![],
            arms: vec![],
            fld: None,
        }
    }
}

impl DbSpecifics for PostgresQuery {
    fn start_field(&mut self, name: &str) {
        self.fld = Some(name.into());
        self.arm = vec![];
    }

    fn end_field(&mut self) {
        assert!(self.fld.is_some());
        assert!(self.arm.len() > 0, "fld {:?}", self.fld);
        if self.arm.len() == 1 {
            self.arms.push(self.arm[0].clone());
        } else {
            self.arms.push(format!("({})", self.arm.join(" or ")));
        }
    }

    fn add_cond(&mut self, op: &str, value: &str) {
        match op {
            "=" => {
                self.params.push(value.into());
                self.arm.push(format!(
                    "{} = ${}",
                    self.fld.as_ref().unwrap(),
                    self.params.len()
                ));
            }
            _ => todo!("operator {op}"),
        }
    }

    fn where_clause(&self) -> String {
        self.arms.join(" and ")
    }

    fn values(&self) -> Vec<Value> {
        self.params.clone()
    }
}

impl Database {
    pub async fn new(connection: &str) -> Result<Self, String> {
        match connect(connection, NoTls).await {
            Ok((client, conn)) => {
                // The connection object performs the actual communication with the database,
                // so spawn it off to run on its own.
                tokio::spawn(async move {
                    if let Err(e) = conn.await {
                        error!("connection error: {}", e);
                    }
                });
                Ok(Self {
                    client: client,
                    primary_keys: Cache::new(10),
                })
            }
            Err(e) => Err(format!("{e}")),
        }
    }

    pub fn connected(&self) -> bool {
        true
    }

    pub async fn read_primary_key(&mut self, tab_name: &str) -> Vec<KeyPart> {
        let tab_name = tab_name.to_string();
        if !self.primary_keys.contains_key(&tab_name) {
            let client = &mut self.client;
            let r = Table::new();

            if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
                let query = format!(
                    r#" SELECT a.attname as field
                    FROM   pg_index i
                    JOIN   pg_attribute a ON a.attrelid = i.indrelid
                                        AND a.attnum   = ANY(i.indkey)
                    WHERE  i.indrelid = to_regclass($1)
                    AND    i.indisprimary;"#
                );
                let stmt = t.prepare(&query).await.unwrap();
                // let tab_name: Box<dyn ToSql + Sync> = ;
                let rows = t
                    .query(&stmt, &[&*Box::new(tab_name.clone())])
                    .await
                    .expect("query");
                extract_result_to_table(&r, rows);
                self.primary_keys.insert(
                    tab_name.clone(),
                    pjl_tab::de::extract_from_table(&r).unwrap(),
                );
            } else {
                todo!()
            }
        }
        self.primary_keys.get(&tab_name).unwrap()
    }

    pub async fn select(&mut self, q: pjl_odata::ODataQuery) -> Table {
        debug!("query: {:#?}", q);
        let (where_clause, mut params) = q.get_where_sql_specific(PostgresQuery::new());
        let sql = if where_clause.len() > 0 {
            format!("SELECT * FROM {} WHERE {}", q.get_table(), where_clause)
        } else {
            format!("SELECT * FROM {}", q.get_table())
        };
        trace!("SQL: {sql}");
        let client = &mut self.client;
        let r = Table::new();

        if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
            let statement = t.prepare(&sql).await.unwrap();
            let mut sql_params: Vec<Box<dyn ToSql + Sync + Send>> = vec![];
            for (idx, typ) in statement.params().iter().enumerate() {
                let x = params.get_mut(idx).unwrap();
                match x {
                    Value::PrimitiveValue(primitive_value) => match primitive_value {
                        PrimitiveValue::Null => todo!(),
                        PrimitiveValue::Boolean(_) => todo!(),
                        PrimitiveValue::Decimal(number) => match typ.name() {
                            "int4" => sql_params.push(Box::new(number.as_i64())),
                            _ => sql_params.push(Box::new(number.as_str().to_string())),
                        },
                        PrimitiveValue::String(v) => match typ.name() {
                            "int4" => sql_params.push(Box::new(v.parse::<i32>().unwrap())),
                            _ => sql_params.push(Box::new(v.clone())),
                        },
                        PrimitiveValue::Custom { datatype, value } => todo!(),
                    },
                    Value::StructureValue(structure_value) => todo!(),
                    Value::ListValue(list_value) => todo!(),
                }
            }
            assert_eq!(params.len(), sql_params.len());
            let final_sql_params = sql_params
                .iter()
                .map(|x| x.as_ref() as &(dyn ToSql + Sync))
                .collect::<Vec<_>>();

            let rows = t.query(&statement, &final_sql_params).await.expect("query");
            extract_result_to_table(&r, rows);
        } else {
            panic!("timout or other error...");
        }
        r
    }

    pub async fn modify(&mut self, tab_name: &str, rows: Table) {
        let pks = self.read_primary_key(tab_name).await;
        let pk = pks.iter().nth(0).unwrap();

        println!("pk: {pk}");
        let v = rows
            .columns()
            .iter()
            .enumerate()
            .map(|(x, y)| (format!("{}", y), format!("${}", x + 1)))
            .collect::<Vec<_>>();
        let ins_fields = v
            .iter()
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>()
            .join(", ");
        let ins_values = v
            .iter()
            .map(|(_, k)| k.clone())
            .collect::<Vec<_>>()
            .join(", ");

        let upd = v
            .iter()
            .map(|(k, _)| format!("{k} = EXCLUDED.{k}"))
            .collect::<Vec<_>>()
            .join(", ");
        // sample insert or update statement:
        // INSERT INTO my_table (id, name, value)
        // VALUES (1, 'example', '123')
        // ON CONFLICT (id)
        // DO UPDATE SET
        //     name = EXCLUDED.name,
        //     value = EXCLUDED.value
        // WHERE my_table.value IS DISTINCT FROM EXCLUDED.value;

        let query = format!(
            "INSERT INTO {} ({})  VALUES ({}) ON CONFLICT ({pk}) DO UPDATE SET {upd}",
            tab_name, ins_fields, ins_values
        );
        println!("{query}");
        let client = &mut self.client;
        let r = Table::new();
        if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
            // prepare statement for loop eventually..
            let stmt = t.prepare(&query).await.unwrap();

            // for all data in the table:
            for idx in 1..=rows.lines() {
                let mut sql_params: Vec<Box<dyn ToSql + Sync + Send>> = vec![];
                let current_row = rows.row(idx);
                for c in rows.columns().iter() {
                    let value = match current_row.get(c) {
                        Some(x) => x,
                        None => String::new(),
                    };
                    sql_params.push(Box::new(value));
                }
                let final_sql_params = sql_params
                    .iter()
                    .map(|x| x.as_ref() as &(dyn ToSql + Sync))
                    .collect::<Vec<_>>();
                let r = t.execute(&stmt, &final_sql_params).await.expect("query");
                println!("query: {} -> {}", query, r);
            }
            t.commit().await.unwrap();
        } else {
            panic!("no transaction found");
        }
    }
}

fn extract_result_to_table(r: &Table, rows: Vec<tokio_postgres::Row>) {
    trace!("result rows {}", rows.len());
    for row in rows {
        let rrow = r.new_row();
        for (idx, c) in row.columns().iter().enumerate() {
            let ty = c.type_();
            match ty.name() {
                "varchar" => {
                    rrow.set(c.name(), row.get(idx));
                }
                "text" => {
                    rrow.set(c.name(), row.get(idx));
                }
                "int4" => {
                    let i = row.get::<'_, _, i32>(idx);
                    rrow.set(c.name(), &i.to_string());
                }
                "name" => {
                    rrow.set(c.name(), row.get(idx));
                }
                _ => todo!("implement conversion of type '{}'", ty.name()),
            }
        }
    }
}

fn params_from_edm<'a>(params: &'a Vec<Value>) -> Vec<&'a (dyn ToSql + Sync)> {
    let mut result: Vec<&'a (dyn ToSql + Sync)> = vec![];
    for x in params.iter() {
        match x {
            Value::PrimitiveValue(primitive_value) => match primitive_value {
                PrimitiveValue::Null => todo!(),
                PrimitiveValue::Boolean(v) => result.push(v),
                PrimitiveValue::Decimal(_number) => todo!(),
                PrimitiveValue::String(v) => result.push(v),
                PrimitiveValue::Custom {
                    datatype: _,
                    value: _,
                } => todo!(),
            },
            Value::StructureValue(_structure_value) => todo!(),
            Value::ListValue(_list_value) => todo!(),
        }
    }
    result
}
