use crate::vm::data::DataValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogExpr {
    And,
    Or,
    Caret,
    Shl,
    Shr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithExpr {
    Plus,
    Minus,
    Star,
    Slash,
    Percent
}

impl From<&str> for LogExpr {
    fn from(value: &str) -> Self {
        match value {
            "&" => LogExpr::And,
            "|" => LogExpr::Or,
            "^" => LogExpr::Caret,
            "<<" => LogExpr::Shl,
            ">>" => LogExpr::Shr,
            o => panic!("Unknowed operator: {}", o)
        }
    }
}

impl From<&str> for ArithExpr {
    fn from(value: &str) -> Self {
        match value {
            "+" => ArithExpr::Plus,
            "-" => ArithExpr::Minus,
            "*" => ArithExpr::Star,
            "/" => ArithExpr::Slash,
            "%" => ArithExpr::Percent,
            o => panic!("Unknowed operator: {}", o)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOps {
    None,
    Arith(ArithExpr),
    Log(LogExpr)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpr {
    pub lhs: DataValue,
    pub op: BinaryOps,
    pub rhs: DataValue
}

impl BinaryExpr {
    pub fn new() -> Self {
        BinaryExpr {
            lhs: DataValue::None,
            op: BinaryOps::None,
            rhs: DataValue::None
        }
    }
}
