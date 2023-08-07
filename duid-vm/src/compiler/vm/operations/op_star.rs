use crate::{
    compiler::vm::{
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_star(&mut self) {
        match self.pop_instructions(1) {
            Some(data) => {
                let data_type = DataType::from([data[0]].as_slice());
                let size = data_type.get_size();

                match data_type {
                    DataType::None => {},
                    DataType::Int8 => {
                        crate::OpArith!(self, i8, size, *);
                    },
                    DataType::Int16 => {
                        crate::OpArith!(self, i16, size, *);
                    },
                    DataType::Int32 => {
                        crate::OpArith!(self, i32, size, *);
                    },
                    DataType::Int64 => {
                        crate::OpArith!(self, i64, size, *);
                    },
                    DataType::Int128 => {
                        crate::OpArith!(self, i128, size, *);
                    },
                    DataType::UInt8 => {
                        crate::OpArith!(self, u8, size, *);
                    },
                    DataType::UInt16 => {
                        crate::OpArith!(self, u16, size, *);
                    },
                    DataType::UInt32 => {
                        crate::OpArith!(self, u32, size, *);
                    },
                    DataType::UInt64 => {
                        crate::OpArith!(self, u64, size, *);
                    },
                    DataType::UInt128 => {
                        crate::OpArith!(self, u128, size, *);
                    },
                    DataType::Float32 => {
                        crate::OpArithFloat!(self, f32, u32, size, *);
                    },
                    DataType::Float64 => {
                        crate::OpArithFloat!(self, f64, u64, size, *);
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
