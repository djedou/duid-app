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
    Variable(Variable)
    //CustomType(String)
}


impl From<&DataValue> for u8 {
    fn from(value: &DataValue) -> u8 {
        match value {
            DataValue::None => 0x00,
            DataValue::Int8(_) => 0x01,
            DataValue::Int16(_) => 0x02,
            DataValue::Int32(_) => 0x03,
            DataValue::Int64(_) => 0x04,
            DataValue::Int128(_) => 0x05,
            DataValue::UInt8(_) => 0x06,
            DataValue::UInt16(_) => 0x07,
            DataValue::UInt32(_) => 0x08,
            DataValue::UInt64(_) => 0x09,
            DataValue::UInt128(_) => 0x10,
            DataValue::Float32(_) => 0x11,
            DataValue::Float64(_) => 0x12,
            DataValue::Variable(_) => 0x13
        }
    }
}

impl From<&[u8]> for DataType {
    fn from(value: &[u8]) -> DataType {
        match value {
            [0x00] => DataType::None,
            [0x01] => DataType::Int8,
            [0x02] => DataType::Int16,
            [0x03] => DataType::Int32,
            [0x04] => DataType::Int64,
            [0x05] => DataType::Int128,
            [0x06] => DataType::UInt8,
            [0x07] => DataType::UInt16,
            [0x08] => DataType::UInt32,
            [0x09] => DataType::UInt64,
            [0x10] => DataType::UInt128,
            [0x11] => DataType::Float32,
            [0x12] => DataType::Float64,
            [0x13] => DataType::Variable,
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
            _ => DataType::None
        }
    }
}