use memuse::DynamicUsage;
use std::{
    collections::HashMap,
    fmt::Write,
    ops::{Index, IndexMut},
    sync::Mutex,
};

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
        if let Ok(x) = self.d.try_lock() {
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
