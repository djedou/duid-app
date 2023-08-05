use crate::compiler::vm::{
    opcode::*,
    data::*,
    vm::DuidVm
};


impl<const N: usize> DuidVm<N> {
    pub fn op_percent(&mut self, data_type: DataType) {
        match data_type {
            DataType::None => {},
            DataType::Int8 => {
                match &mut self.pop(2) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, i8, data, 1, DataValue::Int8(0), %);
                    },
                    _ => {}
                }
            },
            DataType::Int16 => {
                match &mut self.pop(4) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, i16, data, 2, DataValue::Int16(0), %);
                    },
                    _ => {}
                }
            },
            DataType::Int32 => {
                match &mut self.pop(8) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, i32, data, 4, DataValue::Int32(0), %);
                    },
                    _ => {}
                }
            },
            DataType::Int64 => {
                match &mut self.pop(16) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, i64, data, 8, DataValue::Int64(0), %);
                    },
                    _ => {}
                }
            },
            DataType::Int128 => {
                match &mut self.pop(32) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, i128, data, 16, DataValue::Int128(0), %);
                    },
                    _ => {}
                }
            },
            DataType::UInt8 => {
                match &mut self.pop(2) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, u8, data, 1, DataValue::UInt8(0), %);
                    },
                    _ => {}
                }
            },
            DataType::UInt16 => {
                match &mut self.pop(4) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, u16, data, 2, DataValue::UInt16(0), %);
                    },
                    _ => {}
                }
            },
            DataType::UInt32 => {
                match &mut self.pop(8) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, u32, data, 4, DataValue::UInt32(0), %);
                    },
                    _ => {}
                }
            },
            DataType::UInt64 => {
                match &mut self.pop(16) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, u64, data, 8, DataValue::UInt64(0), %);
                    },
                    _ => {}
                }
            },
            DataType::UInt128 => {
                match &mut self.pop(32) {
                    Some(data) => {
                        crate::OpArithOrLog!(self, u128, data, 16, DataValue::UInt128(0), %);
                    },
                    _ => {}
                }
            },
            DataType::Float32 => {
                match &mut self.pop(8) {
                    Some(data) => {
                        crate::OpArithOrLogFloat!(self, f32, u32, data, 4, DataValue::Float32(eq_float::F32(0.)), %);
                    },
                    _ => {}
                }
            },
            DataType::Float64 => {
                match &mut self.pop(16) {
                    Some(data) => {
                        crate::OpArithOrLogFloat!(self, f64, u64, data, 8, DataValue::Float64(eq_float::F64(0.)), %);
                    },
                    _ => {}
                }
            },
            DataType::Variable => {

            },
        }
    }
}
