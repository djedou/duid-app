use crate::{
    compiler::vm::{
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_uminus(&mut self) {
        match self.pop_instructions(1) {
            Some(data) => {
                let data_type = DataType::from([data[0]].as_slice());
                let size = data_type.get_size();

                match data_type {
                    DataType::None => {},
                    DataType::Int8 => {
                        crate::OpUnary!(self, i8, size, -);
                    },
                    DataType::Int16 => {
                        crate::OpUnary!(self, i16, size, -);
                    },
                    DataType::Int32 => {
                        crate::OpUnary!(self, i32, size, -);
                    },
                    DataType::Int64 => {
                        crate::OpUnary!(self, i64, size, -);
                    },
                    DataType::Int128 => {
                        crate::OpUnary!(self, i128, size, -);
                    },
                    DataType::UInt8 => {},
                    DataType::UInt16 => {},
                    DataType::UInt32 => {},
                    DataType::UInt64 => {},
                    DataType::UInt128 => {},
                    DataType::Float32 => {
                        crate::OpUnaryFloat!(self, f32, u32, size, -);
                    },
                    DataType::Float64 => {
                        crate::OpUnaryFloat!(self, f64, u64, size, -);
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
