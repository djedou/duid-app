use crate::vm::data::DataValue;




#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyBoolExpr {
    None,
    OrOr,
    AndAnd
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonExpr {
    None,
    EqEq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le
}

impl From<&str> for ComparisonExpr {
    fn from(value: &str) -> Self {
        match value {
            "==" => ComparisonExpr::EqEq,
            "!=" => ComparisonExpr::Ne,
            ">" => ComparisonExpr::Gt,
            "<" => ComparisonExpr::Lt,
            ">=" => ComparisonExpr::Ge,
            "<=" => ComparisonExpr::Le,
            o => panic!("Unknowed operator: {}", o)
        }
    }
}


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
    Log(LogExpr),
    Comp(ComparisonExpr),
    LazyBool(LazyBoolExpr)
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
