#[derive(Debug,Clone)]
pub struct Number {
    is_int: bool,
    val: String,
}

impl Number {
    pub fn as_i64(&self) -> i64
    {
        let r = self.val.parse::<i64>().unwrap();
        r
    }

    pub fn as_f64(&self) -> f64
    {
        let r = self.val.parse::<f64>().unwrap();
        r
    }

    pub fn is_int(&self) -> bool {
        self.is_int
    }

    pub fn as_str(&self) -> &str {
        &self.val
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
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
            is_int: true,
            val: format!("{value}"),
        }
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number {
            is_int: true,
            val: format!("{value}"),
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number {
            is_int: true,
            val: format!("{value}"),
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number {
            is_int: true,
            val: format!("{value}"),
        }
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number {
            is_int: true,
            val: format!("{value}"),
        }
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number {
            is_int: true,
            val: format!("{value}"),
        }
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number {
            is_int: true,
            val: format!("{value}"),
        }
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number {
            is_int: true,
            val: format!("{value}"),
        }
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number {
            is_int: false,
            val: format!("{value}"),
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number {
            is_int: false,
            val: format!("{value}"),
        }
    }
}
