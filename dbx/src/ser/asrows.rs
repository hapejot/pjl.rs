use log::{debug, trace};

use crate::{
    data::model::{DataModel, Field, FieldType},
    error::Error,
    DBRow, SqlValue,
};

use super::element::SerElement;

pub trait AsRows {
    fn as_rows(&self, el: &SerElement) -> Result<Vec<DBRow>, Error>;
}

impl AsRows for DataModel {
    fn as_rows(&self, el: &SerElement) -> Result<Vec<DBRow>, Error> {
        let mut result = vec![];
        match el {
            SerElement::Sequence(s) => {
                for x in s {
                    let mut ss = self.as_rows(x)?;
                    result.append(&mut ss);
                }
            }
            SerElement::Row(row_type, row_data) => {
                let row_type_str = row_type.as_str();
                if let Some(current_table) = self.tables().find(|x| x.name() == row_type_str) {
                    trace!("convert to rows: {}", row_type_str);
                    for f in current_table.fields() {
                        debug!("-  {}", f.name());
                    }
                    let mut rr = DBRow::new(row_type_str);
                    for (field_name, v) in row_data {
                        if let Some(Field {
                            name: _,
                            key: _,
                            fieldtype,
                            ..
                        }) = current_table.fields().find(|x| &x.name == field_name)
                        {
                            match fieldtype {
                                FieldType::Text(_) => {
                                    rr.insert(field_name.clone(), SqlValue::from(v));
                                }
                                FieldType::DependentList(_) => {
                                    if let SerElement::Sequence(s) = v {
                                        for child in s {
                                            debug!("dependent list building from {:}", child);
                                            if let SerElement::Row(name, _values) = child {
                                                if let Some(t) = self.find_table(name) {
                                                    // debug!("filling table {:#?}", t);
                                                    let f =
                                                        t.fields().find(|x| match &x.fieldtype {
                                                            FieldType::Lookup {
                                                                table,
                                                                as_field,
                                                            } => {
                                                                table == row_type
                                                                    && as_field == field_name
                                                            }
                                                            _ => false,
                                                        });
                                                    if let Some(f) = f {
                                                        debug!("references field {row_type} {field_name} {f:?}");
                                                        for mut x in self.as_rows(child)? {
                                                            let current_key_field =
                                                                current_table.key().last().unwrap();
                                                            let current_key_value =
                                                                rr.get(current_key_field).unwrap();
                                                            x.insert(
                                                                f.name.clone(),
                                                                current_key_value.clone(),
                                                            );
                                                            result.push(x)
                                                        }
                                                    };
                                                } else {
                                                    todo!("handle table not declared");
                                                }
                                            } else {
                                                todo!("sequence contains not only rows!");
                                            }
                                        }
                                    } else {
                                        todo!("non squences");
                                    }
                                }
                                _ => {
                                    debug!("not implemented field: {:?}", fieldtype);
                                }
                            };
                        } else {
                            debug!("{} not found. -> {:}", field_name, v);
                        }
                    }
                    result.insert(0, rr);
                }
            }
            _ => todo!(),
        }
        for x in result.iter() {
            debug!("{x}");
        }
        Ok(result)
    }
}
