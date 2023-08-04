use crate::compiler::vm::{
    opcode::*,
    data::*
};

#[derive(Debug)]
pub struct DuidVm<const N: usize> {
    stack: [u8; N],
    stack_top: usize
}


impl<const N: usize> DuidVm<N> {
    pub fn new() -> Self {
        DuidVm {
            stack: [0u8; N],
            stack_top: 0
        }
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn push(&mut self, value: &[u8]) {
        let end = self.stack_top + value.len();
        self.stack[self.stack_top..end].iter_mut().zip(value.iter()).for_each(|(a, b)| {*a = *b;});
        self.stack_top = end;
    }

    pub fn pop(&mut self, size: usize) -> Option<&[u8]> {
        match self.stack_top >= size {
            true => {
                let start = self.stack_top - size;
                let end = self.stack_top - 1;
                let data = &self.stack[start..=end];
                self.stack_top = start;
                Some(data)
            },
            false => None
        }
    }

    pub fn run(&mut self) {
        // Get OpCode and DataType Code
        while let Some(instr) = self.pop(2) {
            match instr {
                [size, 0x05] => {
                    let value = *size;
                    let _ = self.op_return(DataType::from([value].as_slice()));
                },
                [size, 0x10] => {
                    let value = *size;
                    let _ = self.op_add(DataType::from([value].as_slice()));
                },
                _ => {}
            }
        }
    }

    fn op_add(&mut self, data_type: DataType) {
        match data_type {
            DataType::None => {},
            DataType::Int8 => {
                match &mut self.pop(2) {
                    Some(data) => {
                        let lhs = i8::from_be_bytes(data[..1].try_into().unwrap());
                        let rhs = i8::from_be_bytes(data[1..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int8(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Int16 => {
                match &mut self.pop(4) {
                    Some(data) => {
                        let lhs = i16::from_be_bytes(data[..2].try_into().unwrap());
                        let rhs = i16::from_be_bytes(data[2..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int16(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Int32 => {
                match &mut self.pop(8) {
                    Some(data) => {
                        let lhs = i32::from_be_bytes(data[..4].try_into().unwrap());
                        let rhs = i32::from_be_bytes(data[4..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int32(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Int64 => {
                match &mut self.pop(16) {
                    Some(data) => {
                        let lhs = i64::from_be_bytes(data[..8].try_into().unwrap());
                        let rhs = i64::from_be_bytes(data[8..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int64(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Int128 => {
                match &mut self.pop(32) {
                    Some(data) => {
                        let lhs = i128::from_be_bytes(data[..16].try_into().unwrap());
                        let rhs = i128::from_be_bytes(data[16..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::Int128(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt8 => {
                match &mut self.pop(2) {
                    Some(data) => {
                        let lhs = u8::from_be_bytes(data[..1].try_into().unwrap());
                        let rhs = u8::from_be_bytes(data[1..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt8(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt16 => {
                match &mut self.pop(4) {
                    Some(data) => {
                        let lhs = u16::from_be_bytes(data[..2].try_into().unwrap());
                        let rhs = u16::from_be_bytes(data[2..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt16(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt32 => {
                match &mut self.pop(8) {
                    Some(data) => {
                        let lhs = u32::from_be_bytes(data[..4].try_into().unwrap());
                        let rhs = u32::from_be_bytes(data[4..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt32(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt64 => {
                match &mut self.pop(16) {
                    Some(data) => {
                        let lhs = u64::from_be_bytes(data[..8].try_into().unwrap());
                        let rhs = u64::from_be_bytes(data[8..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt64(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::UInt128 => {
                match &mut self.pop(32) {
                    Some(data) => {
                        let lhs = u128::from_be_bytes(data[..16].try_into().unwrap());
                        let rhs = u128::from_be_bytes(data[16..].try_into().unwrap());
                        self.push(&(lhs + rhs).to_be_bytes());
                        self.push(&[u8::from(&DataValue::UInt128(0)), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Float32 => {
                match &mut self.pop(8) {
                    Some(data) => {
                        let lhs = f32::from_bits(u32::from_be_bytes(data[..4].try_into().unwrap()));
                        let rhs = f32::from_bits(u32::from_be_bytes(data[4..].try_into().unwrap()));
                        self.push(&(lhs + rhs).to_bits().to_be_bytes());
                        self.push(&[u8::from(&DataValue::Float32(eq_float::F32(0.))), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Float64 => {
                match &mut self.pop(16) {
                    Some(data) => {
                        let lhs = f64::from_bits(u64::from_be_bytes(data[..8].try_into().unwrap()));
                        let rhs = f64::from_bits(u64::from_be_bytes(data[8..].try_into().unwrap()));
                        self.push(&(lhs + rhs).to_bits().to_be_bytes());
                        self.push(&[u8::from(&DataValue::Float64(eq_float::F64(0.))), make_op(OpCode::OpReturn)]);
                    },
                    _ => {}
                }
            },
            DataType::Variable => {

            },
        }
    }


    fn op_return(&mut self, data_type: DataType) {
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
