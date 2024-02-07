mod element;

use super::{DBRow, SqlValue};
use crate::data::model::DataModel;
use crate::error::Error;
use asrows::AsRows;
use element::SerElement;
use serde::ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant};
use serde::{ser, Serialize};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::result::Result;
use log::{info, trace};

pub fn serialize_row<T>(model: Rc<DataModel>, v: T) -> Result<Vec<crate::DBRow>, Error>
where
    T: serde::Serialize,
{
    // let meta = model.meta();
    let s = RowSerializer::new(model.clone());
    match &v.serialize(&s) {
        // Ok(x) => x.as_rows(None),
        Ok(x) => model.as_rows(x),
        Err(_) => todo!(),
    }
}
mod asrows;

pub fn serialize_row_with_default<T>(model: Rc<DataModel>, default: DBRow, v: T) -> Vec<DBRow>
where
    T: serde::Serialize,
{
    // let cr = model.meta();
    let mut s = RowSerializer::new(model.clone());
    s.with_default(default);
    match &v.serialize(&s) {
        Ok(x) => x.as_rows(None),
        Err(_) => todo!(),
    }
}

#[derive(Debug, Clone)]
pub struct CopyRuleLib {
    rules: BTreeMap<String, CopyRule>,
}

#[allow(dead_code)]
impl CopyRuleLib {
    pub fn new() -> Self {
        Self {
            rules: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, name: &str, rule: CopyRule) {
        self.rules.insert(name.into(), rule);
    }

    fn get(&self, rel: &String) -> Option<&CopyRule> {
        if let Some(rules) = self.rules.get(rel) {
            Some(rules)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldCopyRule {
    pub source: String,
    pub target: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ManyToMany {
    name: String,
    table1: String,
    copy1: CopyRule,
    table2: String,
    copy2: CopyRule,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CopyRule {
    field_mappings: Vec<FieldCopyRule>,
    many_to_many: Option<Box<ManyToMany>>,
}

impl CopyRule {
    pub fn new(field_mappings: Vec<FieldCopyRule>) -> Self {
        Self {
            field_mappings,
            many_to_many: None,
        }
    }
    pub fn many_to_many(
        mut self,
        name: &str,
        table1: &str,
        copy1: CopyRule,
        table2: &str,
        copy2: CopyRule,
    ) -> Self {
        self.many_to_many = Some(Box::new(ManyToMany {
            name: name.to_string(),
            table1: table1.to_string(),
            copy1: copy1,
            table2: table2.to_string(),
            copy2: copy2,
        }));
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Dependency {
    record_number: usize,
    copy_rule: CopyRule,
}

pub struct NameSerializer {}

impl<'de> ser::Serializer for &NameSerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

struct RowSerializer {
    model: Rc<DataModel>,
    default: Option<DBRow>,
}
#[derive(Debug)]
struct DBRowSerializer {
    model: Rc<DataModel>,
    result: Vec<(String, SerElement)>,
    name: Option<String>,
}

#[allow(dead_code)]
impl DBRowSerializer {
    pub fn new(model: Rc<DataModel>, name: &str) -> Self {
        Self {
            model,
            result: vec![],
            name: Some(name.into()),
        }
    }

    pub fn get_default_values() -> Vec<(String, SqlValue)> {
        vec![]
    }
}
impl SerializeStruct for DBRowSerializer {
    type Ok = SerElement;

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // info!("serialize {}", key);
        let s = SQLValueSerializer {
            model: self.model.clone(),
            name: Some(key.into()),
        };
        self.result.push((key.into(), value.serialize(s).unwrap()));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        trace!("serialize struct end: {:?}", self.name);
        Ok(SerElement::Row(self.name.unwrap(), self.result))
    }
}

impl SerializeStructVariant for DBRowSerializer {
    type Ok = SerElement;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let s = SQLValueSerializer {
            model: self.model.clone(),
            name: Some(key.into()),
        };
        self.result.push((key.into(), value.serialize(s).unwrap()));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        trace!("serialize struct variant end: {:?}", self.name);
        Ok(SerElement::Row(self.name.unwrap(), self.result))
    }
}

impl RowSerializer {
    pub fn new(model: Rc<DataModel>) -> Self {
        Self {
            model,
            default: None,
        }
    }

    fn with_default(&mut self, default: DBRow) {
        self.default = Some(default);
    }
}

impl ser::SerializeMap for DBRowSerializer {
    type Ok = SerElement;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let s = NameSerializer {};
        let _n = key.serialize(&s).unwrap();
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let s = SQLValueSerializer {
            model: self.model.clone(),
            name: None,
        };
        _value.serialize(s).unwrap();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value("N/N".into()))
    }
}

struct SQLValueSerializer {
    model: Rc<DataModel>,
    name: Option<String>,
}

impl ser::Serializer for SQLValueSerializer {
    type Ok = SerElement;
    type Error = Error;
    type SerializeSeq = TableSerializer;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = StructSerializer;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = StructSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v as u16)))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Value(SqlValue::new(
            rusqlite::types::Value::Null,
        )))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let _ = variant_index;
        let _ = name;
        // using the index might lead to the most dense way, however it is hard to maintain the sequence constant, since this is
        // implicitly derived from the sequence in the enum.
        // instead one could use digits as special names in order to assign ids to the values.
        // Ok(SerElement::Value(SqlValue::new(variant_index)))
        Ok(SerElement::Value(variant.into()))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(TableSerializer {
            model: self.model,
            rows: vec![],
            name: self.name.clone(),
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(StructSerializer::new(self.model.clone(), None))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        // let r = serialize_row_with_default(self.model.clone(), default, value);
        // for x in r {
        //     self.rows.push(x);
        // }

        Ok(StructSerializer::new(self.model.clone(), Some(name.into())))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let _ = (name, len, variant_index);

        Ok(StructSerializer::new(
            self.model.clone(),
            Some(variant.into()),
        ))
    }
}

impl<'de> ser::Serializer for &RowSerializer {
    type Ok = SerElement;
    type Error = Error;
    type SerializeSeq = TableSerializer;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = DBRowSerializer;
    type SerializeStruct = DBRowSerializer;
    type SerializeStructVariant = DBRowSerializer;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        info!("serialize sequence");
        let s = TableSerializer {
            rows: vec![],
            model: self.model.clone(),
            name: None,
        };
        Ok(s)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        info!("serialize struct");
        let s = DBRowSerializer::new(self.model.clone(), "person");
        Ok(s)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        info!("serialize struct");
        let s = DBRowSerializer::new(self.model.clone(), name);
        Ok(s)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!();
    }
}

#[derive(Debug)]
struct StructSerializer {
    parent: DBRowSerializer,
    key: Option<String>,
}

impl StructSerializer {
    fn new(model: Rc<DataModel>, name: Option<String>) -> Self {
        Self {
            key: None,
            parent: DBRowSerializer {
                model,
                result: vec![],
                name,
            },
        }
    }

    fn serialize_field_impl<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        SerializeStruct::serialize_field(&mut self.parent, key, value)
    }
}

impl SerializeMap for StructSerializer {
    type Ok = SerElement;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let s = NameSerializer {};
        self.key = Some(key.serialize(&s).unwrap());
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let k = &self.key;
        if let Some(key) = k {
            let s = SQLValueSerializer {
                model: self.parent.model.clone(),
                name: Some(key.into()),
            };
            self.parent
                .result
                .push((key.into(), value.serialize(s).unwrap()));
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        trace!("serialize map for struct serializer - end");
        Ok(SerElement::Row("map".to_string(), self.parent.result))
    }
}

impl SerializeStruct for StructSerializer {
    type Ok = SerElement;

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_field_impl(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        trace!(
            "serialize struct for struct serializer - end - parent name: {:?}",
            self.parent.name
        );
        Ok(SerElement::Row(
            self.parent.name.unwrap(),
            self.parent.result,
        ))
    }
}

impl SerializeStructVariant for StructSerializer {
    type Ok = SerElement;

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_field_impl(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        trace!(
            "serialize struct variant for struct serializer - end - entity name: {:#?}",
            self.parent.name.as_ref().unwrap()
        );
        Ok(SerElement::Row(
            self.parent.name.unwrap(),
            self.parent.result,
        ))
    }
}

struct TableSerializer {
    rows: Vec<SerElement>,
    model: Rc<DataModel>,
    name: Option<String>,
}

impl SerializeSeq for TableSerializer {
    type Ok = SerElement;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let s = SQLValueSerializer {
            model: self.model.clone(),
            name: self.name.clone(),
        };

        // let r = serialize_row_with_default(self.model.clone(), default, value);
        self.rows.push(value.serialize(s).unwrap());
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(SerElement::Sequence(self.rows))
    }
}

