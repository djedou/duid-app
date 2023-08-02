use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Byte
}

/*
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Value::Expr {paren_open, expr, paren_close} => write!(f, "{:#?}{:#?}{:#?}", paren_open, expr, paren_close),
            Value::Byte(n) => write!(f, "{}", n),
            Value::IntI8(n) => write!(f, "{}", n),
            Value::IntI16(n) => write!(f, "{}", n),
            Value::IntI32(n) => write!(f, "{}", n),
            Value::IntI64(n) => write!(f, "{}", n),
            Value::IntI128(n) => write!(f, "{}", n),
            Value::UintU8(n) => write!(f, "{}", n),
            Value::UintU16(n) => write!(f, "{}", n),
            Value::UintU32(n) => write!(f, "{}", n),
            Value::UintU128(n) => write!(f, "{}", n),
            Value::Float32(n) => write!(f, "{}", n),
            Value::Float64(n) => write!(f, "{}", n),
            Value::Int(n) => write!(f, "{}", n),
            Value::Bool(n) => write!(f, "{}", n),
            Value::String(n) => write!(f, "{}", n),
            Value::Char(n) => write!(f, "{}", n)
        }
    }
}
*/