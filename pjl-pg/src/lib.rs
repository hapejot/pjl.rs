use edm::{csdl::Key, number::Number, primitive::PrimitiveValue, value::Value, Schema};
use mini_moka::sync::Cache;
use pjl_odata::DbSpecifics;
use pjl_tab::Table;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;
use tokio_postgres::{
    connect,
    types::{Oid, ToSql},
    Client, NoTls,
};
use tracing::*;

const DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%Z";
const DATE_TIME_FORMAT_OUT: &str = "%Y-%m-%dT%H:%M:%SZ";

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Decl {
    pkpos: Option<i32>,
    attname: String,
    atttypid: i32,
    attnum: i32,
    attnotnull: bool,
    typname: String,
    typlen: i32,
    atttypmod: i32,
    column_type: String,
}

#[derive(Clone, Debug)]
struct TableMetadata {
    colspecs: Vec<Decl>,
}

pub struct Database {
    client: Client,
    primary_keys: Cache<String, Vec<KeyPart>>,
    table_meta: Cache<String, TableMetadata>,
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
            "ne" => {
                self.params.push(value.into());
                self.arm.push(format!(
                    "{} <> ${}",
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

struct ColInfo {
    pos: usize,
    name: String,
    varname: String,
    coltype: String,
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
                    table_meta: Cache::new(10),
                })
            }
            Err(e) => Err(format!("{e}")),
        }
    }

    pub fn connected(&self) -> bool {
        true
    }

    async fn read_table_metadata(&mut self, tab_name: &str) {
        let client = &mut self.client;
        let r = Table::new();

        if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
            let query = format!(
                r#"select array_position(i.indkey, a.attnum) as pkpos, 
                            a.attname, 
                            atttypid,
                            attnum, 
                            attnotnull, 
                            t.typname, 
                            t.typlen, 
                            a.atttypmod,
                            format_type(atttypid, atttypmod) AS column_type
                            from pg_attribute as a
                            join pg_type as t on a.atttypid = t.oid
                            left outer join pg_index as i on i.indrelid = a.attrelid and a.attnum = any (i.indkey)
                            where  a.attrelid = to_regclass($1)
                              and a.attnum > 0"#
            );
            let stmt = t.prepare(&query).await.unwrap();
            // let tab_name: Box<dyn ToSql + Sync> = ;
            let rows = t
                .query(&stmt, &[&*Box::new(tab_name.clone())])
                .await
                .expect("query");
            let r = Table::new();
            extract_result_to_table(&r, rows);
            // let mut s = String::new();
            // r.dump(&mut s);
            // eprintln!("meta:\n{s}");
            let colspecs: Vec<Decl> = pjl_tab::de::extract_from_table(&r).unwrap();
            debug!("colspecs: {colspecs:#?}");
            self.table_meta
                .insert(tab_name.to_string(), TableMetadata { colspecs });
        }
    }

    async fn get_table_metadata(&mut self, tab_name: &str) -> TableMetadata {
        let tab_name = tab_name.to_string();
        if !self.table_meta.contains_key(&tab_name) {
            self.read_table_metadata(&tab_name).await;
        }
        self.table_meta.get(&tab_name).unwrap()
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

    pub async fn modify(&mut self, tab_name: &str, tab: Table) {
        let meta = self.get_table_metadata(tab_name).await;

        let pks = self.read_primary_key(tab_name).await;
        let pk = pks.iter().nth(0).unwrap();

        let mut colinfos = vec![];
        // enrich the columns of the paramter table with data from the underlying metadata
        for (pos, name) in tab.columns().iter().enumerate() {
            let m = meta.colspecs.iter().find(|x| x.attname == *name);
            colinfos.push(ColInfo {
                pos,
                name: name.clone(),
                varname: format!("${}", pos + 1),
                coltype: if let Some(d) = m {
                    d.typname.clone()
                } else {
                    String::from("varchar")
                },
            });
        }

        let ins_fields = colinfos
            .iter()
            .map(|ci| ci.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        let ins_values = colinfos
            .iter()
            .map(|ci| ci.varname.clone())
            .collect::<Vec<_>>()
            .join(", ");

        let upd = colinfos
            .iter()
            .map(|ci| format!("{} = EXCLUDED.{}", ci.name, ci.name))
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
        trace!("{query}");
        let client = &mut self.client;
        let r = Table::new();
        if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
            // prepare statement for loop eventually..
            let stmt = t.prepare(&query).await.unwrap();

            // for all data in the table:
            for idx in 1..=tab.lines() {
                let mut sql_params: Vec<Box<dyn ToSql + Sync + Send>> = vec![];
                let current_row = tab.row(idx);
                for c in colinfos.iter() {
                    match current_row.get(&c.name) {
                        Some(x) => match c.coltype.as_str() {
                            "int4" => sql_params.push(Box::new(x.parse::<i32>().unwrap())),
                            "timestamp" => sql_params.push(Box::new(
                                chrono::NaiveDateTime::parse_from_str(&x, DATE_TIME_FORMAT)
                                    .unwrap(),
                            )),
                            _ => sql_params.push(Box::new(x)),
                        },
                        None => sql_params.push(Box::new(None::<String>)),
                    };
                }
                let final_sql_params = sql_params
                    .iter()
                    .map(|x| x.as_ref() as &(dyn ToSql + Sync))
                    .collect::<Vec<_>>();

                let r = t
                    .execute(&stmt, &final_sql_params)
                    .await
                    .expect(&format!("query {:?}", final_sql_params));
            }
            t.commit().await.unwrap();
        } else {
            panic!("no transaction found");
        }
    }

    pub async fn activate(&mut self, s: Schema) {
        for e in s.entity_types.iter() {
            trace!("activate {}", e.name);
            let m = self.get_table_metadata(&e.name).await;
            if m.colspecs.len() == 0 {
                // create table
                let mut sql = vec![];
                sql.push(format!("CREATE TABLE public.{} (", e.name));
                if e.key.is_none() {
                    sql.push(format!("id serial,"));
                }
                for f in e.properties.iter() {
                    sql.push(format!("{} character varying(100),", f.name));
                }
                let key_part = match &e.key {
                    Some(k) => format!("primary key ({:})", k.properties.join(",")),
                    None => String::from("primary key (id)"),
                };
                sql.push(format!("constraint {}_pkey {key_part})", e.name));

                let client = &mut self.client;
                if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
                    let stmt = sql.join(" ");
                    debug!("statement: {stmt}");
                    t.execute(&stmt, &[]).await.unwrap();
                    t.commit().await.unwrap();
                }
            }
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
                    if let Ok(v) = row.try_get(idx) {
                        rrow.set(c.name(), v);
                    }
                }
                "text" => {
                    if let Ok(v) = row.try_get(idx) {
                        rrow.set(c.name(), v);
                    }
                }
                "int4" => {
                    if let Ok(v) = row.try_get::<'_, _, i32>(idx) {
                        rrow.set(c.name(), &v.to_string());
                    }
                }
                "int2" => {
                    if let Ok(v) = row.try_get::<'_, _, i16>(idx) {
                        rrow.set(c.name(), &v.to_string());
                    }
                }
                "bool" => {
                    if let Ok(v) = row.try_get::<'_, _, bool>(idx) {
                        rrow.set(c.name(), &v.to_string());
                    }
                }
                "name" => {
                    if let Ok(v) = row.try_get(idx) {
                        rrow.set(c.name(), v);
                    }
                }
                "oid" => {
                    if let Ok(v) = row.try_get::<'_, _, Oid>(idx) {
                        rrow.set(c.name(), &v.to_string());
                    }
                }
                "timestamp" => {
                    if let Ok(v) = row.try_get::<'_, _, chrono::NaiveDateTime>(idx) {
                        let s = v.format(DATE_TIME_FORMAT_OUT).to_string();
                        rrow.set(c.name(), &s);
                    }
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
