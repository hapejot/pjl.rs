use crate::value::Value;

#[derive(Debug, Clone)]
pub struct ListValue {
    pub values: Vec<Value>,
}

impl ListValue {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value)
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Value> {
        self.values.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
