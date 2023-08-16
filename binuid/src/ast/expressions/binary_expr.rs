use super::UnaryExpr;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOps {
    None,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    And,
    Or,
    AndAnd,
    OrOr,
    Shl,
    Shr,
    EqEq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le
}


impl From<&str> for BinaryOps {
    fn from(value: &str) -> Self {
        match value {
            "+" => BinaryOps::Plus,
            "-" => BinaryOps::Minus,
            "*" => BinaryOps::Star,
            "/" => BinaryOps::Slash,
            "%" => BinaryOps::Percent,
            "&" => BinaryOps::And,
            "|" => BinaryOps::Or,
            "^" => BinaryOps::Caret,
            "<<" => BinaryOps::Shl,
            ">>" => BinaryOps::Shr,
            "==" => BinaryOps::EqEq,
            "!=" => BinaryOps::Ne,
            ">" => BinaryOps::Gt,
            "<" => BinaryOps::Lt,
            ">=" => BinaryOps::Ge,
            "<=" => BinaryOps::Le,
            o => panic!("Unknowed operator: {}", o)
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpr {
    pub lhs: UnaryExpr,
    pub op: BinaryOps,
    pub rhs: UnaryExpr
}

impl BinaryExpr {
    pub fn new() -> Self {
        BinaryExpr {
            lhs: UnaryExpr::new(),
            op: BinaryOps::None,
            rhs: UnaryExpr::new()
        }
    }
}
