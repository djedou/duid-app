use crate::compiler::vm::opcode::*;


#[macro_export]
macro_rules! OpBinaryInstructions {
    ($stack:expr, $data_type:expr, $lhs:expr, $rhs:expr, $op:expr, $size:expr) => {
        let rhs_address = $stack.bytecode.get_size();
        $stack.bytecode.code.extend_from_slice(&$rhs);
        let rhs_ints = $crate::utils::build_instruction_op_add_datatype($crate::compiler::vm::opcode::OpCode::PUSH, rhs_address, u16::from(&$data_type));
        $stack.bytecode.instructions.extend_from_slice(&rhs_ints);
        
        let lhs_address = $stack.bytecode.get_size();
        $stack.bytecode.code.extend_from_slice(&$lhs);
        let lhs_ints = $crate::utils::build_instruction_op_add_datatype($crate::compiler::vm::opcode::OpCode::PUSH, lhs_address, u16::from(&$data_type));
        $stack.bytecode.instructions.extend_from_slice(&lhs_ints);
        
        let op_add = $crate::utils::build_instruction_op_datatype($op, u16::from(&$data_type));
        $stack.bytecode.instructions.extend_from_slice(&op_add);

        let rt_address = $stack.bytecode.get_size();
        $stack.bytecode.code.extend_from_slice(&[0u8; $size]);
        let rt_ints = $crate::utils::build_instruction_op_add_datatype($crate::compiler::vm::opcode::OpCode::OpReturn, rt_address, u16::from(&$data_type));
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);

        // To be remove: Just here for test purpose!
        let rt_ints = crate::utils::build_instruction_op_add_datatype($crate::compiler::vm::opcode::OpCode::OpOutput, rt_address, u16::from(&$data_type));
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);
    };
}


#[macro_export]
macro_rules! OpBinary {
    ($stack:expr, $data_type:ty, $size:expr, $op:tt) => {
        let index = $size * 2;
        match &mut $stack.pop(index) {
            Some(value) => {
                let rhs = <$data_type>::from_be_bytes(value[..$size].try_into().unwrap());
                let lhs = <$data_type>::from_be_bytes(value[$size..].try_into().unwrap());
                $stack.push(&(lhs $op rhs).to_be_bytes());
            },
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! OpBinaryFloat {
    ($stack:expr, $data_type:ty, $data_type_int:ty, $size:expr, $op:tt) => {
        let index = $size * 2;
        match &mut $stack.pop(index) {
            Some(value) => {
                let rhs = <$data_type>::from_bits(<$data_type_int>::from_be_bytes(value[..$size].try_into().unwrap()));
                let lhs = <$data_type>::from_bits(<$data_type_int>::from_be_bytes(value[$size..].try_into().unwrap()));
                $stack.push(&(lhs $op rhs).to_bits().to_be_bytes());
            },
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! OpUnaryInstructions {
    ($stack:expr, $data_type:expr, $rhs:expr, $op:expr, $size:expr) => {
        let rhs_address = $stack.bytecode.get_size();
        $stack.bytecode.code.extend_from_slice(&$rhs);
        let rhs_ints = $crate::utils::build_instruction_op_add_datatype($crate::compiler::vm::opcode::OpCode::PUSH, rhs_address, u16::from(&$data_type));
        $stack.bytecode.instructions.extend_from_slice(&rhs_ints);

        let op_add = $crate::utils::build_instruction_op_datatype($op, u16::from(&$data_type));
        $stack.bytecode.instructions.extend_from_slice(&op_add);

        let rt_address = $stack.bytecode.get_size();
        $stack.bytecode.code.extend_from_slice(&[0u8; $size]);
        let rt_ints = $crate::utils::build_instruction_op_add_datatype($crate::compiler::vm::opcode::OpCode::OpReturn, rt_address, u16::from(&$data_type));
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);

        // To be remove: Just here for test purpose!
        let rt_ints = crate::utils::build_instruction_op_add_datatype($crate::compiler::vm::opcode::OpCode::OpOutput, rt_address, u16::from(&$data_type));
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);
    };
}

#[macro_export]
macro_rules! OpUnary {
    ($stack:expr, $data_type:ty, $size:expr, $op:tt) => {
        match &mut $stack.pop($size) {
            Some(value) => {
                let rhs = <$data_type>::from_be_bytes(value[..$size].try_into().unwrap());
                $stack.push(&($op rhs).to_be_bytes());
            },
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! OpUnaryFloat {
    ($stack:expr, $data_type:ty, $data_type_int:ty, $size:expr, $op:tt) => {
        match &mut $stack.pop($size) {
            Some(value) => {
                let rhs = <$data_type>::from_bits(<$data_type_int>::from_be_bytes(value[..$size].try_into().unwrap()));
                $stack.push(&($op rhs).to_bits().to_be_bytes());
            },
            _ => {}
        }
    };
}

pub fn build_instruction_op_add_datatype(op: OpCode, add: u32, data_type: u16) -> [u16; 4] {
    [make_op(op), (add >> 16) as u16, add as u16, data_type]
}

pub fn build_instruction_op_datatype(op: OpCode, data_type: u16) -> [u16; 2] {
    [make_op(op), data_type]
}


pub fn get_address(index_one: u16, index_two: u16) -> usize {
    ((index_one << 8) | index_two) as usize
}

fn _convert_u16_to_two_u8s(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}

fn _convert_two_u8s_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

fn _make_three_byte_op(code: u8, data: u16) -> Vec<u8> {
    let mut output = vec![code];
    output.extend(&_convert_u16_to_two_u8s(data));
    output
}


pub fn boolean_into_bits(value: &bool) -> u8 {
    match value {
        true => 0xFF,
        false => 0x00
    }
}

pub fn boolean_from_bits(value: u8) -> bool {
    match value {
        0xFF => true,
        _ => false
    }
}