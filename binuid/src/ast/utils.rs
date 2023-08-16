
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