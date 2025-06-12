use crate::model::*;
use bytes::BytesMut;
use edm::{structure::StructureValue, value::Value};
use postgres::{types::ToSql, Client};
use std::{fmt::Write, string};

pub fn sql_check<X>(r: Result<X, postgres::Error>) -> Result<X, String> {
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

pub fn collect_tables(con: &mut Client) -> Result<Schema, String> {
    let s = sql_check(con.prepare(
        "SELECT table_name FROM information_schema.tables where table_schema = 'public'",
    ))?;
    let mut schema = Schema::new();
    for row in sql_check(con.query(&s, &[]))? {
        let table_name = row.get(0);
        let t = load_table_meta(con, table_name)?;
        schema.add(t);
    }
    Ok(schema)
}

pub fn load_table_meta(con: &mut Client, name: &str) -> Result<Table, String> {
    let mut t = Table::new(name);
    for f in sql_check(con.query("select column_name, data_type from information_schema.columns where table_schema = $1 and table_name = $2", &[&"public", &name]))? {
        let name:&str = f.get("column_name");
        let ftype:&str = f.get("data_type");
        t.add(Field::new(&name, &ftype));
    }

    Ok(t)
}
fn rewrite(s: &str) -> &str {
    match s {
        "blob" => "bytea",
        _ => s,
    }
}
pub fn build_create_table(t: &Table) -> Result<String, std::fmt::Error> {
    let mut sql = String::new();
    write!(&mut sql, "CREATE TABLE {} (", t.name())?;
    for x in t.fields() {
        write!(&mut sql, "{} {}", x.name(), rewrite(x.ftype()))?;
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
#[derive(Debug)]
struct PGValue(Value);
impl ToSql for PGValue {
    fn to_sql(
        &self,
        ty: &postgres::types::Type,
        out: &mut BytesMut,
    ) -> Result<postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        todo!()
    }

    fn accepts(ty: &postgres::types::Type) -> bool
    where
        Self: Sized,
    {
        todo!()
    }

    fn to_sql_checked(
        &self,
        ty: &postgres::types::Type,
        out: &mut BytesMut,
    ) -> Result<postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match &self.0 {
            Value::PrimitiveValue(primitive_value) => match primitive_value {
                edm::primitive::PrimitiveValue::Null => None::<String>.to_sql_checked(ty, out),
                edm::primitive::PrimitiveValue::Boolean(v) => v.to_sql_checked(ty, out),
                edm::primitive::PrimitiveValue::Decimal(number) => {
                    let oid = ty.oid();
                    match oid {
                        20 => number.as_i64().to_sql_checked(ty, out),
                        21 => {
                            let n = number.as_i64() as i16;
                            n.to_sql_checked(ty, out)
                        }
                        23 => {
                            let n = number.as_i64() as i32;
                            n.to_sql_checked(ty, out)
                        }
                        700 => {
                            let n = number.as_f64() as f32;
                            n.to_sql_checked(ty, out)
                        }
                        701 => number.as_f64().to_sql_checked(ty, out),
                        1700 => number.as_str().to_sql_checked(ty, out),
                        _ => todo!(),
                    }
                }
                edm::primitive::PrimitiveValue::String(v) => v.to_sql_checked(ty, out),
                edm::primitive::PrimitiveValue::Custom { datatype, value } => todo!(),
            },
            Value::StructureValue(structure_value) => todo!(),
            Value::ListValue(list_value) => todo!(),
        }
    }
}

pub fn insert(client: &mut Client, schema: &Schema, row: &StructureValue) -> Result<(), String> {
    match schema.get_table(row.datatype()) {
        Some(t) => {
            let mut values: Vec<PGValue> = vec![];
            let mut sql = String::new();
            let mut sep = String::new();
            write!(&mut sql, "INSERT INTO {} (", t.name()).unwrap();
            for f in t.fields() {
                if row.has_field(f.name()) {
                    write!(&mut sql, "{}{}", sep, f.name()).unwrap();
                    sep = String::from(", ");
                    values.push(PGValue(row[f.name()].clone()));
                }
            }
            write!(&mut sql, ") VALUES (").unwrap();
            sep = String::new();
            for idx in 1..=values.len() {
                write!(&mut sql, "{}${}", sep, idx).unwrap();
                sep = String::from(",");
            }
            write!(&mut sql, ")").unwrap();
            let params: Vec<&(dyn ToSql + Sync)> =
                values.iter().map(|x| x as &(dyn ToSql + Sync)).collect();
            println!("SQL: {}", sql);
            let q = sql_check(client.execute(&sql, &params))?;
            if q == 0 {
                Err("There was now row to update.")?;
            } else if q > 1 {
                Err("Multiple rows have been updated")?;
            }
            Ok(())
        }
        _ => Err(format!("table {} is not defined", row.datatype())),
    }
}

pub fn update(client: &mut Client, schema: &Schema, row: &StructureValue) -> Result<(), String> {
    match schema.get_table(row.datatype()) {
        Some(t) => {
            let mut values: Vec<PGValue> = vec![];
            let mut sql = String::new();
            let mut sep = String::new();
            write!(&mut sql, "UPDATE {} SET ", t.name()).unwrap();
            for f in t.fields() {
                if !f.is_key() {
                    let idx = values.len() + 1;
                    if row.has_field(f.name()) {
                        write!(&mut sql, "{}{} = ${}", sep, f.name(), idx).unwrap();
                        let v = PGValue(row[f.name()].clone());
                        values.push(v);
                        sep = String::from(", ");
                    }
                }
            }
            sep = String::new();
            write!(&mut sql, " WHERE ").unwrap();
            for f in t.fields() {
                if f.is_key() {
                    let idx = values.len() + 1;
                    if row.has_field(f.name()) {
                        write!(&mut sql, "{}{} = ${}", sep, f.name(), idx).unwrap();
                        let v = PGValue(row[f.name()].clone());
                        values.push(v);
                        sep = String::from(" AND ");
                    } else {
                        Err(format!(
                            "not all keys are provided for an update: {} was missing.",
                            f.name()
                        ))?;
                    }
                }
            }
            let params: Vec<&(dyn ToSql + Sync)> =
                values.iter().map(|x| x as &(dyn ToSql + Sync)).collect();
            println!("SQL: {}", sql);
            let q = sql_check(client.execute(&sql, &params))?;
            if q == 0 {
                Err("There was now row to update.")?;
            } else if q > 1 {
                Err("Multiple rows have been updated")?;
            }

            Ok(())
        }
        None => Err(format!("table {} not defined.", row.datatype())),
    }
}
