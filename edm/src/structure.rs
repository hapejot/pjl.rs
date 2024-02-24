use crate::value::Value;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops;

#[derive(Debug, Clone)]
pub struct StructureValue {
    pub datatype: String,
    pub values: BTreeMap<String, Value>,
}
impl StructureValue {
    pub fn new() -> Self {
        StructureValue {
            datatype: String::new(),
            values: BTreeMap::new(),
        }
    }

    pub fn new_with_type(datatype: &str) -> Self {
        StructureValue {
            datatype: String::from(datatype),
            values: BTreeMap::new(),
        }
    }

    pub fn keys(&self) -> std::collections::btree_map::Keys<'_, String, Value> {
        self.values.keys()
    }

    pub fn datatype(&self) -> &str {
        self.datatype.as_ref()
    }
}

impl ops::Index<&str> for StructureValue {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        self.values.get(index).unwrap()
    }
}

impl ops::IndexMut<&str> for StructureValue {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        if !self.values.contains_key(index) {
            self.values.insert(
                index.into(),
                Value::PrimitiveValue(crate::primitive::PrimitiveValue::Null),
            );
        }
        self.values.get_mut(index).unwrap()
    }
}


impl Display for StructureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}