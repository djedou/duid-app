use crate::vm::{
    data::*,
    vm::DuidVm
};


impl<const N: usize> DuidVm<N> {
    pub fn op_ne(&mut self) {
        match self.pop_instructions(1) {
            Some(data) => {
                let data_type = DataType::from([data[0]].as_slice());
                let size = data_type.get_size();

                match data_type {
                    DataType::None => {},
                    DataType::Int8 => {
                        crate::OpCompBinary!(self, i8, size, !=);
                    },
                    DataType::Int16 => {
                        crate::OpCompBinary!(self, i16, size, !=);
                    },
                    DataType::Int32 => {
                        crate::OpCompBinary!(self, i32, size, !=);
                    },
                    DataType::Int64 => {
                        crate::OpCompBinary!(self, i64, size, !=);
                    },
                    DataType::Int128 => {
                        crate::OpCompBinary!(self, i128, size, !=);
                    },
                    DataType::UInt8 => {
                        crate::OpCompBinary!(self, u8, size, !=);
                    },
                    DataType::UInt16 => {
                        crate::OpCompBinary!(self, u16, size, !=);
                    },
                    DataType::UInt32 => {
                        crate::OpCompBinary!(self, u32, size, !=);
                    },
                    DataType::UInt64 => {
                        crate::OpCompBinary!(self, u64, size, !=);
                    },
                    DataType::UInt128 => {
                        crate::OpCompBinary!(self, u128, size, !=);
                    },
                    DataType::Float32 => {
                        crate::OpCompBinaryFloat!(self, f32, u32, size, !=);
                    },
                    DataType::Float64 => {
                        crate::OpCompBinaryFloat!(self, f64, u64, size, !=);
                    },
                    DataType::Byte => {},
                    DataType::Bool => {},
                    DataType::String => {},
                    DataType::Chr => {},
                    DataType::Variable => {}
                }
            },
            None => {}
        }
    }
}
