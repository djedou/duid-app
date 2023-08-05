use crate::compiler::vm::{
    opcode::*,
    data::*,
    vm::DuidVm
};


impl<const N: usize> DuidVm<N> {
    pub fn op_slash(&mut self, data_type: DataType) {
        match data_type {
            DataType::None => {},
            DataType::Int8 => {
                match &mut self.pop(2) {
                    Some(data) => {
                        let rhs = i8::from_be_bytes(data[..1].try_into().unwrap());
                        let lhs = i8::from_be_bytes(data[1..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int8(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Int16 => {
                match &mut self.pop(4) {
                    Some(data) => {
                        let rhs = i16::from_be_bytes(data[..2].try_into().unwrap());
                        let lhs = i16::from_be_bytes(data[2..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int16(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Int32 => {
                match &mut self.pop(8) {
                    Some(data) => {
                        let rhs = i32::from_be_bytes(data[..4].try_into().unwrap());
                        let lhs = i32::from_be_bytes(data[4..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int32(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Int64 => {
                match &mut self.pop(16) {
                    Some(data) => {
                        let rhs = i64::from_be_bytes(data[..8].try_into().unwrap());
                        let lhs = i64::from_be_bytes(data[8..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int64(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Int128 => {
                match &mut self.pop(32) {
                    Some(data) => {
                        let rhs = i128::from_be_bytes(data[..16].try_into().unwrap());
                        let lhs = i128::from_be_bytes(data[16..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int128(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt8 => {
                match &mut self.pop(2) {
                    Some(data) => {
                        let rhs = u8::from_be_bytes(data[..1].try_into().unwrap());
                        let lhs = u8::from_be_bytes(data[1..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt8(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt16 => {
                match &mut self.pop(4) {
                    Some(data) => {
                        let rhs = u16::from_be_bytes(data[..2].try_into().unwrap());
                        let lhs = u16::from_be_bytes(data[2..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt16(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt32 => {
                match &mut self.pop(8) {
                    Some(data) => {
                        let rhs = u32::from_be_bytes(data[..4].try_into().unwrap());
                        let lhs = u32::from_be_bytes(data[4..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt32(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt64 => {
                match &mut self.pop(16) {
                    Some(data) => {
                        let rhs = u64::from_be_bytes(data[..8].try_into().unwrap());
                        let lhs = u64::from_be_bytes(data[8..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt64(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt128 => {
                match &mut self.pop(32) {
                    Some(data) => {
                        let rhs = u128::from_be_bytes(data[..16].try_into().unwrap());
                        let lhs = u128::from_be_bytes(data[16..].try_into().unwrap());
                        self.push(&(lhs / rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt128(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Float32 => {
                match &mut self.pop(8) {
                    Some(data) => {
                        let rhs = f32::from_bits(u32::from_be_bytes(data[..4].try_into().unwrap()));
                        let lhs = f32::from_bits(u32::from_be_bytes(data[4..].try_into().unwrap()));
                        self.push(&(lhs / rhs).to_bits().to_be_bytes());
                        self.push(&[u8::from(&DataValue::Float32(eq_float::F32(0.))), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Float64 => {
                match &mut self.pop(16) {
                    Some(data) => {
                        let rhs = f64::from_bits(u64::from_be_bytes(data[..8].try_into().unwrap()));
                        let lhs = f64::from_bits(u64::from_be_bytes(data[8..].try_into().unwrap()));
                        self.push(&(lhs / rhs).to_bits().to_be_bytes());
                        self.push(&[u8::from(&DataValue::Float64(eq_float::F64(0.))), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Variable => {

            },
        }
    }
}
