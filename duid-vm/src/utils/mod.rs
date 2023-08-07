use crate::compiler::vm::opcode::*;

/*
#[macro_export]
macro_rules! OpArith {
    ($stack:expr, $data_type:ty, $data:expr, $size:expr, $value:expr, $op:tt ) => {
        let rhs = <$data_type>::from_be_bytes($data[..$size].try_into().unwrap());
        let lhs = <$data_type>::from_be_bytes($data[$size..].try_into().unwrap());
        $stack.push(&(lhs $op rhs).to_be_bytes());
        $stack.push(&[u8::from(&$value), make_op($crate::compiler::vm::opcode::OpCode::OpReturn)]);
    };
}

#[macro_export]
macro_rules! OpArithFloat {
    ($stack:expr, $data_type:ty, $data_type_int:ty, $data:expr, $size:literal, $value:expr, $op:tt ) => {
        let rhs = <$data_type>::from_bits(<$data_type_int>::from_be_bytes($data[..$size].try_into().unwrap()));
        let lhs = <$data_type>::from_bits(<$data_type_int>::from_be_bytes($data[$size..].try_into().unwrap()));
        $stack.push(&(lhs $op rhs).to_bits().to_be_bytes());
        $stack.push(&[u8::from(&$value), make_op($crate::compiler::vm::opcode::OpCode::OpReturn)]);
    };
}

#[macro_export]
macro_rules! ByteCodeFromDataValue {
    ($stack:expr, $rhs:expr, $lhs:expr, $data_type:expr, $op:expr ) => {
        $stack.bytecode.code.extend_from_slice(&$rhs);
        $stack.bytecode.code.extend_from_slice(&$lhs);
        $stack.bytecode.code.extend_from_slice(&[u8::from(&$data_type), make_op($op)]);
    }
}
*/
pub fn build_instruction_op_add_datatype(op: OpCode, add: u32, data_type: u16) -> [u16; 4] {
    [make_op(op), (add >> 16) as u16, add as u16, data_type]
}

pub fn build_instruction_op_datatype(op: OpCode, data_type: u16) -> [u16; 2] {
    [make_op(op), data_type]
}


pub fn get_address(index_one: u16, index_two: u16) -> usize {
    ((index_one << 8) | index_two) as usize
}

fn convert_u16_to_two_u8s(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}

pub fn convert_two_u8s_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

fn make_three_byte_op(code: u8, data: u16) -> Vec<u8> {
    let mut output = vec![code];
    output.extend(&convert_u16_to_two_u8s(data));
    output
}
