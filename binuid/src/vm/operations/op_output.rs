use crate::{
    utils::{get_usize_from_u16s, boolean_from_bits},
    vm::{
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_output(&mut self) {
        match self.pop_instructions(5) {
            Some(data) => {
                let address = get_usize_from_u16s(data[0] as u16, data[1] as u16);
                let data_type = DataType::from([data[2]].as_slice());
                let size = get_usize_from_u16s(data[3] as u16, data[4] as u16);
                match self.pop_memory(address, size) {
                    Some(m_data) => {
                        match data_type {
                            DataType::None => {},
                            DataType::I8Type => {
                                let res = i8::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::I16Type => {
                                let res = i16::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::I32Type => {
                                let res = i32::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::I64Type => {
                                let res = i64::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::I128Type => {
                                let res = i128::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::U8Type => {
                                let res = u8::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::U16Type => {
                                let res = u16::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::U32Type => {
                                let res = u32::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::U64Type => {
                                let res = u64::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::U128Type => {
                                let res = u128::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::F32Type => {
                                let res = f32::from_bits(u32::from_be_bytes(m_data.try_into().unwrap()));
                                println!("{:?}", res);
                            },
                            DataType::F64Type => {
                                let res = f64::from_bits(u64::from_be_bytes(m_data.try_into().unwrap()));
                                println!("{:?}", res);
                            },
                            DataType::ByteType => {
                                let res = u8::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::BoolType => {
                                let res = boolean_from_bits(u8::from_be_bytes(m_data.try_into().unwrap()));
                                println!("{:?}", res);
                            },
                            n => {
                                println!("Output is not yet implemented for {:?}", n);
                            },
                        }
                    },
                    None => {}
                }
            },
            None => {}
        }
    }
}
