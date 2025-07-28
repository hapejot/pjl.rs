use edm::{primitive::PrimitiveValue, value::Value, Schema};
use indexmap::IndexMap;
use mini_moka::sync::Cache;
use pjl_odata::{ConditionValue, DbSpecifics};
use pjl_tab::Table;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::{collections::HashMap, time::Duration};
use tokio::time::timeout;
use tokio_postgres::{
    connect,
    types::{Oid, ToSql},
    Client, NoTls,
};
use tracing::*;
use types::SqlType;

pub mod types;

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
    keys: HashMap<String, Vec<String>>,
}

pub struct Database {
    client: Client,
    primary_keys: Cache<String, Vec<KeyPart>>,
    table_meta: Cache<String, TableMetadata>,
    mapping: pjl_tab::map::ValueMapping<String, types::SqlType>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlTable {
    name: String,
    fields: IndexMap<String, SqlType>,
    keys: HashMap<String, Vec<String>>,
}

impl SqlTable {
    pub fn name(&self) -> &str {
        &self.name
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

    fn add_cond(&mut self, op: &str, value: &ConditionValue) {
        match op {
            "=" => {
                self.params.push(value.value());
                self.arm.push(format!(
                    "{} = ${}",
                    self.fld.as_ref().unwrap(),
                    self.params.len()
                ));
            }
            "ne" => {
                self.params.push(value.value());
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

#[allow(dead_code)]
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
                tokio::spawn(async move {
                    if let Err(e) = conn.await {
                        error!("connection error: {}", e);
                    }
                });
                Ok(Self {
                    client: client,
                    primary_keys: Cache::new(10),
                    table_meta: Cache::new(10),
                    mapping: types::postgres_to_standard_sql_type_mapping(),
                })
            }
            Err(e) => Err(format!("{e}")),
        }
    }

    pub fn connected(&self) -> bool {
        true
    }

    async fn read_table_metadata(&mut self, tab_name: &str) -> Result<(), String> {
        let client = &mut self.client;
        let _r = Table::new();

        if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
            let query = format!(
                r#"select   a.attname, 
                            atttypid,
                            attnum, 
                            attnotnull, 
                            t.typname, 
                            t.typlen, 
                            a.atttypmod,
                            format_type(atttypid, atttypmod) AS column_type
                            from pg_attribute as a
                            join pg_type as t on a.atttypid = t.oid                            
                            where  a.attrelid = to_regclass($1)
                              and a.attnum > 0 order by attnum"#
            );
            let stmt = t.prepare(&query).await.map_err(|e| e.to_string())?;
            let rows = t
                .query(&stmt, &[&*Box::new(tab_name)])
                .await
                .map_err(|e| e.to_string())?;
            let r = Table::new();
            extract_result_to_table(&r, rows);
            let colspecs: Vec<Decl> = pjl_tab::de::extract_from_table(&r).map_err(|e| e.to_string())?;
            debug!("colspecs: {colspecs:#?}");

            let query = format!(
                r#"select indexrelid::regclass::text, indkey from pg_index as i 
                        where  i.indrelid = to_regclass($1)"#
            );
            let stmt = t.prepare(&query).await.map_err(|e| e.to_string())?;
            let mut r_keys = HashMap::new();
            let rows = t
                .query(&stmt, &[&*Box::new(tab_name)])
                .await
                .map_err(|e| e.to_string())?;
            let mut out = String::new();
            r.dump(&mut out);
            debug!("{out}");
            for x in rows {
                let index_name = x.get::<'_, _, String>(0);
                let index_fields = x.get::<'_, _, Vec<i16>>(1);
                let mut index_field_names = vec![];

                for i in index_fields {
                    let attr = r.row(i as usize);
                    let n = attr.get("attname").unwrap();
                    index_field_names.push(n);
                }
                r_keys.insert(index_name, index_field_names);
            }

            self.table_meta.insert(
                tab_name.to_string(),
                TableMetadata {
                    colspecs,
                    keys: r_keys,
                },
            );
            Ok(())
        } else {
            Err("Failed to start transaction or timeout".to_string())
        }
    }

    async fn get_table_metadata(&mut self, tab_name: &str) -> Result<TableMetadata, String> {
        let tab_name = tab_name.to_string();
        if !self.table_meta.contains_key(&tab_name) {
            self.read_table_metadata(&tab_name).await?;
        }
        self.table_meta.get(&tab_name).map(|v| v.clone()).ok_or_else(|| "Table metadata not found".to_string())
    }

    pub async fn read_primary_key(&mut self, tab_name: &str) -> Result<Vec<KeyPart>, String> {
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
                let stmt = t.prepare(&query).await.map_err(|e| e.to_string())?;
                let rows = t
                    .query(&stmt, &[&*Box::new(tab_name.clone())])
                    .await
                    .map_err(|e| e.to_string())?;
                extract_result_to_table(&r, rows);
                self.primary_keys.insert(
                    tab_name.clone(),
                    pjl_tab::de::extract_from_table(&r).map_err(|e| e.to_string())?,
                );
            } else {
                return Err("Failed to start transaction or timeout".to_string());
            }
        }
        self.primary_keys.get(&tab_name).map(|v| v.clone()).ok_or_else(|| "Primary key not found".to_string())
    }

    pub async fn select(&mut self, q: pjl_odata::ODataQuery) -> Result<Table, String> {
        debug!("query: {:#?}", q);
        let (where_clause, mut params) = q.get_where_sql_specific(PostgresQuery::new());
        let mut sql_parts = vec![];
        sql_parts.push(format!("SELECT * FROM {}", q.get_table()));
        if where_clause.len() > 0 {
            sql_parts.push(format!("WHERE {}", where_clause));
        }
        if let Some(orderby) = q.orderby() {
            sql_parts.push(format!("ORDER BY {}", orderby));
        }
        let sql = sql_parts.join(" ");
        trace!("SQL: {sql}");
        let client = &mut self.client;
        let r = Table::new();

        if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
            let statement = t.prepare(&sql).await.map_err(|e| e.to_string())?;
            let mut sql_params: Vec<Box<dyn ToSql + Sync + Send>> = vec![];
            for (idx, typ) in statement.params().iter().enumerate() {
                let x = params.get_mut(idx).unwrap();
                match x {
                    Value::PrimitiveValue(primitive_value) => match primitive_value {
                        PrimitiveValue::Null => return Err("Null value not supported".to_string()),
                        PrimitiveValue::Boolean(b) => match typ.name() {
                            "int4" => sql_params.push(Box::new(if *b { 1 } else { 0 })),
                            "bool" => sql_params.push(Box::new(*b)),
                            _ => sql_params.push(Box::new(b.to_string())),
                        },
                        PrimitiveValue::Decimal(number) => match typ.name() {
                            "int4" => sql_params.push(Box::new((number.as_i64()) as i32)),
                            _ => sql_params.push(Box::new(number.as_str().to_string())),
                        },
                        PrimitiveValue::String(v) => match typ.name() {
                            "int4" => sql_params.push(Box::new(v.parse::<i32>().map_err(|e| e.to_string())?)),
                            _ => sql_params.push(Box::new(v.clone())),
                        },
                        PrimitiveValue::Custom { .. } => return Err("Custom primitive not supported".to_string()),
                    },
                    Value::StructureValue(_) => return Err("StructureValue not supported".to_string()),
                    Value::ListValue(_) => return Err("ListValue not supported".to_string()),
                }
            }
            assert_eq!(params.len(), sql_params.len());
            trace!("sql_params: {sql_params:?}");
            let final_sql_params = sql_params
                .iter()
                .map(|x| x.as_ref() as &(dyn ToSql + Sync))
                .collect::<Vec<_>>();

            let rows = t.query(&statement, &final_sql_params).await.map_err(|e| e.to_string())?;
            extract_result_to_table(&r, rows);
            Ok(r)
        } else {
            Err("timout or other error...".to_string())
        }
    }

    pub async fn modify(&mut self, tab_name: &str, tab: Table) -> Result<(), String> {
        let meta = self.get_table_metadata(tab_name).await?;

        let mut colinfos = vec![];
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

        let ins_field_names = colinfos
            .iter()
            .map(|ci| ci.name.clone())
            .collect::<Vec<_>>();
        let ins_fields = &ins_field_names.join(", ");
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
        let mut pk = String::new();
        trace!("ins_field_names: {:?}", ins_field_names);
        for (_, x) in meta.keys.iter() {
            trace!("checking key {:?}", x);
            if is_subset(x, &ins_field_names) {
                pk = x.join(",");
                trace!("found key");
                break;
            }
        }

        let query = format!(
            "INSERT INTO {} ({})  VALUES ({}) ON CONFLICT ({pk}) DO UPDATE SET {upd}",
            tab_name, ins_fields, ins_values
        );
        trace!("{query}");
        let client = &mut self.client;
        let _r = Table::new();
        if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
            let stmt = t.prepare(&query).await.map_err(|e| e.to_string())?;
            for idx in 1..=tab.lines() {
                let mut sql_params: Vec<Box<dyn ToSql + Sync + Send>> = vec![];
                let current_row = tab.row(idx);
                for c in colinfos.iter() {
                    match current_row.get(&c.name) {
                        Some(x) => match c.coltype.as_str() {
                            "int4" => sql_params.push(Box::new(x.parse::<i32>().map_err(|e| e.to_string())?)),
                            "bool" => sql_params.push(Box::new(x.parse::<bool>().map_err(|e| e.to_string())?)),
                            "timestamp" => sql_params.push(Box::new(
                                chrono::NaiveDateTime::parse_from_str(&x, DATE_TIME_FORMAT)
                                    .map_err(|e| e.to_string())?,
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

                t.execute(&stmt, &final_sql_params)
                    .await
                    .map_err(|e| format!("query {:?}: {}", final_sql_params, e))?;
            }
            t.commit().await.map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("no transaction found".to_string())
        }
    }

    pub async fn activate(&mut self, s: Schema) -> Result<(), String> {
        for e in s.entity_types.iter() {
            trace!("activate {}", e.name);
            let m = self.get_table_metadata(&e.name).await?;
            if m.colspecs.len() == 0 {
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
                    t.execute(&stmt, &[]).await.map_err(|e| e.to_string())?;
                    t.commit().await.map_err(|e| e.to_string())?;
                } else {
                    return Err("Failed to start transaction or timeout".to_string());
                }
            }
        }
        Ok(())
    }

    pub async fn describe(&mut self, tab_name: &str) -> Result<SqlTable, String> {
        let meta = self.get_table_metadata(tab_name).await?;
        let mut r = SqlTable {
            name: tab_name.to_string(),
            fields: IndexMap::new(),
            keys: HashMap::new(),
        };

        for c in meta.colspecs.iter() {
            let type_str = c.column_type.trim();
            let t = self.to_generic_sql_type(type_str);
            r.fields.insert(c.attname.clone(), t);
        }

        for (name, fields) in meta.keys {
            r.keys.insert(name, fields);
        }
        Ok(r)
    }

    pub async fn define(&mut self, def: &SqlTable) -> Result<(), String> {
        let m = self.get_table_metadata(&def.name).await?;
        if m.colspecs.len() == 0 {
            let mut sql = vec![];
            sql.push(format!("CREATE TABLE public.{} (", def.name));
            sql.push(format!("id serial,"));
            for (f, t) in def.fields.iter() {
                if f != "id" {
                    sql.push(format!("{} {},", f, self.to_pg_type(t)));
                }
            }
            let key_part = String::from("primary key (id)");
            sql.push(format!("constraint {}_pkey {key_part})", def.name));

            let client = &mut self.client;
            if let Ok(Ok(t)) = timeout(Duration::from_secs(2), client.transaction()).await {
                let stmt = sql.join(" ");
                debug!("statement: {stmt}");
                t.execute(&stmt, &[]).await.map_err(|e| e.to_string())?;
                t.commit().await.map_err(|e| e.to_string())?;
            } else {
                return Err("Failed to start transaction or timeout".to_string());
            }
        }
        Ok(())
    }

    fn to_generic_sql_type(&mut self, type_str: &str) -> SqlType {
        let re = Regex::new(r"^([a-zA-Z\s]+)\s*(?:\(\s*(\d+)\s*\))?$").unwrap();
        let rs = if let Some(caps) = re.captures(type_str) {
            let name_str = caps.get(1).unwrap().as_str().to_string();
            if let Some(size) = caps.get(2).map(|m| m.as_str().parse::<usize>().unwrap()) {
                format!(
                    "{}({})",
                    self.mapping.map(&name_str).unwrap().to_string(),
                    size
                )
            } else {
                let type_str = type_str.to_string();
                self.mapping.map(&type_str).unwrap().to_string()
            }
        } else {
            String::from("unknwon")
        };
        SqlType::parse(&rs)
    }

    fn to_pg_type(&mut self, t: &SqlType) -> String {
        match t {
            SqlType::Varchar(n) => format!(
                "{}({})",
                self.mapping.rmap(&SqlType::Varchar(None)).unwrap(),
                n.unwrap()
            ),
            SqlType::Char(n) => format!(
                "{}({})",
                self.mapping.rmap(&SqlType::Char(None)).unwrap(),
                n.unwrap()
            ),
            x => self.mapping.rmap(x).unwrap(),
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

#[allow(dead_code)]
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

fn is_subset(vec1: &[String], vec2: &[String]) -> bool {
    let set2: HashSet<_> = vec2.iter().cloned().collect();
    vec1.iter().all(|item| set2.contains(item))
}
