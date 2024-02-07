pub mod data;
pub mod error;

use crate::error::Error;
use data::model::FieldType::{Lookup, Number, Text};
use dbimpl::DatabaseImpl;
use rusqlite::{
    types::{ToSqlOutput, Value},
    Connection, Row, ToSql,
};
use serde::{de::DeserializeOwned, Serialize};
#[allow(dead_code)]
use std::fmt::Write;
use std::{
    fmt::Display,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
};
use log::*;
pub mod de;
pub mod ser;

use crate::data::model::{DataModel, Table};

use std::clone::Clone;
#[derive(Debug, Clone)]
pub struct DatabaseBuilder {}

#[derive(Debug, Clone, PartialEq)]
pub struct SqlValue(Value);

impl SqlValue {
    pub fn new<T: ToSql>(v: T) -> SqlValue {
        match v.to_sql() {
            Ok(ToSqlOutput::Owned(val)) => SqlValue(val.clone()),
            Ok(ToSqlOutput::Borrowed(val)) => SqlValue(val.into()),
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }

    fn to_sql(&self) -> ToSqlOutput<'_> {
        self.0.to_sql().unwrap()
    }
}

impl Display for SqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Value::Null => write!(f, "Null"),
            Value::Integer(v) => write!(f, "{}", v),
            Value::Real(v) => write!(f, "{}", v),
            Value::Text(v) => write!(f, "{}", v),
            Value::Blob(v) => write!(f, "U8[{}]", v.len()),
        }
    }
}

impl From<edm::value::Value> for SqlValue {
    fn from(value: edm::value::Value) -> Self {
        match value {
            edm::value::Value::PrimitiveValue(v) => match v {
                edm::primitive::PrimitiveValue::Null => SqlValue(Value::Null),
                edm::primitive::PrimitiveValue::Boolean(v) => SqlValue(Value::Integer(if v { 1 } else { 0 })),
                edm::primitive::PrimitiveValue::Decimal(_) => todo!(),
                edm::primitive::PrimitiveValue::String(v) => SqlValue(Value::Text(v)),
                edm::primitive::PrimitiveValue::Custom { datatype: _, value: _ } => todo!(),
            },
            edm::value::Value::StructureValue(_) => todo!(),
            edm::value::Value::ListValue(_) => todo!(),
        }
    }
}

impl ToSql for SqlValue {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

impl From<&str> for SqlValue {
    fn from(value: &str) -> Self {
        SqlValue(Value::Text(value.to_string()))
    }
}

impl From<&SqlValue> for String {
    fn from(SqlValue(value): &SqlValue) -> Self {
        match value {
            Value::Null => String::new(),
            Value::Integer(n) => format!("{}", n),
            Value::Real(n) => format!("{}", n),
            Value::Text(t) => t.clone(),
            Value::Blob(_) => String::from("<BLOB>"),
        }
        // if let Value::Text(s) = value.0 {
        //     s
        // } else {
        //     panic!("exctracting string value from a non-string. {:?}", value.0);
        // }
    }
}

impl From<Value> for SqlValue {
    fn from(value: Value) -> Self {
        SqlValue(value)
    }
}

impl From<bool> for SqlValue {
    fn from(value: bool) -> Self {
        SqlValue(Value::Integer(if value { 1 } else { 0 }))
    }
}

impl From<u64> for SqlValue {
    fn from(value: u64) -> Self {
        SqlValue(Value::Integer(value as i64))
    }
}

impl From<SqlValue> for u64 {
    fn from(value: SqlValue) -> Self {
        if let SqlValue(Value::Integer(n)) = value {
            n as u64
        } else {
            panic!("no integer Value")
        }
    }
}

#[derive(Debug, Clone)]
pub struct DBRow {
    table: Option<String>,
    values: Vec<(String, SqlValue)>,
}

#[allow(dead_code)]
impl DBRow {
    pub fn get(&self, k: &str) -> Option<&SqlValue> {
        if let (Some(tabname), Some(idx)) = (&self.table, k.find(".")) {
            let fld = &k[idx + 1..];
            let tab = &k[..idx];
            info!("find field {} {}", tab, fld);
            if tab == tabname {
                if let Some((_, val)) = self.values.iter().find(|(key, _)| key == fld) {
                    Some(val)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            if let Some((_, val)) = self.values.iter().find(|(key, _)| key == k) {
                Some(val)
            } else {
                None
            }
        }
    }

    pub fn new(name: &str) -> DBRow {
        DBRow {
            table: Some(name.into()),
            values: vec![],
        }
    }

    pub fn insert(&mut self, k: String, v: SqlValue) {
        let mut remove_idx = None;
        for idx in 0..self.values.len() {
            if self.values[idx].0 == k {
                remove_idx = Some(idx);
                break;
            }
        }
        if let Some(idx) = remove_idx {
            self.values.remove(idx);
        }
        self.values.push((k.clone(), v));
    }

    pub fn keys(&self) -> Vec<&str> {
        self.values.iter().map(|(k, _)| k.as_str()).collect()
    }

    pub fn exists(&self, k: &str) -> bool {
        self.values.iter().any(|(key, _)| key == k)
    }

    pub fn index(&self, k: &str) -> Option<usize> {
        self.values.iter().position(|(key, _)| key == k)
    }

    pub fn remove(&mut self, k: &str) {
        if let Some(pos) = self.index(k) {
            self.values.remove(pos);
        }
    }

    pub fn set(&mut self, k: &str, v: SqlValue) {
        self.remove(k);
        self.values.push((k.into(), v));
    }

    pub fn get_at(&self, idx: usize) -> &SqlValue {
        &self.values[idx].1
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn key_at(&self, idx: usize) -> &str {
        self.values[idx].0.as_str()
    }

    pub fn table(&self) -> &str {
        match &self.table {
            Some(t) => {
                if let Some(idx) = t.find(".") {
                    let tab = &t[idx + 1..];
                    // let tab = &t[..idx];
                    tab
                } else {
                    t.as_str()
                }
            }
            None => todo!(),
        }
    }

    fn create_from(table: String, row: &Row<'_>) -> DBRow {
        let mut r = DBRow::new(table.as_str());
        for field in row.as_ref().column_names() {
            let v: Value = row.get(field).unwrap();
            r.insert(field.into(), SqlValue(v));
        }
        r
    }
}

impl Display for DBRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.table {
            Some(tab_name) => write!(f, "{}(", tab_name),
            None => write!(f, "("),
        }?;

        let mut sep = "";
        for (name, SqlValue(v)) in self.values.iter() {
            write!(f, "{}{}=", sep, name)?;
            match v {
                Value::Null => write!(f, "Null"),
                Value::Integer(v) => write!(f, "{}", v),
                Value::Real(_) => todo!(),
                Value::Text(v) => write!(f, "{}", v),
                Value::Blob(_) => todo!(),
            }?;
            sep = ", ";
        }
        write!(f, ")")
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Database {
    arc: Arc<DatabaseGuarded>,
    model: Option<Rc<DataModel>>,
}

pub struct DatabaseGuarded {
    mutex: Mutex<DatabaseImpl>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    datatype: String,
    key: bool,
    null: bool,
    exists: bool,
    changed: bool,
    default: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct DBField {
    name: String,
    datatype: String,
    default: Option<String>,
    key: bool,
    has_null: bool,
}

#[derive(Debug)]
pub struct DBTable {
    name: String,
    fields: Vec<DBField>,
}

#[derive(Debug)]
pub struct DataDictionary {}

mod dbimpl;

impl DBTable {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: vec![],
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn load_table_meta(&mut self, con: &Connection) {
        trace!("load table_info for {}", self.name);
        let mut s = con
            .prepare(format!("pragma table_info({:})", self.name).as_str())
            .unwrap();
        let mut q = s.query(()).expect("ok");
        self.fields = vec![];
        while let Ok(Some(r)) = q.next() {
            let db_field = DBField {
                name: r.get(1).unwrap(),
                datatype: r.get(2).unwrap(),
                default: r.get(4).unwrap(),
                key: 0 < r.get(5).unwrap(),
                has_null: 0 == r.get::<usize, i64>(3).unwrap(),
            };
            self.fields.push(db_field);
        }
    }

    fn key(&self) -> Vec<&str> {
        self.fields
            .iter()
            .filter(|x| x.key)
            .map(|x| x.name.as_str())
            .collect()
    }

    fn field(&self, field_name: &str) -> Option<&DBField> {
        self.fields.iter().find(|x| x.name == field_name)
    }
}

impl Display for DBTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Table {}", self.name)?;
        for fld in self.fields.iter() {
            writeln!(
                f,
                "{:1} {:20} {:20} {:20}",
                if fld.key { "X" } else { "" },
                fld.name,
                fld.datatype,
                fld.has_null
            )?;
        }
        Ok(())
    }
}

impl Database {
    pub fn new() -> Self {
        Self {
            model: None,
            arc: Arc::new(DatabaseGuarded {
                mutex: Mutex::new(DatabaseImpl::new()),
            }),
        }
    }

    pub fn connect(&self, file: Option<&str>) {
        let con = match file {
            Some(path) => Connection::open(path).unwrap(),
            None => Connection::open_in_memory().unwrap(),
        };

        let mut l = self.locked();
        (*l).set_connection(con);
        l.load_meta();
    }

    pub fn is_connected(&self) -> bool {
        true
    }

    fn locked(&self) -> MutexGuard<'_, DatabaseImpl> {
        self.arc.mutex.lock().unwrap()
    }

    pub fn activate_structure(&self, model: DataModel) {
        let mut x = self.locked();
        x.activate_structure(model);
    }

    // pub fn new_structure(&self) -> Structure {
    //     Structure::new()
    // }

    pub fn modify_from(&self, table_name: &str, row: &DBRow) {
        let x = self.locked();
        x.modify_from_upd_first(table_name, row);
    }

    pub fn select<T: DeserializeOwned>(&self, q: crate::data::Query) -> Vec<T> {
        let x = self.locked();
        x.select(q)
    }

    pub fn modify_from_ser<T>(&self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        if let Some(model) = {
            let x = self.locked();
            (*x).model()
        } {
            debug!("serialize row");
            let x = ser::serialize_row(model, value)?;
            info!("serialization generated {} rows. ***************", x.len());
            for r in x {
                info!("write row: {}", r);
                self.modify_from(r.table(), &r);
            }
        }
        Ok(())
    }

    pub fn execute_query(&self, arg: &str) -> Vec<DBRow> {
        let x = self.locked();
        x.execute_query(arg)
    }

    pub fn tables(&self) -> Vec<String> {
        let x = self.locked();
        (*x).tables().map(|x| x.name.clone()).collect()
    }

    pub fn select_rows(&self, q: data::Query) -> Result<edm::value::Value, Error> {
        let x = self.locked();
        x.select_rows(q)
    }
}

fn build_alter_table(t: &DBTable, t0: &Table) -> Result<Vec<String>, std::fmt::Error> {
    let mut result = vec![];
    // write!(&mut sql, "ALTER TABLE {} ", t.name)?;
    for x in t0.fields() {
        if let Some(_db_field) = t.field(x.name.as_str()) {
            trace!("field {} unchanged", x.name);
        } else {
            match &x.fieldtype {
                Text(length) => {
                    info!("create text field {}", x.name);
                    let mut sql = String::new();
                    write!(&mut sql, "ALTER TABLE {} ", t.name)?;
                    write!(&mut sql, "ADD COLUMN {} varchar({length})", x.name)?;
                    result.push(sql);
                }
                Number => {
                    info!("create number field {}", x.name);
                    let mut sql = String::new();
                    write!(&mut sql, "ALTER TABLE {} ", t.name)?;
                    write!(&mut sql, "ADD COLUMN {} number", x.name)?;
                    result.push(sql);
                }
                Lookup {
                    table: _,
                    as_field: _,
                } => {
                    info!("create lookup field {}", x.name);
                    let mut sql = String::new();
                    write!(&mut sql, "ALTER TABLE {} ", t.name)?;
                    write!(&mut sql, "ADD COLUMN {} varchar", x.name)?;
                    result.push(sql);
                }
                data::model::FieldType::DependentList { .. } => {}
                data::model::FieldType::ReferenceList { .. } => {}
            }
        }
    }
    // info!("sql: {}", sql);
    Ok(result)
}

fn build_create_table(t: &Table) -> Result<String, std::fmt::Error> {
    let mut sql = String::new();
    write!(&mut sql, "CREATE TABLE {} (", t.name())?;
    for x in t.fields() {
        match &x.fieldtype {
            Text(length) => write!(&mut sql, "{} varchar({length}),", x.name)?,
            Number => write!(&mut sql, "{} number,", x.name)?,
            Lookup { .. } => write!(&mut sql, "{} varchar,", x.name)?,
            _ => info!("ignoring field {:?}", x.fieldtype),
        }
    }
    write!(&mut sql, "primary key (")?;
    let mut sep = "";
    for x in t.key() {
        write!(&mut sql, "{}{}", sep, x)?;
        sep = ",";
    }
    write!(&mut sql, ") );")?;
    Ok(sql)
}

impl DatabaseBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Database {
        Database {
            model: None,
            arc: Arc::new(DatabaseGuarded {
                mutex: Mutex::new(DatabaseImpl::new()),
            }),
        }
    }
}

impl DataDictionary {
    pub fn collect_tables(&mut self, con: &Connection) {
        let mut s = con
            .prepare(format!("select name from sqlite_schema where type = 'table'").as_str())
            .unwrap();
        let mut q = s.query(()).unwrap();
        while let Ok(Some(r)) = q.next() {
            let _table_name: String = r.get(0).unwrap();
        }
    }
}

#[allow(dead_code)]

fn create_update_statement_from<'a>(
    table_name: &str,
    key: &[&str],
    row: &'a DBRow,
) -> (String, Vec<ToSqlOutput<'a>>) {
    trace!(
        "generate sql update statment for table {} and key {:?}",
        table_name,
        key
    );

    let mut sql = String::new();
    write!(&mut sql, "UPDATE {} SET ", table_name).unwrap();
    let mut sep = "";
    let mut params = vec![];
    for (k, v) in row.values.iter() {
        let sqlv = v.to_sql();
        write!(&mut sql, "{}{} = ?", sep, k).unwrap();
        sep = ",";
        params.push(sqlv);
    }
    write!(&mut sql, " WHERE ").unwrap();
    sep = "";
    for k in key.iter() {
        if let Some(sqlv) = row.get(k) {
            write!(&mut sql, "{}{} = ?", sep, k).unwrap();
            sep = " AND ";
            params.push(sqlv.to_sql());
        } else {
            error!("dbrow: {:?}", row);
            panic!("key field '{}' not in row", k);
        }
    }
    (sql, params)
}

fn create_insert_statement_from<'a>(arg: &str, s: &'a DBRow) -> (String, Vec<ToSqlOutput<'a>>) {
    let mut sql = String::new();
    write!(&mut sql, "INSERT INTO {}(", arg).unwrap();
    let mut sep = "";
    for (k, _) in s.values.iter() {
        write!(&mut sql, "{}{}", sep, k).unwrap();
        sep = ",";
    }
    write!(&mut sql, ") VALUES (").unwrap();
    sep = "";
    let mut params = vec![];
    for (_k, v) in s.values.iter() {
        write!(&mut sql, "{}?", sep).unwrap();
        sep = ",";
        params.push(v.to_sql());
    }

    write!(&mut sql, ")").unwrap();
    // println!("insert: {}", sql);
    (sql, params)
}
