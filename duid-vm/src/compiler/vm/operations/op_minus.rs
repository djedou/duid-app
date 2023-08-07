use crate::{
    compiler::vm::{
        opcode::*,
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_minus(&mut self) {
        match self.pop_instructions(1) {
            Some(data) => {
                let data_type = DataType::from([data[0]].as_slice());
                let size = data_type.get_size();

                match data_type {
                    DataType::None => {},
                    DataType::Int8 => {
                        match &mut self.pop(size * 2) {
                            Some(value) => {
                                let rhs = i8::from_be_bytes(value[..size].try_into().unwrap());
                                let lhs = i8::from_be_bytes(value[size..].try_into().unwrap());
                                self.push(&(lhs - rhs).to_be_bytes());
                            },
                            _ => {}
                        }
                    },
                    /*DataType::Int16 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArith!(self, i16, data, 2, DataValue::Int16(0), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::Int32 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArith!(self, i32, data, 4, DataValue::Int32(0), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::Int64 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArith!(self, i64, data, 8, DataValue::Int64(0), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::Int128 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArith!(self, i128, data, 16, DataValue::Int128(0), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::UInt8 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArith!(self, u8, data, 1, DataValue::UInt8(0), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::UInt16 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArith!(self, u16, data, 2, DataValue::UInt16(0), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::UInt32 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArith!(self, u32, data, 4, DataValue::UInt32(0), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::UInt64 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArith!(self, u64, data, 8, DataValue::UInt64(0), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::UInt128 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArith!(self, u128, data, 16, DataValue::UInt128(0), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::Float32 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArithFloat!(self, f32, u32, data, 4, DataValue::Float32(eq_float::F32(0.)), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::Float64 => {
                        match &mut self.pop(size) {
                            Some(data) => {
                                crate::OpArithFloat!(self, f64, u64, data, 8, DataValue::Float64(eq_float::F64(0.)), +);
                            },
                            _ => {}
                        }
                    },
                    DataType::Variable => {
        
                    },*/
                    _ => {}
                }
            },
            None => {}
        }
    }
}
