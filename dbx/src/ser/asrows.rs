use log::{debug, error, trace};

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
                self.serialize_row(row_type, row_data, &mut result)?;
            }
            _ => todo!(),
        }
        for x in result.iter() {
            debug!("{x}");
        }
        Ok(result)
    }
}

fn print_fields(current_table: &crate::data::model::Table) {
    for f in current_table.fields() {
        debug!("-  {}", f.name());
    }
}

impl DataModel {
    fn dep_list_gen(
        &self,
        t: &crate::data::model::Table,
        row_type: &String,
        field_name: &String,
        child: &SerElement,
        current_table: &crate::data::model::Table,
        rr: &DBRow,
        result: &mut Vec<DBRow>,
    ) -> Result<(), Error> {
        let f = t.fields().find(|x| match &x.fieldtype {
            FieldType::Lookup { table, as_field } => table == row_type && as_field == field_name,
            _ => false,
        });
        if let Some(f) = f {
            debug!("references field {row_type} {field_name} {f:?}");
            for mut x in self.as_rows(child)? {
                let current_key_field = current_table.key().last().unwrap();
                let current_key_value = rr.get(current_key_field).unwrap();
                x.insert(f.name.clone(), current_key_value.clone());
                result.push(x)
            }
        };
        Ok(())
    }

    fn gen_dep(
        &self,
        child: &SerElement,
        row_type: &String,
        field_name: &String,
        current_table: &crate::data::model::Table,
        rr: &DBRow,
        result: &mut Vec<DBRow>,
    ) -> Result<(), Error> {
        Ok(if let SerElement::Row(name, _values) = child {
            if let Some(t) = self.find_table(name) {
                self.dep_list_gen(t, row_type, field_name, child, current_table, rr, result)?;
            } else {
                todo!("handle table not declared");
            }
        } else {
            todo!("sequence contains not only rows!");
        })
    }

    fn gen_field(
        &self,
        current_table: &crate::data::model::Table,
        field_name: &String,
        rr: &mut DBRow,
        value_to_serialize: &SerElement,
        row_type: &String,
        result: &mut Vec<DBRow>,
    ) -> Result<(), Error> {
        Ok(
            if let Some(Field {
                name: fld_name,
                key: _,
                fieldtype,
                ..
            }) = current_table.fields().find(|x| &x.name == field_name)
            {
                match fieldtype {
                    FieldType::Text(_) => {
                        rr.insert(field_name.clone(), SqlValue::from(value_to_serialize));
                    }
                    FieldType::DependentList(_) => {
                        if let SerElement::Sequence(s) = value_to_serialize {
                            for child in s {
                                debug!("dependent list building from {:}", child);
                                self.gen_dep(
                                    child,
                                    row_type,
                                    field_name,
                                    current_table,
                                    &*rr,
                                    result,
                                )?;
                            }
                        } else {
                            todo!("non squences");
                        }
                    }
                    FieldType::Lookup { table, as_field } => {
                        if let SerElement::Row(s, values) = value_to_serialize {
                            if let Some(val) = values.iter().find(|(k, v)| k == as_field) {
                                rr.insert(field_name.clone(), SqlValue::from(&val.1));
                                debug!("serialize lookup row '{}'", table);
                                self.serialize_row(table, values, result).unwrap();
                            }
                            else {
                                panic!("could not locate name {}", as_field)
                            }
                        }
                    }
                    _ => {
                        debug!("not implemented field {fld_name}: {:?}", fieldtype);
                    }
                };
            } else {
                debug!("{} not found. -> {:}", field_name, value_to_serialize);
            },
        )
    }

    fn serialize_row(
        &self,
        row_type: &String,
        row_data: &Vec<(String, SerElement)>,
        result: &mut Vec<DBRow>,
    ) -> Result<(), Error> {
        debug!("serialize row '{}'", row_type);
        let row_type_str = row_type.as_str();
        Ok(
            if let Some(current_table) = self.tables().find(|x| x.name() == row_type_str) {
                trace!("convert to rows: {}", row_type_str);
                print_fields(current_table);
                let mut rr = DBRow::new(row_type_str);
                for (field_name, v) in row_data {
                    self.gen_field(current_table, field_name, &mut rr, v, row_type, result)?;
                }
                result.insert(0, rr);
            } else {
                error!("table not found: {}", row_type);
            },
        )
    }
}
