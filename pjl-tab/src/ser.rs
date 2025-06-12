use super::Table;
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize,
};
use std::fmt::Error;
use tracing::{info, instrument, trace};

pub fn table_from<T>(x: T) -> Result<Table, String>
where
    T: serde::Serialize,
{
    let mut d = TableSer::new();
    match &x.serialize(&mut d) {
        Ok(_) => Ok(d.t),
        Err(e) => Err(format!("de error {}", e)),
    }
}

enum SerState {
    Default,
    MapKey,
    MapValue,
}
struct TableSer {
    state: SerState,
    t: Table,
    row: Option<usize>,
    col: Option<&'static str>,
    val: Option<String>,
}

impl<'a> TableSer {
    fn new() -> Self {
        Self {
            state: SerState::Default,
            t: Table::new(),
            row: None,
            col: None,
            val: None,
        }
    }
}

impl<'a> serde::ser::Serializer for &'a mut TableSer {
    type Ok = ();

    type Error = std::fmt::Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

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

    #[instrument(skip_all)]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        match self.state {
            SerState::Default => {
                self.val = Some(v.into());
            }
            SerState::MapKey => {
                self.col = Some(pjl_static_strings::StringTable::get(v));
            }
            SerState::MapValue => {
                let r = self.t.row(self.row.unwrap());
                r.set(self.col.unwrap(), v);
                self.col = None;
            }
        }
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
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

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        let _ = name;
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        let _ = variant;
        let _ = name;
        let _ = variant_index;
        todo!()
    }

    #[instrument(skip_all)]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let _ = len;
        Ok(self)
    }

    #[instrument(skip_all)]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let _ = len;
        Ok(self)
    }

    #[instrument(skip_all)]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let _ = len;
        let _ = name;
        Ok(self)
    }

    #[instrument(skip_all)]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let _ = len;
        let _ = variant;
        let _ = variant_index;
        let _ = name;
        Ok(self)
    }

    #[instrument(skip_all)]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let _ = len;
        Ok(self)
    }

    #[instrument(skip_all)]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let _ = name;
        let _ = len;
        assert!(self.row.is_some());
        Ok(self)
    }

    #[instrument(skip_all)]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let _ = name;
        let _ = variant_index;
        let _ = variant;
        let _ = len;
        Ok(self)
    }
}

impl SerializeSeq for &mut TableSer {
    type Ok = ();

    type Error = std::fmt::Error;

    #[instrument(skip_all)]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        trace!("serialize element");
        let r = self.t.new_row();
        self.row = Some(r.id);
        value.serialize(&mut **self)
    }

    #[instrument(skip_all)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeTuple for &mut TableSer {
    type Ok = ();

    type Error = std::fmt::Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeTupleStruct for &mut TableSer {
    type Ok = ();

    type Error = std::fmt::Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    #[instrument(skip_all)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeTupleVariant for &mut TableSer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeMap for &mut TableSer {
    type Ok = ();

    type Error = Error;

    #[instrument(skip_all)]
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        info!("serialize key");
        self.state = SerState::MapKey;
        key.serialize(&mut **self)
    }

    #[instrument(skip_all)]
    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        info!("serialize value");
        self.state = SerState::MapValue;
        _value.serialize(&mut **self)
    }

    #[instrument(skip_all)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        info!("end");
        self.state = SerState::Default;
        Ok(())
    }
}

impl<'a> SerializeStruct for &mut TableSer {
    type Ok = ();

    type Error = Error;

    #[instrument(skip_all)]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        trace!("serialize field");
        self.col = Some(key);
        match value.serialize(&mut **self) {
            Ok(_) => {
                let r = self.t.row(self.row.unwrap());
                r.set(self.col.unwrap(), self.val.as_ref().unwrap());
            }
            Err(_) => todo!(),
        }
        Ok(())
    }

    #[instrument(skip_all)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeStructVariant for &mut TableSer {
    type Ok = ();

    type Error = Error;

    #[instrument(skip_all)]
    fn serialize_field<T>(&mut self, key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = key;
        todo!()
    }

    #[instrument(skip_all)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
