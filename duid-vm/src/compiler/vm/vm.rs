use crate::compiler::vm::{
    opcode::*,
    data::*
};

#[derive(Debug)]
pub struct DuidVm<const N: usize> {
    code: Vec<u8>,
    stack: [u8; N],
    stack_top: usize,
    pc: usize
}


impl<const N: usize> DuidVm<N> {
    pub fn new() -> Self {
        DuidVm {
            code: Vec::with_capacity(0),
            stack: [0u8; N],
            stack_top: 0,
            pc: 0
        }
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn load_code(&mut self, value: &[u8]) {
        self.code.extend_from_slice(value);
    }

    pub fn pop_code(&mut self) -> Option<&[u8]> {
        
        /*match self.stack_top >= size {
            true => {
                let start = self.stack_top - size;
                let end = self.stack_top - 1;
                let data = &self.stack[start..=end];
                self.stack_top = start;
                Some(data)
            },
            false => None
        }*/
        None
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
                [type_code, 0x05] => {
                    let value = *type_code;
                    let _ = self.op_return(DataType::from([value].as_slice()));
                },
                [type_code, 0x10] => {
                    let value = *type_code;
                    let _ = self.op_add(DataType::from([value].as_slice()));
                },
                [type_code, 0x11] => {
                    let value = *type_code;
                    let _ = self.op_minus(DataType::from([value].as_slice()));
                },
                [type_code, 0x12] => {
                    let value = *type_code;
                    let _ = self.op_star(DataType::from([value].as_slice()));
                },
                [type_code, 0x13] => {
                    let value = *type_code;
                    let _ = self.op_slash(DataType::from([value].as_slice()));
                },
                [type_code, 0x14] => {
                    let value = *type_code;
                    let _ = self.op_percent(DataType::from([value].as_slice()));
                },
                [type_code, 0x15] => {
                    let value = *type_code;
                    let _ = self.op_bit_and(DataType::from([value].as_slice()));
                },
                [type_code, 0x16] => {
                    let value = *type_code;
                    let _ = self.op_bit_or(DataType::from([value].as_slice()));
                },
                [type_code, 0x17] => {
                    let value = *type_code;
                    let _ = self.op_bit_xor(DataType::from([value].as_slice()));
                },
                _ => {}
            }
        }
    }
}
