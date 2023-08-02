use std::fmt;

/*
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
}*/



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignOps {
    Assign,
    PlusAssign,
    MinusAssign,
    TimesAssign,
    DivideAssign,
    ModulusAssign
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithOps {
    Plus,
    Minus,
    Times,
    Divide,
    Modulus
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignOps {
    Plus,
    Minus
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicOps {
    And,
    Or,
    Not
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitwiseOps {
    And,
    Or,
    Not,
    LeftShit,
    RightShift
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonOps {
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Lower,
    LowerEqual
}
