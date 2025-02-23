use super::Table;
use serde::{
    de::{MapAccess, SeqAccess},
    Deserialize, Deserializer,
};
use std::{default, error::Error, fmt::Display};

fn de<'de, T>() -> Option<T>
where
    T: serde::Deserialize<'de>,
{
    None
}

#[derive(Debug)]
pub enum DeErr {
    General,
    Message(String),
}
impl Display for DeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for DeErr {}
impl serde::de::Error for DeErr {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DeErr::Message(msg.to_string())
    }
}

struct Temp<'table> {
    table: &'table Table,
    current_field: Option<String>,
    col: usize,
    rowidx: usize,
    is_key: bool,
}

impl<'table> Temp<'table> {
    fn new(table: &'table Table) -> Self {
        Self {
            table,
            col: 0,
            rowidx: 0,
            current_field: None,
            is_key: true,
        }
    }
}

impl<'de> Deserializer<'de> for &mut Temp<'_> {
    type Error = DeErr;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Some(name) = self.current_field.as_ref() {
            let row = self.table.row(self.rowidx);
            if let Some(v) = row.get(&name) {
                let v = v.parse::<i64>().unwrap();
                visitor.visit_i64(v)
            } else {
                Err(DeErr::Message(format!(
                    "column {} invalid for getting a value",
                    name
                )))
            }
        } else {
            Err(DeErr::Message(format!("no current field")))
        }
}

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.is_key {
            let cols = self.table.columns();
            if let Some(c) = cols.get(self.col) {
                self.current_field = Some(c.clone());
                visitor.visit_str(c)
            } else {
                Err(DeErr::Message(format!("column {} invalid", self.col)))
            }
        } else {
            if let Some(name) = self.current_field.as_ref() {
                let row = self.table.row(self.rowidx);
                if let Some(v) = row.get(&name) {
                    visitor.visit_str(&v)
                } else {
                    Err(DeErr::Message(format!(
                        "column {} invalid for getting a value",
                        name
                    )))
                }
            } else {
                Err(DeErr::Message(format!("no current field")))
            }
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let c = self.table.columns();
        if let Some(colname) = c.get(self.col) {
            match visitor.visit_str::<DeErr>(&colname) {
                Ok(v) => {
                    self.current_field = Some(colname.clone());
                    Ok(v)
                }
                e => e,
            }
        } else {
            Err(DeErr::Message(format!("column {}", self.col)))
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(false)
    }
}

impl<'de> MapAccess<'de> for Temp<'_> {
    type Error = DeErr;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        self.is_key = true;
        self.current_field = None;
        let col_count = self.table.column_count();
        if self.col < col_count {
            seed.deserialize(&mut *self).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.is_key = false;
        assert!(self.current_field.is_some());
        let r = seed.deserialize(&mut *self);
        self.col += 1;
        r
    }
}

impl<'de> SeqAccess<'de> for Temp<'_> {
    type Error = DeErr;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.rowidx < self.table.lines() {
            self.rowidx += 1;
            self.col = 0;
            seed.deserialize(&mut *self).map(Some)
        } else {
            Ok(None)
        }
    }
}

pub fn extract_from_table<D>(tab: &Table) -> Result<D, DeErr>
where
    D: serde::Deserialize<'static>,
{
    let mut t = Temp::new(&tab);
    D::deserialize(&mut t)
}
