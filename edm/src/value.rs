use std::fmt::Display;

use crate::{list::ListValue, primitive::PrimitiveValue, structure::StructureValue};

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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::PrimitiveValue(v) => match v {
                PrimitiveValue::Null => write!(f, "Null"),
                PrimitiveValue::Boolean(x) => write!(f, "{}", x),
                PrimitiveValue::Decimal(x) => write!(f, "{}", x),
                PrimitiveValue::String(x) => write!(f, "{}", x),
                PrimitiveValue::Custom { datatype, value } => write!(f, "{}({})", datatype, value),
            },
            Value::StructureValue(_) => todo!(),
            Value::ListValue(_) => todo!(),
        }
    }
}

impl serde::ser::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        use serde::ser::SerializeStruct;

        match self {
            Value::PrimitiveValue(v) => match v {
                PrimitiveValue::Null => serializer.serialize_none(),
                PrimitiveValue::Boolean(_) => todo!(),
                PrimitiveValue::Decimal(x) => {
                    if x.is_int() {
                        serializer.serialize_i64(x.as_i64())
                    } else {
                        serializer.serialize_f64(x.as_f64())
                    }
                }
                PrimitiveValue::String(x) => serializer.serialize_str(x),
                PrimitiveValue::Custom { .. } => todo!(),
            },
            Value::StructureValue(v) => {
                let name = v.datatype.clone();
                let name = name.leak();
                let mut s = serializer.serialize_struct(name, v.values.len())?;
                for (k, vv) in v.values.iter() {
                    let key = k.clone().leak();
                    s.serialize_field(key, vv)?;
                }

                s.end()
            }
            Value::ListValue(v) => {
                let mut s = serializer.serialize_seq(Some(v.len()))?;
                for x in v.iter() {
                    s.serialize_element(x)?;
                }
                s.end()
            }
        }
    }
}
