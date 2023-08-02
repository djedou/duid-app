use crate::ast::Visibility;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataType {
    #[default]
    None,
    //Module,
    Int8,
    //CustomType(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataValue {
    #[default]
    None,
    //Module(Vec<Data>),
    Int8(u8),
    //CustomTypeValue((String, String))
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Data {
    pub visible: Visibility,
    //pub value: DataValue,
    pub type_: DataType,
    pub instance_methods: BTreeMap<String, String>,
    pub static_methods: BTreeMap<String, String>
}