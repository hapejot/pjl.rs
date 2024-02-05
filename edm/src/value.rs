use crate::{
    list::ListValue, primitive::PrimitiveValue, structure::StructureValue,
};

#[derive(Debug, Clone)]
pub enum Value {
    PrimitiveValue(PrimitiveValue),
    StructureValue(StructureValue),
    ListValue(ListValue),
}

// impl From<&str> for Value {
//     fn from(value: &str) -> Self {
//         Value::PrimitiveValue(PrimitiveValue::from(value))
//     }
// }

impl From<StructureValue> for Value {
    fn from(value: StructureValue) -> Self {
        Value::StructureValue(value)
    }
}

impl From<ListValue> for Value {
    fn from(value: ListValue) -> Self {
        Value::ListValue(value)
    }
}

impl Value {
    pub fn null() -> Value {
        Value::PrimitiveValue(PrimitiveValue::Null)
    }
}

impl<T> From<T> for Value
where
    T: Into<PrimitiveValue>,
{
    fn from(value: T) -> Self {
        let v: PrimitiveValue = value.into();
        Self::PrimitiveValue(v.into())
    }
}
