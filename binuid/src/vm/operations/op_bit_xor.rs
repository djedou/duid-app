use crate::{
    utils::{get_usize_from_u16s},
    vm::{
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_bit_xor(&mut self) {
        match self.pop_instructions(3) {
            Some(data) => {
                let data_type = DataType::from([data[0]].as_slice());
                let size = get_usize_from_u16s(data[1] as u16, data[2] as u16); 

                match data_type {
                    DataType::None => {},
                    DataType::ByteType => {crate::OpBinary!(self, i8, size, ^);},
                    DataType::BoolType => {crate::OpBinary!(self, i8, size, ^);},
                    DataType::CharType => {},
                    DataType::F32Type => {},
                    DataType::F64Type => {},
                    DataType::FnType => {},
                    DataType::I8Type => {crate::OpBinary!(self, i8, size, ^);},
                    DataType::I16Type => {crate::OpBinary!(self, i16, size, ^);},
                    DataType::I32Type => {crate::OpBinary!(self, i32, size, ^);},
                    DataType::I64Type => {crate::OpBinary!(self, i64, size, ^);},
                    DataType::I128Type => {crate::OpBinary!(self, i128, size, ^);},
                    DataType::U8Type => {crate::OpBinary!(self, u8, size, ^);},
                    DataType::U16Type => {crate::OpBinary!(self, u16, size, ^);},
                    DataType::U32Type => {crate::OpBinary!(self, u32, size, ^);},
                    DataType::U64Type => {crate::OpBinary!(self, u64, size, ^);},
                    DataType::U128Type => {crate::OpBinary!(self, u128, size, ^);},
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
