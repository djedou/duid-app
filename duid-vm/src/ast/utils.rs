


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mutability {
    Immutable,
    Mutable
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Visibility {
    #[default]
    Privite,
    Public
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Byte,
    IntI8,
    IntI16,
    IntI32,
    IntI64,
    IntI128,
    UintU8,
    UintU16,
    UintU32,
    UintU128,
    Float32,
    Float64,
    Int,
    Bool,
    String,
    Char,
    Custom
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Parenthesis {
    Open,
    Close
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bracket {
    Open,
    Close
}