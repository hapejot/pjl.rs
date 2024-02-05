use crate::number::Number;


#[derive(Debug,Clone)]
pub enum PrimitiveValue {
    Null,
    Boolean(bool),
    Decimal(Number),
    String(String),
    Custom { datatype: String, value: String },
}

impl std::fmt::Display for PrimitiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveValue::Null => write!(f, "null"),
            PrimitiveValue::Boolean(v) => write!(f, "{v}"),
            PrimitiveValue::Decimal(v) => write!(f, "{v}"),
            PrimitiveValue::String(v) => write!(f, "{v}"),
            PrimitiveValue::Custom { datatype, value } => write!(f, "{value}:{datatype}"),
        }
    }
}

impl<T> From<T> for PrimitiveValue
where
    T: Into<Number>,
{
    fn from(value: T) -> Self {
        PrimitiveValue::Decimal(value.into())
    }
}

impl From<bool> for PrimitiveValue {
    fn from(value: bool) -> Self {
        PrimitiveValue::Boolean(value)
    }
}

impl From<&str> for PrimitiveValue {
    fn from(value: &str) -> Self {
        PrimitiveValue::String(value.into())
    }
}

impl PartialEq for PrimitiveValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Decimal(l0), Self::Decimal(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl PartialEq<PrimitiveValue> for &str {
    fn eq(&self, other: &PrimitiveValue) -> bool {
        format!("{}", other).as_str() == *self
    }
}
