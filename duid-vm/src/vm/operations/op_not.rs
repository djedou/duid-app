use crate::vm::{
    data::*,
    vm::DuidVm
};


impl<const N: usize> DuidVm<N> {
    pub fn op_not(&mut self) {
        match self.pop_instructions(1) {
            Some(data) => {
                let data_type = DataType::from([data[0]].as_slice());
                let size = data_type.get_size();

                match data_type {
                    DataType::None => {},
                    DataType::Int8 => {
                        crate::OpUnary!(self, i8, size, !);
                    },
                    DataType::Int16 => {
                        crate::OpUnary!(self, i16, size, !);
                    },
                    DataType::Int32 => {
                        crate::OpUnary!(self, i32, size, !);
                    },
                    DataType::Int64 => {
                        crate::OpUnary!(self, i64, size, !);
                    },
                    DataType::Int128 => {
                        crate::OpUnary!(self, i128, size, !);
                    },
                    DataType::UInt8 => {
                        crate::OpUnary!(self, u8, size, !);
                    },
                    DataType::UInt16 => {
                        crate::OpUnary!(self, u16, size, !);
                    },
                    DataType::UInt32 => {
                        crate::OpUnary!(self, u32, size, !);
                    },
                    DataType::UInt64 => {
                        crate::OpUnary!(self, u64, size, !);
                    },
                    DataType::UInt128 => {
                        crate::OpUnary!(self, u128, size, !);
                    },
                    DataType::Float32 => {},
                    DataType::Float64 => {},
                    DataType::Byte => {
                        crate::OpUnary!(self, u8, size, !);
                    },
                    DataType::Bool => {
                        crate::OpUnary!(self, u8, size, !);
                    },
                    DataType::String => {},
                    DataType::Chr => {},
                    DataType::Variable => {}
                }
            },
            None => {}
        }
    }
}
