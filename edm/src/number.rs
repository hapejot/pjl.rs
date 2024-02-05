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
