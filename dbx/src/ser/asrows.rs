use edm::value::Value as EdmValue;
use edm::{list::ListValue, structure::StructureValue};
use log::{debug, error, trace};

use crate::{
    data::model::{DataModel, Field, FieldType},
    error::Error,
};

pub trait AsRows {
    fn as_rows(&self, el: &edm::value::Value) -> Result<Vec<StructureValue>, Error>;
}

impl AsRows for DataModel {
    fn as_rows(&self, el: &edm::value::Value) -> Result<Vec<StructureValue>, Error> {
        let mut result = vec![];
        match el {
            edm::value::Value::PrimitiveValue(_) => panic!("primitive values cannot be converted."),
            edm::value::Value::StructureValue(StructureValue { datatype, values }) => {
                let values = values.iter().map(|(x, y)| (x.clone(), y.clone())).collect();
                self.serialize_row(datatype, &values, &mut result)?;
            }
            edm::value::Value::ListValue(ListValue { values }) => {
                for x in values {
                    let mut ss = self.as_rows(x)?;
                    result.append(&mut ss);
                }
            }
        }
        for x in result.iter() {
            debug!("{x:?}");
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
    /// if table t contains a field with name <field_name>
    /// that is a lookup field to table <row_type>
    /// generate rows for child, this should result
    ///
    fn dep_list_gen(
        &self,
        t: &crate::data::model::Table,
        row_type: &String,
        field_name: &String,
        child: &EdmValue,
        current_table: &crate::data::model::Table,
        current_row: &StructureValue,
        result: &mut Vec<StructureValue>,
    ) -> Result<(), Error> {
        trace!(
            "dep_list_gen {} {} {} {}",
            row_type,
            field_name,
            t.name(),
            current_table.name()
        );
        trace!{"find lookup field {field_name} for table {} in table {}", row_type, t.name()}
        let f = t.fields().find(|x| match &x.fieldtype {
            FieldType::Lookup { table, as_field } => table == row_type && as_field == field_name,
            _ => false,
        });

        if let Some(f) = f {
            debug!("field found {}", f.name());
            for mut x in self.as_rows(child)? {
                trace!("for row {x:?}");
                let current_key_field = current_table.key().last().unwrap();
                trace!("key field: {}", current_key_field);
                let current_key_value = &current_row[current_key_field];
                x[f.name.as_str()] = current_key_value.clone();
                result.push(x)
            }
        };
        Ok(())
    }

    fn gen_dep(
        &self,
        child: &EdmValue,
        row_type: &String,
        field_name: &String,
        current_table: &crate::data::model::Table,
        rr: &StructureValue,
        result: &mut Vec<StructureValue>,
    ) -> Result<(), Error> {
        Ok(
            if let EdmValue::StructureValue(StructureValue { datatype, .. }) = child {
                if let Some(t) = self.find_table(&datatype) {
                    self.dep_list_gen(t, row_type, field_name, child, current_table, rr, result)?;
                } else {
                    todo!("handle table not declared");
                }
            } else {
                todo!("sequence contains not only rows!");
            },
        )
    }

    fn gen_field(
        &self,
        current_table: &crate::data::model::Table,
        field_name: &String,
        current_row: &mut StructureValue,
        value: &edm::value::Value,
        row_type: &String,
        result: &mut Vec<StructureValue>,
    ) -> Result<(), Error> {
        Ok(
            // if current table definition has field <field_name> -> fld_name
            if let Some(Field { fieldtype, .. }) =
                current_table.fields().find(|x| &x.name == field_name)
            {
                match fieldtype {
                    // direct text is just added to the current row
                    FieldType::Text(_) => {
                        current_row[field_name.as_str()] = value.clone();
                    }
                    FieldType::DependentList(_) => match value {
                        EdmValue::PrimitiveValue(_) => todo!(),
                        EdmValue::StructureValue(StructureValue { values, .. }) => {
                            for (_, child) in values {
                                debug!("dependent list building from {:?}", child);
                                self.gen_dep(
                                    child,
                                    row_type,
                                    field_name,
                                    current_table,
                                    &*current_row,
                                    result,
                                )?;
                            }
                        }
                        EdmValue::ListValue(ListValue { values }) => {
                            for child in values {
                                debug!("List: dependent list building from {:?}", child);
                                self.gen_dep(
                                    child,
                                    row_type,
                                    field_name,
                                    current_table,
                                    &*current_row,
                                    result,
                                )?;
                            }
                        }
                    },
                    FieldType::Lookup { table, as_field } => {
                        if let EdmValue::StructureValue(StructureValue { values, .. }) = value
                        {
                            if let Some(val) = values.iter().find(|(k, _)| *k == as_field) {
                                current_row[field_name.as_str()] = val.1.clone();
                                debug!("serialize lookup row '{}'", table);
                                let values =
                                    values.iter().map(|(x, y)| (x.clone(), y.clone())).collect();
                                self.serialize_row(table, &values, result).unwrap();
                            } else {
                                panic!("could not locate name {}", as_field)
                            }
                        }
                    }
                    _ => {
                        debug!("not implemented field {field_name}: {:?}", fieldtype);
                    }
                };
            } else {
                debug!("{} not found. -> {:?}", field_name, value);
            },
        )
    }

    fn serialize_row(
        &self,
        row_type: &String,
        row_data: &Vec<(String, EdmValue)>,
        result: &mut Vec<StructureValue>,
    ) -> Result<(), Error> {
        debug!("serialize row '{}'", row_type);
        let row_type_str = row_type.as_str();
        Ok(
            if let Some(current_table) = self.tables().find(|x| x.name() == row_type_str) {
                trace!("convert to rows: {}", row_type_str);
                print_fields(current_table);
                let mut rr = StructureValue::new_with_type(row_type_str);
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
