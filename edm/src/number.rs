#[derive(Debug,Clone)]
pub struct Number {
    val: String,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val.as_str())
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}


impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number {
            val: format!("{value}"),
        }
    }
}
