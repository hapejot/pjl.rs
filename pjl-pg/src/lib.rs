use std::time::Duration;

use edm::{number::Number, primitive::PrimitiveValue, value::Value};
use pjl_odata::DbSpecifics;
use pjl_tab::Table;
use tokio::time::timeout;
use tokio_postgres::{connect, types::ToSql, Client, NoTls};
use tracing::*;

pub struct Database {
    client: Client,
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
    pub async fn new(connection: &str) -> Self {
        match connect(connection, NoTls).await {
            Ok((client, conn)) => {
                // The connection object performs the actual communication with the database,
                // so spawn it off to run on its own.
                tokio::spawn(async move {
                    if let Err(e) = conn.await {
                        error!("connection error: {}", e);
                    }
                });
                Self { client: client }
            }
            Err(_) => todo!(),
        }
    }

    pub fn connected(&self) -> bool {
        true
    }

    pub async fn select(&mut self, q: pjl_odata::ODataQuery) -> Table {
        debug!("query: {:#?}",q);
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
                        _ => todo!("implement conversion of type '{}'", ty.name()),
                    }
                    // let val: String = row.get(idx);
                    // rrow.set(c.name(), &val);
                }
            }
        } else {
            panic!("timout or other error...");
        }
        r
    }

    pub fn modify(&self, arg: &str, tab: Table) {
        let _ = tab;
        let _ = arg;
        todo!()
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
