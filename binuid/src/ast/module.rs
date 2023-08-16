use std::fmt;
use crate::ast::{Visibility, Item, Expression};
use crate::vm::data::DataType;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleHeader {
    pub visible: Visibility,
    pub namespace: Option<String>,
    pub exported: Vec<DataType>
}

impl ModuleHeader {
    pub fn new() -> Self {
        ModuleHeader {
            visible: Visibility::Privite,
            namespace: None,
            exported: Vec::with_capacity(0)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleContent {
    None,
    Item(Item),
    Expr(Expression)
}

impl fmt::Display for ModuleContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            ModuleContent::None => write!(f, "None"),
            ModuleContent::Item(item) => write!(f, "{:#?}", item),
            ModuleContent::Expr(expr) => write!(f, "{:#?}", expr)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub header: ModuleHeader,
    pub contents: Vec<ModuleContent>
}

impl Module {
    pub fn new() -> Self {
        Module {
            header: ModuleHeader::new(),
            contents: Vec::with_capacity(0)
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:#?} {:#?}", &self.header, &self.contents)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub module: Module,
}

impl Ast {
    pub fn extend_module_content(&mut self, contents: &[ModuleContent]) { 
        self.module.contents.extend_from_slice(contents);
    }
}
