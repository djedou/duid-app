//use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Variable {
    pub value: String,
    pub data_type: DataType
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataValue {
    #[default]
    None,
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    Float32(eq_float::F32),
    Float64(eq_float::F64),
    Byte(u8),
    Bool(bool),
    String(String),
    Chr(char),
    Variable(Variable)
    //CustomType(String)
}


impl From<&DataValue> for u16 {
    fn from(value: &DataValue) -> u16 {
        match value {
            DataValue::None => 0x0000,
            DataValue::Int8(_) => 0x0001,
            DataValue::Int16(_) => 0x0002,
            DataValue::Int32(_) => 0x0003,
            DataValue::Int64(_) => 0x0004,
            DataValue::Int128(_) => 0x0005,
            DataValue::UInt8(_) => 0x0006,
            DataValue::UInt16(_) => 0x0007,
            DataValue::UInt32(_) => 0x0008,
            DataValue::UInt64(_) => 0x0009,
            DataValue::UInt128(_) => 0x000A,
            DataValue::Float32(_) => 0x000B,
            DataValue::Float64(_) => 0x000C,
            DataValue::Byte(_) => 0x000D,
            DataValue::Bool(_) => 0x000E,
            DataValue::String(_) => 0x000F,
            DataValue::Chr(_) => 0x0010,
            DataValue::Variable(_) => 0x0011,
        }
    }
}

impl From<&[u16]> for DataType {
    fn from(value: &[u16]) -> DataType {
        match value {
            [0x0000] => DataType::None,
            [0x0001] => DataType::Int8,
            [0x0002] => DataType::Int16,
            [0x0003] => DataType::Int32,
            [0x0004] => DataType::Int64,
            [0x0005] => DataType::Int128,
            [0x0006] => DataType::UInt8,
            [0x0007] => DataType::UInt16,
            [0x0008] => DataType::UInt32,
            [0x0009] => DataType::UInt64,
            [0x000A] => DataType::UInt128,
            [0x000B] => DataType::Float32,
            [0x000C] => DataType::Float64,
            [0x000D] => DataType::Byte,
            [0x000E] => DataType::Bool,
            [0x000F] => DataType::String,
            [0x0010] => DataType::Chr,
            [0x0011] => DataType::Variable,
            _ => DataType::None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataType {
    #[default]
    None,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Float32,
    Float64,
    Byte,
    Bool,
    String,
    Chr,
    Variable
    //CustomType(String)
}

impl From<&str> for DataType {
    fn from(value: &str) -> DataType {
        match value {
            "Int8" => DataType::Int8,
            "Int16" => DataType::Int16,
            "Int32" => DataType::Int32,
            "Int64" => DataType::Int64,
            "Int128" => DataType::Int128,
            "UInt8" => DataType::UInt8,
            "UInt16" => DataType::UInt16,
            "UInt32" => DataType::UInt32,
            "UInt64" => DataType::UInt64,
            "UInt128" => DataType::UInt128,
            "Float32" => DataType::Float32,
            "Float64" => DataType::Float64,
            "Byte" => DataType::Byte,
            "Bool" => DataType::Bool,
            "String" => DataType::String,
            "Chr" => DataType::Chr,
            _ => DataType::None
        }
    }
}

pub trait TDataType {
    fn get_size(&self) -> usize;
}


impl TDataType for DataType {
    fn get_size(&self) -> usize {
        match self {
            DataType::None => 0,
            DataType::Int8 => 1,
            DataType::Int16 => 2,
            DataType::Int32 => 4,
            DataType::Int64 => 8,
            DataType::Int128 => 16,
            DataType::UInt8 => 1,
            DataType::UInt16 => 2,
            DataType::UInt32 => 4,
            DataType::UInt64 => 8,
            DataType::UInt128 => 16,
            DataType::Float32 => 4,
            DataType::Float64 => 8,
            DataType::Byte => 1,
            DataType::Bool => 1,
            DataType::String => 100, // TODO
            DataType::Chr => 1,
            DataType::Variable => 100, // TODO
        }
    }
}