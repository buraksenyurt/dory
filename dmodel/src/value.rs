use std::fmt::{Display, Formatter};


/// Indicates the type of value in the Item object.
///
/// It is designed to work with light weight and low cost values.
/// For example, with ThinNumber, it is specified to keep an 8-bit integer.
/// Logical kept boolean values.
/// Text type that can carry large data is subject to length validation in the Item::new function.
///
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
