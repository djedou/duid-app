
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataType {
    #[default]
    None,
    ByteType,
    BoolType,
    CharType,
    F32Type,
    F64Type,
    FnType, // it as always an address
    I8Type,
    I16Type,
    I32Type,
    I64Type,
    I128Type,
    U8Type,
    U16Type,
    U32Type,
    U64Type,
    U128Type,
    RefType,
    StringType,
    UnitType,
    TupleType,
    ArrayType,
    VecType, // it as always an address
    VariableType, // it as always an address
    StructStructType, // it as always an address
    TupleStructType, // it as always an address
    UnitStructType, // it as always an address
    EnumerationType, // it as always an address
}

impl From<&DataType> for u16 {
    fn from(value: &DataType) -> u16 {
        match value {
            DataType::None => 0x0000,
            DataType::ByteType => 0x0001,
            DataType::BoolType => 0x0002,
            DataType::CharType => 0x0003,
            DataType::F32Type => 0x0004,
            DataType::F64Type => 0x0005,
            DataType::FnType => 0x0006,
            DataType::I8Type => 0x0007,
            DataType::I16Type => 0x0008,
            DataType::I32Type => 0x0009,
            DataType::I64Type => 0x000A,
            DataType::I128Type => 0x000B,
            DataType::U8Type => 0x000C,
            DataType::U16Type => 0x000D,
            DataType::U32Type => 0x000E,
            DataType::U64Type => 0x000F,
            DataType::U128Type => 0x0010,
            DataType::RefType => 0x0011,
            DataType::StringType => 0x0012,
            DataType::UnitType => 0x0013,
            DataType::TupleType => 0x0014,
            DataType::ArrayType => 0x0015,
            DataType::VecType => 0x0016,
            DataType::VariableType => 0x0017,
            DataType::StructStructType => 0x0018,
            DataType::TupleStructType => 0x0019,
            DataType::UnitStructType => 0x001A,
            DataType::EnumerationType => 0x001B,
        }
    }
}

impl From<&[u16]> for DataType {
    fn from(value: &[u16]) -> DataType {
        match value {
            [0x0000] => DataType::None,
            [0x0001] => DataType::ByteType,
            [0x0002] => DataType::BoolType,
            [0x0003] => DataType::CharType,
            [0x0004] => DataType::F32Type,
            [0x0005] => DataType::F64Type,
            [0x0006] => DataType::FnType,
            [0x0007] => DataType::I8Type,
            [0x0008] => DataType::I16Type,
            [0x0009] => DataType::I32Type,
            [0x000A] => DataType::I64Type,
            [0x000B] => DataType::I128Type,
            [0x000C] => DataType::U8Type,
            [0x000D] => DataType::U16Type,
            [0x000E] => DataType::U32Type,
            [0x000F] => DataType::U64Type,
            [0x0010] => DataType::U128Type,
            [0x0011] => DataType::RefType,
            [0x0012] => DataType::StringType,
            [0x0013] => DataType::UnitType,
            [0x0014] => DataType::TupleType,
            [0x0015] => DataType::ArrayType,
            [0x0016] => DataType::VecType,
            [0x0017] => DataType::VariableType,
            [0x0018] => DataType::StructStructType,
            [0x0019] => DataType::TupleStructType,
            [0x001A] => DataType::UnitStructType,
            [0x001B] => DataType::EnumerationType,
            _ => DataType::None
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataValue {
    #[default]
    None,
    ByteValue(u8),
    BoolValue(u8),
    CharValue(u32),
    F32Value(eq_float::F32),
    F64Value(eq_float::F64),
    FnValue(Vec<u8>), // it as always an address
    I8Value(i8),
    I16Value(i16),
    I32Value(i32),
    I64Value(i64),
    I128Value(i128),
    U8Value(u8),
    U16Value(u16),
    U32Value(u32),
    U64Value(u64),
    U128Value(u128),
    RefValue(u32),
    StringValue(Vec<u8>),
    UnitValue(u8),
    TupleValue(Vec<u8>),
    ArrayValue(Vec<u8>),
    VecValue(Vec<u8>), // it as always an address
    VariableValue(Vec<u8>), // it as always an address
    StructStructValue(Vec<u8>), // it as always an address
    TupleStructValue(Vec<u8>), // it as always an address
    UnitStructValue(Vec<u8>), // it as always an address
    EnumerationValue(Vec<u8>), // it as always an address
}

impl DataValue {
    pub fn into_bytes(&self) -> Vec<u8> {
        match self {
            DataValue::None => vec![],
            DataValue::ByteValue(v) => vec![*v],
            DataValue::BoolValue(v) => vec![*v],
            DataValue::CharValue(v) => v.to_be_bytes().to_vec(),
            DataValue::F32Value(eq_float::F32(v)) => v.to_bits().to_be_bytes().to_vec(),
            DataValue::F64Value(eq_float::F64(v)) => v.to_bits().to_be_bytes().to_vec(),
            DataValue::FnValue(_) => vec![], 
            DataValue::I8Value(v) => v.to_be_bytes().to_vec(),
            DataValue::I16Value(v) => v.to_be_bytes().to_vec(),
            DataValue::I32Value(v) => v.to_be_bytes().to_vec(),
            DataValue::I64Value(v) => v.to_be_bytes().to_vec(),
            DataValue::I128Value(v) => v.to_be_bytes().to_vec(),
            DataValue::U8Value(v) => vec![*v],
            DataValue::U16Value(v) => v.to_be_bytes().to_vec(),
            DataValue::U32Value(v) => v.to_be_bytes().to_vec(),
            DataValue::U64Value(v) => v.to_be_bytes().to_vec(),
            DataValue::U128Value(v) => v.to_be_bytes().to_vec(),
            DataValue::RefValue(v) => v.to_be_bytes().to_vec(),
            DataValue::StringValue(v) => v.to_owned(),
            DataValue::UnitValue(v) => vec![*v],
            DataValue::TupleValue(_) => vec![],
            DataValue::ArrayValue(_) => vec![],
            DataValue::VecValue(_) => vec![], 
            DataValue::VariableValue(v) => v.to_owned(), 
            DataValue::StructStructValue(_) => vec![], 
            DataValue::TupleStructValue(_) => vec![], 
            DataValue::UnitStructValue(_) => vec![], 
            DataValue::EnumerationValue(_) => vec![]
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Data {
    pub value: DataValue,
    pub type_: DataType,
    pub size: u32
}

impl Data {
    pub fn new() -> Data {
        Data {
            value: DataValue::None,
            type_: DataType::None,
            size: 0
        }
    }
}