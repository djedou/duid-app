use crate::{
    utils::{get_address, boolean_from_bits},
    compiler::vm::{
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_output(&mut self) {
        match self.pop_instructions(3) {
            Some(data) => {
                let address = get_address(data[0] as u16, data[1] as u16);
                let data_type = DataType::from([data[2]].as_slice());
                let size = data_type.get_size();
                match self.pop_memory(address, size) {
                    Some(m_data) => {
                        match data_type {
                            DataType::None => {

                            },
                            DataType::Int8 => {
                                let res = i8::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::Int16 => {
                                let res = i16::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::Int32 => {
                                let res = i32::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::Int64 => {
                                let res = i64::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::Int128 => {
                                let res = i128::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::UInt8 => {
                                let res = u8::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::UInt16 => {
                                let res = u16::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::UInt32 => {
                                let res = u32::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::UInt64 => {
                                let res = u64::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::UInt128 => {
                                let res = u128::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::Float32 => {
                                let res = f32::from_bits(u32::from_be_bytes(m_data.try_into().unwrap()));
                                println!("{:?}", res);
                            },
                            DataType::Float64 => {
                                let res = f64::from_bits(u64::from_be_bytes(m_data.try_into().unwrap()));
                                println!("{:?}", res);
                            },
                            DataType::Byte => {
                                let res = u8::from_be_bytes(m_data.try_into().unwrap());
                                println!("{:?}", res);
                            },
                            DataType::Bool => {
                                let res = boolean_from_bits(u8::from_be_bytes(m_data.try_into().unwrap()));
                                println!("{:?}", res);
                            },
                            DataType::String => {

                            },
                            DataType::Chr => {

                            },
                            DataType::Variable => {

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
