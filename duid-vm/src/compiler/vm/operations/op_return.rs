use crate::compiler::vm::{
    opcode::*,
    data::*,
    vm::DuidVm
};


impl<const N: usize> DuidVm<N> {
    pub fn op_return(&mut self, data_type: DataType) {
        match data_type {
            DataType::None => {

            },
            DataType::Int8 => {
                match self.pop(1) {
                    Some(data) => {
                        let res = i8::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::Int16 => {
                match self.pop(2) {
                    Some(data) => {
                        let res = i16::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::Int32 => {
                match self.pop(4) {
                    Some(data) => {
                        let res = i32::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::Int64 => {
                match self.pop(8) {
                    Some(data) => {
                        let res = i64::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::Int128 => {
                match self.pop(16) {
                    Some(data) => {
                        let res = i128::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::UInt8 => {
                match self.pop(1) {
                    Some(data) => {
                        let res = u8::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::UInt16 => {
                match self.pop(2) {
                    Some(data) => {
                        let res = u16::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::UInt32 => {
                match self.pop(4) {
                    Some(data) => {
                        let res = u32::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::UInt64 => {
                match self.pop(8) {
                    Some(data) => {
                        let res = u64::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::UInt128 => {
                match self.pop(16) {
                    Some(data) => {
                        let res = u128::from_be_bytes(data.try_into().unwrap());
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::Float32 => {
                match self.pop(4) {
                    Some(data) => {
                        let res = f32::from_bits(u32::from_be_bytes(data.try_into().unwrap()));
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::Float64 => {
                match self.pop(8) {
                    Some(data) => {
                        let res = f64::from_bits(u64::from_be_bytes(data.try_into().unwrap()));
                        println!("result: {:?}", res);
                    },
                    _ => {}
                }
            },
            DataType::Variable => {

            },
        }
    }
}
