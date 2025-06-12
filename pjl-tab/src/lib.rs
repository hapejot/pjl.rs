use memuse::DynamicUsage;
use pjl_static_strings::StringTable;
use serde::ser::{SerializeSeq, SerializeStruct};
use std::{collections::HashMap, fmt::{Debug, Write}, sync::Mutex};
use tracing::instrument;

pub struct Row<'a> {
    table: &'a Table,
    id: usize,
}
impl<'a> Row<'a> {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set(&self, name: &str, value: &str) {
        if let Some(idx) = self.table.make_col_idx(name) {
            self.table.put(self.id, idx, value);
        } else {
            panic!("Column {} not found", name)
        }
    }
    pub fn get(&self, name: &str) -> Option<String> {
        if let Some(idx) = self.table.translate_col(name) {
            self.table.get(self.id, idx)
        } else {
            panic!("no column {}", name);
        }
    }

    pub fn columns(&self) -> Vec<String> {
        self.table.columns()
    }
}

// impl<'a> Index<&str> for Row<'a> {
//     type Output = String;

//     fn index(&self, index: &str) -> &Self::Output {
//         if let Some(idx) = self.table.translate_col(index) {
//             self.table.get(self.id, idx).unwrap()
//         }
//     }
// }

// impl<'a> IndexMut<&str> for Row<'a> {
//     fn index_mut(&mut self, index: &str) -> &mut Self::Output {
//         if let Some(idx) = self.table.translate_col(index) {
//             self.table.get_mut(self.id, idx)
//         }
//     }
// }

#[derive(Debug)]
struct TableVar {
    columns: Vec<String>,
    data: HashMap<(usize, usize), String>,
    row_count: usize,
}

#[derive(Debug)]
pub struct Table {
    d: Mutex<TableVar>,
}

#[allow(dead_code)]
pub struct TableIterator<'a> {
    tab: &'a Table,
    idx: usize,
}

impl<'a> Iterator for TableIterator<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> IntoIterator for &'a Table {
    type Item = Row<'a>;
    type IntoIter = TableIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl serde::Serialize for Table {
    #[instrument(skip_all)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let row_count = self.lines();
        let mut seq = serializer.serialize_seq(Some(row_count))?;
        for rownum in 1..row_count + 1 {
            seq.serialize_element(&self.row(rownum))?;
        }
        seq.end()
    }
}

impl<'a> serde::Serialize for Row<'a> {
    #[instrument(skip_all)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sstruct = serializer.serialize_struct("row", self.columns().len())?;
        let cols = self.columns();
        for x in cols {
            let key = StringTable::get(&x);
            match self.get(&x) {
                Some(v) => sstruct.serialize_field(key, &v)?,
                None => sstruct.skip_field(key)?,
            }
        }
        sstruct.end()
    }
}

impl DynamicUsage for Table {
    fn dynamic_usage(&self) -> usize {
        match self.d.try_lock() {
            Ok(x) => x.dynamic_usage() + size_of_val(self),
            Err(_) => todo!(),
        }
    }

    fn dynamic_usage_bounds(&self) -> (usize, Option<usize>) {
        todo!()
    }
}

impl DynamicUsage for TableVar {
    fn dynamic_usage(&self) -> usize {
        self.columns.dynamic_usage() + self.data.dynamic_usage() + self.row_count.dynamic_usage()
    }

    fn dynamic_usage_bounds(&self) -> (usize, Option<usize>) {
        todo!()
    }
}

impl Table {
    pub fn new() -> Self {
        let columns = vec![];
        let row_count = 0;
        let data = HashMap::new();
        Self {
            d: Mutex::new(TableVar {
                columns,
                row_count,
                data,
            }),
        }
    }

    pub fn size(&self) -> usize {
        size_of_val(self)
    }

    pub fn add_column(&self, name: &str) -> Result<(), String> {
        let name = name.to_lowercase().to_string();
        if let Ok(mut x) = self.d.try_lock() {
            if x.columns.contains(&name) {
                return Err(format!("column {name} exists already."));
            }
            x.columns.push(name);
        }
        Ok(())
    }

    pub fn lines(&self) -> usize {
        if let Ok(x) = self.d.try_lock() {
            x.row_count
        } else {
            todo!("what if the table cannot be locked.")
        }
    }

    pub fn new_row(&self) -> Row<'_> {
        if let Ok(mut x) = self.d.try_lock() {
            x.row_count += 1;
            Row {
                table: self,
                id: x.row_count,
            }
        } else {
            todo!("what if the table cannot be locked.")
        }
    }
    pub fn row(&self, idx: usize) -> Row<'_> {
        if let Ok(_x) = self.d.try_lock() {
            Row {
                table: self,
                id: idx,
            }
        } else {
            todo!("what if the table cannot be locked.")
        }
    }

    fn translate_col(&self, index: &str) -> Option<usize> {
        if let Ok(x) = self.d.try_lock() {
            let needle = index.to_lowercase();
            if let Some(found) = x.columns.iter().position(|x| x == &needle) {
                Some(found + 1)
            } else {
                None
            }
        } else {
            todo!("what if the table cannot be locked.")
        }
    }

    fn make_col_idx(&self, index: &str) -> Option<usize> {
        if let Ok(mut x) = self.d.try_lock() {
            let needle = index.to_lowercase();
            if let Some(found) = x.columns.iter().position(|x| x == &needle) {
                Some(found + 1)
            } else {
                x.columns.push(needle.to_string());
                let idx = x.columns.len();
                Some(idx)
            }
        } else {
            todo!("what if the table cannot be locked.")
        }
    }

    pub fn put(&self, id: usize, idx: usize, value: &str) {
        if let Ok(mut x) = self.d.try_lock() {
            x.data.insert((id, idx), value.to_string());
        } else {
            todo!("what if the table cannot be locked.")
        }
    }

    pub fn get(&self, id: usize, idx: usize) -> Option<String> {
        if let Ok(x) = self.d.try_lock() {
            let k = (id, idx);
            x.data.get(&k).map(|x| x.clone())
        } else {
            todo!("what if the table cannot be locked.")
        }
    }

    pub fn dump(&self, out: &mut impl Write) {
        if let Ok(x) = self.d.try_lock() {
            let mut w = vec![0; x.columns.len()];
            // calculate widths...
            for ((_, col), val) in x.data.iter() {
                let n = val.chars().count();
                if w[col - 1] < n {
                    w[col - 1] = n;
                }
            }
            let mut sep = String::from("+");
            for idx in 0..x.columns.len() {
                let len = w[idx];
                let s = "-".repeat(len);
                sep.push_str(&s);
                sep.push('+');
            }

            writeln!(out, "{sep}").unwrap();
            // print head
            write!(out, "|").unwrap();
            for idx in 0..x.columns.len() {
                let hd = &x.columns[idx];
                let len = w[idx];
                write!(out, "{:1$}|", hd, len).unwrap();
            }
            writeln!(out).unwrap();
            writeln!(out, "{sep}").unwrap();
            for rownum in 1..x.row_count + 1 {
                write!(out, "|").unwrap();
                for idx in 0..x.columns.len() {
                    let k = (rownum, idx + 1);
                    let len = w[idx];
                    if let Some(v) = x.data.get(&k) {
                        write!(out, "{:1$}|", v, len).unwrap();
                    } else {
                        write!(out, "{:1$}|", "", len).unwrap();
                    }
                }
                writeln!(out).unwrap();
            }
            writeln!(out, "{sep}").unwrap();
        } else {
            todo!("what if the table cannot be locked.")
        }
    }

    pub fn columns(&self) -> Vec<String> {
        let mut r = vec![];
        if let Ok(x) = self.d.try_lock() {
            for c in x.columns.iter() {
                r.push(c.clone());
            }
        }
        r
    }

    // fn get_mut(&self, id: usize, idx: usize) -> Option<&mut String> {
    //     if let Ok(mut x) = self.d.try_lock() {
    //         let k = (id, idx);
    //         if !x.data.contains_key(&k)  {
    //             x.data.insert(k, String::new());

    //         }
    //         return x.data.get_mut(&k);
    //     } else {
    //         todo!("what if the table cannot be locked.")
    //     }
    // }
}
pub mod ser {
    use super::Table;
    use serde::{
        ser::{
            SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
            SerializeTupleStruct, SerializeTupleVariant,
        },
        Serialize,
    };
    use std::fmt::Error;
    use tracing::{instrument, trace};

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

    struct TableSer {
        t: Table,
        row: Option<usize>,
        col: Option<&'static str>,
        val: Option<String>,
    }

    impl<'a> TableSer {
        fn new() -> Self {
            Self {
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
            self.val = Some(v.into());
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
            let _ = key;
            todo!()
        }

        #[instrument(skip_all)]
        fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
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
}
