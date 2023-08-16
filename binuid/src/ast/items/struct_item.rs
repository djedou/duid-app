
use crate::vm::data::DataType;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Struct {
    UnitStruct(DataType),
    StructStruct(String),
    TupleStruct(String)
}
