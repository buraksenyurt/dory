use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Value {
    ThinNumber(i8),
    MidNumber(i16),
    LargeNumber(i32),
    ThinFloat(f32),
    LargeFloat(f64),
    Text(&'static str),
    Logical(bool)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
