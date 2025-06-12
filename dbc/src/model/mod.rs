use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    name: String,
    ftype: String,
    nullable: bool,
    key: bool,
}

impl Field {
    pub fn new(name: &str, ftype: &str) -> Self {
        let name = String::from(name);
        let ftype = String::from(ftype);
        Self {
            name,
            ftype,
            nullable: false,
            key: false,
        }
    }

    pub fn key(mut self) -> Self {
        self.key = true;
        self
    }

    pub fn nullable(mut self) -> Self {
        self.nullable = true;
        self
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }
    pub fn is_key(&self) -> bool {
        self.key
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ftype(&self) -> &str {
        &self.ftype
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    name: String,
    fields: Vec<Field>,
}

impl Table {
    pub fn new(name: &str) -> Self {
        let name = String::from(name);
        let fields = vec![];
        Self { name, fields }
    }

    pub fn add(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn get_field(&self, fname: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name() == fname)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    tables: Vec<Table>,
}

impl Schema {
    pub fn new() -> Self {
        let tables = vec![];
        Self { tables }
    }

    pub fn add(&mut self, t: Table) {
        self.tables.push(t);
    }

    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|t| t.name() == name)
    }
}

pub enum SchemaDiff {
    NewTable(String),
    NewField(String, String),
}

pub fn diff_schema(s1: &Schema, s2: &Schema) -> Vec<SchemaDiff> {
    // check for new tables
    // and new fields
    let mut r = vec![];
    for t2 in s2.tables() {
        match s1.tables().iter().find(|t| t.name() == t2.name()) {
            Some(t1) => {
                for f2 in t2.fields() {
                    match t1.fields().iter().find(|f| f.name() == f2.name()) {
                        Some(f1) => {
                            // ignore for now, fields might have different types or settings....
                        }
                        None => r.push(SchemaDiff::NewField(t2.name().into(), f2.name().into())),
                    }
                }
            }
            None => r.push(SchemaDiff::NewTable(t2.name().into())),
        }
    }
    r
}

pub fn read_schema(path: &str) -> Schema {
    let f = File::open(path).unwrap();
    serde_yaml::from_reader(f).unwrap()
}

pub fn write_schema(s: &Schema, path: &str) {
    let f = File::create(path).unwrap();
    serde_yaml::to_writer(f, s).unwrap();
}
