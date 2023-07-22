use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// ANCHOR: operator
pub enum Operator {
    Plus,
    Minus,
}
// ANCHOR_END: operator

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
        }
    }
}