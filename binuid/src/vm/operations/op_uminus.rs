use crate::{
    utils::{get_usize_from_u16s},
    vm::{
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_uminus(&mut self) {
        match self.pop_instructions(3) {
            Some(data) => {
                let data_type = DataType::from([data[0]].as_slice());
                let size = get_usize_from_u16s(data[1] as u16, data[2] as u16);  
                match data_type {
                    DataType::None => {},
                    DataType::ByteType => {},
                    DataType::BoolType => {},
                    DataType::CharType => {},
                    DataType::F32Type => {crate::OpUnaryFloat!(self, f32, u32, size, -);},
                    DataType::F64Type => {crate::OpUnaryFloat!(self, f64, u64, size, -);},
                    DataType::FnType => {},
                    DataType::I8Type => {crate::OpUnary!(self, i8, size, -);},
                    DataType::I16Type => {crate::OpUnary!(self, i16, size, -);},
                    DataType::I32Type => {crate::OpUnary!(self, i32, size, -);},
                    DataType::I64Type => {crate::OpUnary!(self, i64, size, -);},
                    DataType::I128Type => {crate::OpUnary!(self, i128, size, -);},
                    DataType::U8Type => {},
                    DataType::U16Type => {},
                    DataType::U32Type => {},
                    DataType::U64Type => {},
                    DataType::U128Type => {},
                    DataType::RefType => {},
                    DataType::StringType => {},
                    DataType::UnitType => {},
                    DataType::TupleType => {},
                    DataType::ArrayType => {},
                    DataType::VecType => {},
                    DataType::VariableType => {},
                    DataType::StructStructType => {},
                    DataType::TupleStructType => {},
                    DataType::UnitStructType => {},
                    DataType::EnumerationType => {}
                }
            },
            None => {}
        }
    }
}
