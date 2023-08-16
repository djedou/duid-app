use crate::vm::data::Data;
use super::Expression;


#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum UnaryOps {
    #[default]
    Plus,
    Minus,
    Not
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum UnaryData {
    #[default]
    None,
    Data(Data),
    Expr(Box<Expression>)
}

impl From<&str> for UnaryOps {
    fn from(value: &str) -> Self {
        match value {
            "-" => UnaryOps::Minus,
            "!" => UnaryOps::Not,
            _ => UnaryOps::Plus
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpr {
    pub op: UnaryOps,
    pub data: UnaryData
}

impl UnaryExpr {
    pub fn new() -> Self {
        UnaryExpr {
            op: UnaryOps::default(),
            data: UnaryData::default()
        }
    }
}
