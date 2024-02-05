use crate::{
    data::model::meta::Meta,
    DBRow, SqlValue,
};
use std::fmt::Display;
use log::{info, trace};

#[derive(Debug)]
pub enum SerElement {
    Empty,
    Value(SqlValue),
    Sequence(Vec<SerElement>),
    Row(String, Vec<(String, SerElement)>),
}

impl SerElement {
    /// converts a SerElement structure to a list of rows ready to be posted to the database.
    ///
    pub fn as_rows(&self, context: Option<&str>) -> Vec<DBRow> {
        let mut result = vec![];
        match (self, context) {
            (SerElement::Empty, None) => todo!(),
            (SerElement::Empty, Some(_)) => todo!(),
            (SerElement::Value(_), None) => todo!(),
            (SerElement::Value(_), Some(_)) => todo!(),
            (SerElement::Sequence(s), context) => {
                for x in s {
                    let mut ss = x.as_rows(context);
                    result.append(&mut ss);
                }
            }
            (SerElement::Row(row_type, row_data), None) => {
                trace!("convert to rows: {}", row_type);
                let mut rr = DBRow::new(row_type.as_str());
                for (field_name, v) in row_data {
                    match self.get_relation(row_type.as_str(), field_name.as_str()) {
                        Some(_r) => {
                            // info!("found relation {} {}", row_type, field_name);
                            // match &r.kind {
                            //     One => {
                            //         handle_one_relation(v, &mut rr, field_name, row_type, &mut result);
                            //         let sub_row = &result[0];
                            //         for (f_fld, t_fld) in r.fields.iter() {
                            //             info!("field map {} <- {}", f_fld, t_fld);
                            //             rr.insert(
                            //                 f_fld.clone(),
                            //                 sub_row.get(t_fld).unwrap().clone(),
                            //             );
                            //         }
                            //     }
                            //     Many => {
                            //         handle_many_relation(v, &mut rr, field_name, row_type, &mut result);
                            //         for sub_row in result.iter_mut() {
                            //             for (f_fld, t_fld) in r.fields.iter() {
                            //                 info!("field map {} <- {}", t_fld, f_fld);
                            //                 sub_row.insert(
                            //                     t_fld.clone(),
                            //                     rr.get(f_fld).unwrap().clone(),
                            //                 );
                            //             }
                            //         }
                            //     }
                            //     ManyMany(rel_table) => {
                            //         info!("many to many relation {}", rel_table);
                            //         handle_many_many_relation(
                            //             v,
                            //             &mut rr,
                            //             field_name,
                            //             row_type,
                            //             rel_table,
                            //             &mut result,
                            //         );
                            //     }
                            // }
                        }
                        None => {
                            trace!("no relation {} {}", row_type, field_name);
                            handle_field(v, &mut rr, field_name, row_type, &mut result);
                        }
                    };
                }
                result.insert(0, rr);
            }
            (SerElement::Row(_, _r), Some(_n)) => todo!(),
        }
        result
    }

    fn get_relation(&self, _as_str_1: &str, _as_str_2: &str) -> Option<String> {
        todo!()
    }
}

impl Display for SerElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerElement::Empty => write!(f, "Empty"),
            SerElement::Value(x) => write!(f, "Value({x})"),
            SerElement::Sequence(s) => {
                write!(f, "Sequence(")?;
                let mut sep = "";
                for x in s {
                    write!(f, "{}{}", sep, x)?;
                    sep = ", ";
                }
                write!(f, ")")
            }
            SerElement::Row(name, values) => {
                write!(f, "{}(", name)?;
                let mut sep = "";
                for (k, v) in values {
                    write!(f, "{}{}={}", sep, k, v)?;
                    sep = ", ";
                }
                write!(f, ")")
            }
        }
    }
}

/// v - input value to be processed
/// rr - the direct result row. This will be enhanced by the new value
/// k - name of the field
/// n - name of the table this row belongs to
/// meta - information about relations and their fields
/// result - is the list of rows that are returned on the side while processing the value.
fn handle_field(
    v: &SerElement,
    rr: &mut DBRow,
    k: &String,
    n: &String,
    result: &mut Vec<DBRow>,
) {
    match v {
        SerElement::Empty => rr.insert(k.clone(), SqlValue(rusqlite::types::Value::Null)),
        SerElement::Value(v) => rr.insert(k.clone(), v.clone()),
        SerElement::Sequence(v) => {
            info!("sub row sequence {}.{}", n, k);
            for x in v {
                let mut sub_rows = x.as_rows(None);
                result.append(&mut sub_rows);
            }
        }
        SerElement::Row(n, _) => {
            info!("sub row {}.{}", n, k);
            let mut sub_rows = v.as_rows(None);
            result.append(&mut sub_rows);
        }
    }
}

/// v - input value to be processed
/// rr - the direct result row. This will be enhanced by the new value
/// k - name of the field
/// n - name of the table this row belongs to
/// meta - information about relations and their fields
/// result - is the list of rows that are returned on the side while processing the value.
#[allow(dead_code)]
fn handle_one_relation(
    v: &SerElement,
    _rr: &mut DBRow,
    _k: &String,
    _n: &String,
    // meta: &Meta,
    result: &mut Vec<DBRow>,
) {
    match v {
        SerElement::Empty => todo!("implement empty relationship"),
        SerElement::Value(_v) => panic!("relation cannot use atomic values"),
        SerElement::Sequence(_v) => panic!("one relations cannot refer to vectors"),
        SerElement::Row(_n, _) => {
            let mut rr = v.as_rows(None);
            result.append(&mut rr);
        }
    }
}

/// v - input value to be processed
/// rr - the direct result row. This will be enhanced by the new value
/// k - name of the field
/// n - name of the table this row belongs to
/// meta - information about relations and their fields
/// result - is the list of rows that are returned on the side while processing the value.
#[allow(dead_code)]
fn handle_many_relation(
    v: &SerElement,
    _rr: &mut DBRow,
    k: &String,
    n: &String,
    _meta: &Meta,
    result: &mut Vec<DBRow>,
) {
    info!("handle many relation {} {}", k, n);
    match v {
        SerElement::Empty => todo!("implement empty relationship"),
        SerElement::Value(_v) => panic!("relation cannot use atomic values"),
        SerElement::Sequence(v) => {
            for x in v {
                info!("many: handle row {}", x);
                let mut rr = x.as_rows(None);
                result.append(&mut rr);
            }
        }
        SerElement::Row(_n, _) => {
            let mut rr = v.as_rows(None);
            result.append(&mut rr);
        }
    }
}
/// v - input value to be processed
/// rr - the direct result row. This will be enhanced by the new value
/// k - name of the field
/// n - name of the table this row belongs to
/// meta - information about relations and their fields
/// result - is the list of rows that are returned on the side while processing the value.
#[allow(dead_code)]
fn handle_many_many_relation(
    v: &SerElement,
    rr: &mut DBRow,
    k: &String,
    n: &String,
    rel_table: &String,
    meta: &Meta,
    result: &mut Vec<DBRow>,
) {
    info!("handle many many relation {} {} {}", k, n, rel_table);
    match v {
        SerElement::Empty => todo!("implement empty relationship"),
        SerElement::Value(_v) => panic!("relation cannot use atomic values"),
        SerElement::Sequence(v) => {
            for x in v {
                let mut sub_rows = x.as_rows(None);
                let mut rel_row = DBRow::new(&rel_table);
                let rel = meta.get_relation(n, k).unwrap();
                let row1 = &sub_rows[0];
                for (fld, target) in rel.fields.iter() {
                    let vtarget;
                    if let Some(v0) = rr.get(fld.as_str()) {
                        vtarget = v0.clone();
                    } else {
                        if let Some(v0) = row1.get(fld.as_str()) {
                            vtarget = v0.clone();
                        } else {
                            panic!("field {} could not be mapped.", fld);
                        }
                    }
                    rel_row.insert(target.clone(), vtarget);
                }
                result.append(&mut sub_rows);
                result.push(rel_row);
            }
        }
        SerElement::Row(_n, _) => todo!(),
    }
}

impl From<SqlValue> for SerElement {
    fn from(value: SqlValue) -> Self {
        SerElement::Value(value)
    }
}

impl From<&SerElement> for SqlValue {
    fn from(value: &SerElement) -> Self {
        match value {
            SerElement::Empty => todo!(),
            SerElement::Value(v) => v.clone(),
            SerElement::Sequence(_) => todo!(),
            SerElement::Row(_, _) => todo!(),
        }
    }
}
