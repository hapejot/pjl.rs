use std::{rc::Rc, slice::Iter};

use edm::structure::StructureValue;
use edm::{list::ListValue, primitive::PrimitiveValue};
use log::{debug, error, info, trace};
use rusqlite::{ffi::Error, types::ValueRef, ErrorCode};

use rusqlite::types::Value as SQLiteValue;

use crate::{
    build_alter_table, build_create_table, create_insert_statement_from,
    create_update_statement_from,
    data::{
        model::{
            DataModel, DependentListItem,
            FieldType::{DependentList, Lookup, ReferenceList},
        },
        Query, WhereCondition,
    },
    de, DBTable, SqlValue,
};

pub struct DatabaseImpl {
    con: Option<crate::Connection>,
    tables: Vec<crate::DBTable>,
    model: Option<Rc<crate::DataModel>>,
}

impl DatabaseImpl {
    /// if the primary key is satisfied first try to insert
    /// if this returns an error, try to update.
    ///
    /// if the primary key is not satisfied just use an insert since an update
    /// could update more than one row.
    ///
    #[allow(dead_code)]
    pub fn modify_from(&self, table_name: &str, row: &edm::value::Value) {
        if let edm::value::Value::StructureValue(row) = row {
            let (sql_ins, params) = create_insert_statement_from(&table_name, &row);
            if let Some(con) = &self.con {
                let mut stmt = con.prepare(sql_ins.as_str()).unwrap();
                match stmt.execute(rusqlite::params_from_iter(params)) {
                    Ok(_) => {}
                    Err(rusqlite::Error::SqliteFailure(
                        Error {
                            code: ErrorCode::ConstraintViolation,
                            extended_code: 1555,
                        },
                        _,
                    )) => {
                        debug!("PK already exists");
                        let (sql_upd, params) = create_update_statement_from(table_name, &[], &row);
                        let mut stmt = con.prepare(sql_upd.as_str()).unwrap();

                        match stmt.execute(rusqlite::params_from_iter(params)) {
                            Ok(x) => {
                                assert_eq!(x, 1);
                            }
                            Err(_) => todo!(),
                        }
                    }
                    _ => panic!(),
                }
            }
        }
    }

    pub fn modify_from_upd_first(&self, table_name: &str, row: &edm::structure::StructureValue) {
        if let (Some(con), Some(tab)) = (&self.con, self.table(table_name)) {
            let key = tab.key();
            trace!("{:}", tab);
            assert!(key.len() > 0, "no keys found in {:?}", tab);
            let (sql_upd, params) = create_update_statement_from(table_name, &key, &row);
            trace!("sql upd: {}", sql_upd);
            let mut stmt = match con.prepare(sql_upd.as_str()) {
                Ok(stmt) => stmt,

                Err(x) => {
                    error!("dbrow: {:?}", row);
                    panic!("prepare: [{}] -> {}", sql_upd, x);
                }
            };
            match stmt.execute(rusqlite::params_from_iter(params)) {
                Ok(1) => {}
                Ok(x) => {
                    trace!("update {} rows.", x);
                    let (sql_ins, params) = create_insert_statement_from(&table_name, &row);

                    let mut stmt = con.prepare(sql_ins.as_str()).unwrap();
                    match stmt.execute(rusqlite::params_from_iter(params)) {
                        Ok(_) => {}
                        _ => panic!(),
                    }
                }
                Err(_) => todo!(),
            }
        } else {
            if let None = &self.con {
                panic!("connection not usable");
            }
            if let None = &self.table(table_name) {
                panic!("table not available: {}", table_name);
            }
        }
    }

    pub fn select<T>(&self, q: crate::data::Query) -> Vec<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut result: Vec<T> = vec![];
        if let Some(con) = &self.con {
            let mut stmt = con.prepare(q.get_sql().as_str()).unwrap();
            let sql_result = stmt.query(rusqlite::params_from_iter(q.get_params()));
            match sql_result {
                Ok(mut rows) => {
                    while let Some(row) = rows.next().unwrap() {
                        result.push(
                            de::from_row(row)
                                .expect(format!("could not convert row {:?}", row).as_str()),
                        );
                        // for c in 0..&stmt.column_count() {}
                    }
                }
                Err(x) => {
                    error!("SELECT ERROR {:#?}", x);
                }
            }
        };
        result
    }

    pub fn select_rows(
        &self,
        q: crate::data::Query,
    ) -> Result<edm::value::Value, crate::error::Error> {
        debug!("select rows");
        let mut result = edm::list::ListValue::new();
        if let Some(con) = &self.con {
            let mut stmt = con.prepare(q.get_sql().as_str()).unwrap();
            debug!("params: {:?}", q.get_params());
            let sql_result = stmt.query(rusqlite::params_from_iter(q.get_params()));
            debug!("sql result: {}", sql_result.is_ok());
            match sql_result {
                Ok(mut rows) => {
                    while let Some(row) = rows.next().unwrap() {
                        trace!("copy row from result to edm structure");
                        result.push(structure_from(q.table(), row));
                    }
                    let model = self.model().unwrap();
                    let tab = model.find_table(q.table().as_str()).unwrap();
                    debug!("table definition: {:#?}", tab);
                    let key_field = tab.key().last().unwrap();
                    for rowv in result.iter_mut() {
                        if let edm::value::Value::StructureValue(rrow) = rowv {
                            let key_value = rrow[key_field].clone();
                            for f in tab.fields() {
                                debug!("processing field {}", f.name());
                                if !q.contains(f.name()) {
                                    match &f.fieldtype {
                                        Lookup { table, as_field } => {
                                            let _ = (table, as_field);
                                            todo!()
                                        }
                                        DependentList(items) => {
                                            let mut subrows = ListValue::new();
                                            debug!("list items: {}", items.len());
                                            for DependentListItem { table, on_field } in items {
                                                debug!("loading sub rows for table {}", table);
                                                let cond = WhereCondition::new().and(
                                                    crate::data::WhereExpr::Equals(
                                                        on_field.into(),
                                                        key_value.clone().into(),
                                                    ),
                                                );
                                                let sub_q = Query::new(table, vec!["*"], cond);
                                                if let edm::value::Value::ListValue(sub_rows) =
                                                    self.select_rows(sub_q)?
                                                {
                                                    debug!("sub rows received: {}", sub_rows.len());
                                                    // rrow.insert(f.name, Sql)
                                                    for x in sub_rows.iter() {
                                                        subrows.push(x.clone());
                                                    }
                                                }
                                            }
                                            debug!("total sub rows {}", subrows.len());
                                            rrow[f.name.as_str()] = subrows.into();
                                        }
                                        ReferenceList { table, via_table } => {
                                            let _ = (table, via_table);
                                            todo!()
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
                Err(x) => {
                    error!("SELECT ERROR {:#?}", x);
                }
            }
            Ok(result.into())
        } else {
            Err(crate::error::Error::Message("not connected.".into()))
        }
    }

    pub fn execute_query(&self, arg: &str) -> Vec<edm::structure::StructureValue> {
        let mut result = vec![];
        if let Some(con) = &self.con {
            let mut stmt_m = con.prepare(arg).unwrap();
            let sql_result = stmt_m.query([]);
            // let n = sql_result.column_count();
            match sql_result {
                Ok(mut rows) => {
                    let names = rows
                        .as_ref()
                        .unwrap()
                        .column_names()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>();
                    while let Some(row) = rows.next().unwrap() {
                        let mut res_row = StructureValue::new();
                        for idx in 0..names.len() {
                            let v = SQLiteValue::from(row.get::<_, SQLiteValue>(idx).unwrap());
                            res_row[names[idx].as_str()] = edm::value::Value::from_sql(v);
                        }
                        result.push(res_row);
                    }
                }
                Err(x) => {
                    error!("SELECT ERROR {:#?}", x);
                }
            }
        };
        result
    }

    pub fn collect_tables(&mut self) {
        if let Some(con) = &self.con {
            let mut s = con
                .prepare(format!("select name from sqlite_schema where type = 'table'").as_str())
                .unwrap();
            let mut q = s.query(()).unwrap();

            while let Ok(Some(r)) = q.next() {
                let table_name: String = r.get::<_, String>(0).unwrap();
                match self.table_find(table_name.as_str()) {
                    Some(_t) => {}
                    None => self.tables.push(DBTable::new(table_name.as_str())),
                }
            }
        }
    }

    pub fn load_meta(&mut self) {
        info!("loading metadata");
        self.collect_tables();
        if let Some(con) = &self.con {
            for t in self.tables.iter_mut() {
                t.load_table_meta(con);
            }
        }
    }

    // #[instrument(skip(self, model))]
    pub fn activate_structure(&mut self, model: DataModel) {
        info!("activate structure");
        self.model = Some(Rc::new(model));
        if let Some(con) = &self.con {
            if let Some(m) = &self.model {
                for t in m.tables() {
                    info!("activate table {}", t.name());
                    let dbtab = self.table(t.name());
                    match dbtab {
                        Some(dbtable) => {
                            debug!("{} already on db", t.name());
                            let srcs = build_alter_table(dbtable, t).unwrap();
                            for src in srcs {
                                con.execute(src.as_str(), []).unwrap();
                            }
                        }
                        None => {
                            debug!("{} new table", t.name());
                            let src = build_create_table(t).unwrap();
                            info!("{}", src);
                            con.execute(src.as_str(), []).unwrap();
                        }
                    }
                }
            }
        }
        self.load_meta();
        info!("activation finished");
    }

    fn table(&self, table_name: &str) -> Option<&DBTable> {
        self.tables.iter().find(|x| x.name == table_name)
    }

    pub fn new() -> DatabaseImpl {
        DatabaseImpl {
            con: None,
            tables: vec![],
            model: None,
        }
    }

    pub fn model(&self) -> Option<Rc<DataModel>> {
        self.model.clone()
    }

    pub(crate) fn set_connection(&mut self, con: rusqlite::Connection) {
        self.con = Some(con);
    }

    pub(crate) fn tables(&self) -> Iter<'_, DBTable> {
        self.tables.iter()
    }

    fn table_find(&self, as_str: &str) -> Option<&DBTable> {
        self.tables().find(|x| x.name() == as_str)
    }
}

fn structure_from(table: String, row: &rusqlite::Row<'_>) -> edm::value::Value {
    let mut res = edm::structure::StructureValue::new_with_type(table.as_str());
    for col in row.as_ref().columns().iter() {
        match row.get_ref(col.name()) {
            Ok(v) => {
                let vv = match v {
                    ValueRef::Null => edm::value::Value::PrimitiveValue(PrimitiveValue::Null),
                    ValueRef::Integer(v) => v.into(),
                    ValueRef::Real(_) => todo!(),
                    ValueRef::Text(v) => String::from_utf8(v.into()).unwrap().as_str().into(),
                    ValueRef::Blob(_) => todo!(),
                };
                res[col.name()] = vv;
            } //res[col.name()] = v.into(),
            Err(e) => error!(
                "could not extract string from field {} type was: {:?} {:?}",
                col.name(),
                col.decl_type(),
                e
            ),
        }
    }
    res.into()
}

pub trait IntoValue {
    fn from_sql(value: SQLiteValue) -> Self;
}

impl IntoValue for edm::value::Value {
    fn from_sql(value: SQLiteValue) -> Self {
        match value {
            SQLiteValue::Null => edm::value::Value::PrimitiveValue(PrimitiveValue::Null),
            SQLiteValue::Integer(v) => {
                edm::value::Value::PrimitiveValue(PrimitiveValue::Decimal(v.into()))
            }
            SQLiteValue::Real(v) => {
                edm::value::Value::PrimitiveValue(PrimitiveValue::Decimal(v.into()))
            }
            SQLiteValue::Text(v) => edm::value::Value::PrimitiveValue(PrimitiveValue::String(v)),
            SQLiteValue::Blob(_) => todo!(),
        }
    }
}
