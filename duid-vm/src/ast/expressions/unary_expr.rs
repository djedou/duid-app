use crate::vm::data::DataValue;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOps {
    None,
    Minus,
    Not
}

impl From<&str> for UnaryOps {
    fn from(value: &str) -> Self {
        match value {
            "-" => UnaryOps::Minus,
            "!" => UnaryOps::Not,
            o => panic!("Unknowed operator: {}", o)
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpr {
    pub op: UnaryOps,
    pub rhs: DataValue
}

impl UnaryExpr {
    pub fn new() -> Self {
        UnaryExpr {
            op: UnaryOps::None,
            rhs: DataValue::None
        }
    }
}
