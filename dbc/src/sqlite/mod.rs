use crate::model::*;
use std::fmt::Write;
use rusqlite::{params, Connection};



pub fn collect_tables(con: &Connection) -> Result<Schema, String> {
    let mut s = sql_check(con.prepare(&format!(
        "select name from sqlite_schema where type = 'table'"
    )))?;
    let mut q = sql_check(s.query(()))?;
    let mut schema = Schema::new();
    while let Ok(Some(r)) = q.next() {
        let table_name: String = r.get::<_, String>(0).unwrap();
        let t = load_table_meta(con, &table_name)?;
        schema.add(t);
    }
    Ok(schema)
}


pub fn load_table_meta(con: &Connection, name: &str) -> Result<Table, String> {
    let mut s = sql_check(con.prepare(format!("pragma table_info({:})", name).as_str()))?;
    let mut q = sql_check(s.query(()))?;
    let mut t = Table::new(name);
    while let Ok(Some(r)) = q.next() {
        let name: String = sql_check(r.get(1))?;
        let ftype: String = sql_check(r.get(2))?;
        t.add(Field::new(&name, &ftype));
    }
    Ok(t)
}

pub fn build_create_table(t: &Table) -> Result<String, std::fmt::Error> {
    let mut sql = String::new();
    write!(&mut sql, "CREATE TABLE {} (", t.name())?;
    for x in t.fields() {
        write!(&mut sql, "{} {}", x.name(), x.ftype())?;
        if !x.is_nullable() {
            write!(&mut sql, " not null")?;
        }
        write!(&mut sql, ",")?;
    }
    write!(&mut sql, "primary key (")?;
    let mut sep = "";
    for x in t.fields() {
        if x.is_key() {
            write!(&mut sql, "{}{}", sep, x.name())?;
            sep = ",";
        }
    }
    write!(&mut sql, ") );")?;
    Ok(sql)
}

pub fn build_add_field(tname: &str, f: &Field) -> Result<String, String> {
    let mut sql = String::new();
    write!(
        &mut sql,
        "alter table {tname} add {} {}",
        f.name(),
        f.ftype()
    )
    .unwrap();
    if !f.is_nullable() {
        write!(&mut sql, " not null").unwrap();
    }
    Ok(sql)
}

pub fn sql_check<R>(r: Result<R, rusqlite::Error>) -> Result<R, String> {
    match r {
        Ok(x) => Ok(x),
        Err(e) => match e {
            rusqlite::Error::SqliteFailure(error, _) => todo!(),
            rusqlite::Error::SqliteSingleThreadedMode => todo!(),
            rusqlite::Error::FromSqlConversionFailure(_, _, error) => todo!(),
            rusqlite::Error::IntegralValueOutOfRange(_, _) => todo!(),
            rusqlite::Error::Utf8Error(utf8_error) => todo!(),
            rusqlite::Error::NulError(nul_error) => todo!(),
            rusqlite::Error::InvalidParameterName(_) => todo!(),
            rusqlite::Error::InvalidPath(path_buf) => todo!(),
            rusqlite::Error::ExecuteReturnedResults => todo!(),
            rusqlite::Error::QueryReturnedNoRows => todo!(),
            rusqlite::Error::InvalidColumnIndex(_) => todo!(),
            rusqlite::Error::InvalidColumnName(_) => todo!(),
            rusqlite::Error::InvalidColumnType(_, _, _) => todo!(),
            rusqlite::Error::StatementChangedRows(_) => todo!(),
            rusqlite::Error::InvalidFunctionParameterType(_, _) => todo!(),
            rusqlite::Error::InvalidFilterParameterType(_, _) => todo!(),
            rusqlite::Error::UserFunctionError(error) => todo!(),
            rusqlite::Error::ToSqlConversionFailure(error) => todo!(),
            rusqlite::Error::InvalidQuery => todo!(),
            rusqlite::Error::ModuleError(_) => todo!(),
            rusqlite::Error::UnwindingPanic => todo!(),
            rusqlite::Error::GetAuxWrongType => todo!(),
            rusqlite::Error::MultipleStatement => todo!(),
            rusqlite::Error::InvalidParameterCount(_, _) => todo!(),
            rusqlite::Error::BlobSizeError => todo!(),
            rusqlite::Error::SqlInputError {
                error,
                msg,
                sql,
                offset,
            } => {
                let mut o = String::new();
                writeln!(&mut o, "error: {error}").unwrap();
                writeln!(&mut o, "msg: {msg}").unwrap();
                writeln!(&mut o, "sql: {sql}").unwrap();
                writeln!(&mut o, "offset: {offset}").unwrap();
                Err(o)
            }
            rusqlite::Error::InvalidDatabaseIndex(_) => todo!(),
            _ => todo!(),
        },
    }
}
