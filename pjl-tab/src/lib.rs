use memuse::DynamicUsage;
use pjl_static_strings::StringTable;
use serde::{
    de::Visitor,
    ser::{SerializeSeq, SerializeStruct},
};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Debug, Write},
    sync::Mutex,
};
use tracing::instrument;

pub mod de;
pub mod ser;
pub mod map; 
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

struct DeSerVisitor {
    // row: usize,
}

impl<'de> serde::Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // let visitor = DeSerVisitor { row: 1 };
        let visitor = DeSerVisitor {  };
        deserializer.deserialize_seq(visitor)
    }
}

impl<'de> Visitor<'de> for DeSerVisitor {
    type Value = Table;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        eprintln!("expecting...");
        write!(formatter, "expecting...")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let return_table = Table::new();
        while let Some(value) = seq
            .next_element::<BTreeMap<String, serde_json::Value>>()
            .unwrap()
        {
            let row = return_table.new_row();
            for (name, value) in value.iter() {
                let string_value = match value {
                    serde_json::Value::Null => None,
                    serde_json::Value::Bool(x) => Some(format!("{x}")),
                    serde_json::Value::Number(number) => Some(format!("{number}")),
                    serde_json::Value::String(s) => Some(s.clone()),
                    serde_json::Value::Array(_values) => todo!(),
                    serde_json::Value::Object(_map) => todo!(),
                };
                if let Some(v) = string_value {
                    row.set(name, &v);
                }
            }
        }
        Ok(return_table)
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
            // Hilfsfunktion: Prüft ob ein String eine Zahl ist
            let is_numeric = |s: &str| -> bool {
                s.parse::<f64>().is_ok()
            };

            // Erster Durchlauf: Berechne maximale Spaltenbreiten (Unicode-aware)
            // und erkenne ob eine Spalte numerisch ist
            let mut col_widths: Vec<usize> = x
                .columns
                .iter()
                .map(|hd| hd.chars().count())
                .collect();

            // Spalte ist numerisch, wenn ALLE nicht-leeren Werte Zahlen sind
            let mut col_is_numeric: Vec<bool> = vec![true; x.columns.len()];

            for rownum in 1..x.row_count + 1 {
                for (idx, width) in col_widths.iter_mut().enumerate() {
                    let k = (rownum, idx + 1);
                    if let Some(v) = x.data.get(&k) {
                        let char_count = v.chars().count();
                        if char_count > *width {
                            *width = char_count;
                        }
                        // Prüfe ob der Wert numerisch ist
                        if !v.is_empty() && !is_numeric(v) {
                            col_is_numeric[idx] = false;
                        }
                    }
                }
            }

            // Markdown table header
            write!(out, "|").unwrap();
            for (idx, hd) in x.columns.iter().enumerate() {
                let padding = col_widths[idx] - hd.chars().count();
                if col_is_numeric[idx] {
                    // Rechtsbündig für numerische Spalten
                    write!(out, " {}{} |", " ".repeat(padding), hd).unwrap();
                } else {
                    write!(out, " {}{} |", hd, " ".repeat(padding)).unwrap();
                }
            }
            writeln!(out).unwrap();

            // Markdown separator line mit Ausrichtungsmarkierung
            write!(out, "|").unwrap();
            for (idx, width) in col_widths.iter().enumerate() {
                if col_is_numeric[idx] {
                    // Rechtsbündig: --:
                    write!(out, "-{}:|", "-".repeat(*width)).unwrap();
                } else {
                    // Linksbündig (Standard)
                    write!(out, "-{}-|", "-".repeat(*width)).unwrap();
                }
            }
            writeln!(out).unwrap();

            // Data rows
            for rownum in 1..x.row_count + 1 {
                write!(out, "|").unwrap();
                for (idx, width) in col_widths.iter().enumerate() {
                    let k = (rownum, idx + 1);
                    if let Some(v) = x.data.get(&k) {
                        let padding = width - v.chars().count();
                        if col_is_numeric[idx] {
                            // Rechtsbündig
                            write!(out, " {}{} |", " ".repeat(padding), v).unwrap();
                        } else {
                            // Linksbündig
                            write!(out, " {}{} |", v, " ".repeat(padding)).unwrap();
                        }
                    } else {
                        write!(out, " {} |", " ".repeat(*width)).unwrap();
                    }
                }
                writeln!(out).unwrap();
            }
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

    pub fn column_count(&self) -> usize {
        if let Ok(x) = self.d.try_lock() {
            x.columns.len()
        } else {
            panic!("could not lock table")
        }
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
