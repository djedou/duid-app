use crate::vm::opcode::*;
use std::num::ParseIntError;


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
macro_rules! OpCompBinary {
    ($stack:expr, $data_type:ty, $size:expr, $op:tt) => {
        let index = $size * 2;
        match &mut $stack.pop(index) {
            Some(value) => {
                let rhs = <$data_type>::from_be_bytes(value[..$size].try_into().unwrap());
                let lhs = <$data_type>::from_be_bytes(value[$size..].try_into().unwrap());
                let res = $crate::utils::boolean_into_bits(&(lhs $op rhs));
                $stack.push(&res.to_be_bytes());
            },
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! OpLazyBoolBinary {
    ($stack:expr, $data_type:ty, $size:expr, $op:tt) => {
        let index = $size * 2;
        match &mut $stack.pop(index) {
            Some(value) => {
                let rhs = $crate::utils::boolean_from_bits(<$data_type>::from_be_bytes(value[..$size].try_into().unwrap()));
                let lhs = $crate::utils::boolean_from_bits(<$data_type>::from_be_bytes(value[$size..].try_into().unwrap()));
                let res = $crate::utils::boolean_into_bits(&(lhs $op rhs));
                $stack.push(&res.to_be_bytes());
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
macro_rules! OpCompBinaryFloat {
    ($stack:expr, $data_type:ty, $data_type_int:ty, $size:expr, $op:tt) => {
        let index = $size * 2;
        match &mut $stack.pop(index) {
            Some(value) => {
                let rhs = <$data_type>::from_bits(<$data_type_int>::from_be_bytes(value[..$size].try_into().unwrap()));
                let lhs = <$data_type>::from_bits(<$data_type_int>::from_be_bytes(value[$size..].try_into().unwrap()));
                let res = $crate::utils::boolean_into_bits(&(lhs $op rhs));
                $stack.push(&res.to_be_bytes());
            },
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! OpUnaryInstructions {
    ($stack:expr, $data_type:expr, $rhs:expr, $op:expr, $size:expr) => {
        let rhs_address = $stack.bytecode.get_address();
        $stack.bytecode.code.extend_from_slice(&$rhs);
        let rhs_ints = $crate::utils::build_instruction_opcode_address_datasize(
            $crate::vm::opcode::OpCode::PUSH, 
            rhs_address,
            $size
        );
        $stack.bytecode.instructions.extend_from_slice(&rhs_ints);
        // #############
        let op_add = $crate::utils::build_instruction_opcode_datatype(
            $op, 
            u16::from(&$data_type),
            $size
        );
        $stack.bytecode.instructions.extend_from_slice(&op_add);
        // #######
        let rt_address = $stack.bytecode.get_address();
        let sizes: Vec<_> = (0..$size).into_iter().map(|_| 0u8).collect();
        $stack.bytecode.code.extend_from_slice(&sizes);
        let rt_ints = $crate::utils::build_instruction_opcode_address_datasize(
            $crate::vm::opcode::OpCode::OpReturn, 
            rt_address,
            $size
        );
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);
        $stack.bytecode.set_return_address(rt_address);

        // To be remove: Just here for test purpose!
        /*let rt_ints = $crate::utils::build_instruction_opcode_address_datatype_datasize(
            $crate::vm::opcode::OpCode::OpOutput, 
            rt_address, 
            u16::from(&$data_type),
            $size
        );
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);*/
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

#[macro_export]
macro_rules! UnaryBytecodes {
    ($stack:expr, $unary_expr:expr) => {
        match $unary_expr.op {
            $crate::ast::UnaryOps::Plus => {
                match &$unary_expr.data {
                    $crate::ast::UnaryData::Data(data) => {
                        $crate::OpUnaryInstructions!($stack, data.type_, data.value.into_bytes(), OpCode::OpUAnd, data.size);
                    },
                    $crate::ast::UnaryData::Expr(ex) => {
                        $stack.add_expression(&*ex);  
                    },
                    _ => {}
                }
            },
            $crate::ast::UnaryOps::Minus => {
                match &$unary_expr.data {
                    $crate::ast::UnaryData::Data(data) => {
                        $crate::OpUnaryInstructions!($stack, data.type_, data.value.into_bytes(), OpCode::OpUMinus, data.size);
                    },
                    $crate::ast::UnaryData::Expr(expr) => {
                        $stack.add_expression(&*expr);
                    },
                    _ => {}
                }
            },
            $crate::ast::UnaryOps::Not => {
                match &$unary_expr.data {
                    $crate::ast::UnaryData::Data(data) => {
                        $crate::OpUnaryInstructions!($stack, data.type_, data.value.into_bytes(), OpCode::OpUNot, data.size);
                    },
                    $crate::ast::UnaryData::Expr(expr) => {
                        $stack.add_expression(&*expr);  
                    },
                    _ => {}
                }
            }
        }
    };
}

#[macro_export]
macro_rules! OpBinaryBytecodes {
    ($stack:expr, $op:expr, $binary_ops:expr) => {
        let $crate::compiler::bytecode::UnaryData::Data(rhs_data) = &$binary_ops.rhs.data else {
            panic!("Data should have a size at compile time!");
        };

        let $crate::compiler::bytecode::UnaryData::Data(lhs_data) = &$binary_ops.lhs.data else {
            panic!("Data should have a size at compile time!");
        };

        $crate::UnaryBytecodes!($stack, $binary_ops.lhs);
        let lhs_return_address = $stack.bytecode.get_return_address();
        $crate::UnaryBytecodes!($stack, $binary_ops.rhs);
        let rhs_return_address = $stack.bytecode.get_return_address();

        
        let rhs_ints = $crate::utils::build_instruction_opcode_address_datasize(
            $crate::vm::opcode::OpCode::PUSH, 
            rhs_return_address,
            rhs_data.size
        );
        $stack.bytecode.instructions.extend_from_slice(&rhs_ints);
        
        let lhs_ints = $crate::utils::build_instruction_opcode_address_datasize(
            $crate::vm::opcode::OpCode::PUSH, 
            lhs_return_address,
            lhs_data.size
        );
        $stack.bytecode.instructions.extend_from_slice(&lhs_ints);

        // #############
        let op_add = $crate::utils::build_instruction_opcode_datatype(
            $op, 
            u16::from(&lhs_data.type_),
            lhs_data.size
        );
        $stack.bytecode.instructions.extend_from_slice(&op_add);

        // #######
        let rt_address = $stack.bytecode.get_address();
        let sizes: Vec<_> = (0..lhs_data.size).into_iter().map(|_| 0u8).collect();
        $stack.bytecode.code.extend_from_slice(&sizes);
        let rt_ints = $crate::utils::build_instruction_opcode_address_datasize(
            $crate::vm::opcode::OpCode::OpReturn, 
            rt_address,
            lhs_data.size
        );
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);
        $stack.bytecode.set_return_address(rt_address);

        // To be remove: Just here for test purpose!
        let rt_ints = $crate::utils::build_instruction_opcode_address_datatype_datasize(
            $crate::vm::opcode::OpCode::OpOutput, 
            rt_address, 
            u16::from(&lhs_data.type_),
            lhs_data.size
        );
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);
    };
}


#[macro_export]
macro_rules! OpCompBinaryBytecodes {
    ($stack:expr, $op:expr, $binary_ops:expr) => {
        let $crate::compiler::bytecode::UnaryData::Data(rhs_data) = &$binary_ops.rhs.data else {
            panic!("Data should have a size at compile time!");
        };

        let $crate::compiler::bytecode::UnaryData::Data(lhs_data) = &$binary_ops.lhs.data else {
            panic!("Data should have a size at compile time!");
        };

        $crate::UnaryBytecodes!($stack, $binary_ops.lhs);
        let lhs_return_address = $stack.bytecode.get_return_address();
        $crate::UnaryBytecodes!($stack, $binary_ops.rhs);
        let rhs_return_address = $stack.bytecode.get_return_address();

        
        let rhs_ints = $crate::utils::build_instruction_opcode_address_datasize(
            $crate::vm::opcode::OpCode::PUSH, 
            rhs_return_address,
            rhs_data.size
        );
        $stack.bytecode.instructions.extend_from_slice(&rhs_ints);
        
        let lhs_ints = $crate::utils::build_instruction_opcode_address_datasize(
            $crate::vm::opcode::OpCode::PUSH, 
            lhs_return_address,
            lhs_data.size
        );
        $stack.bytecode.instructions.extend_from_slice(&lhs_ints);

        // #############
        let op_add = $crate::utils::build_instruction_opcode_datatype(
            $op, 
            u16::from(&lhs_data.type_),
            lhs_data.size
        );
        $stack.bytecode.instructions.extend_from_slice(&op_add);

        // #######
        let rt_address = $stack.bytecode.get_address();
        $stack.bytecode.code.extend_from_slice(&[0u8; 1]);
        let rt_ints = $crate::utils::build_instruction_opcode_address_datasize(
            $crate::vm::opcode::OpCode::OpReturn, 
            rt_address,
            1
        );
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);
        $stack.bytecode.set_return_address(rt_address);

        // To be remove: Just here for test purpose!
        let rt_ints = $crate::utils::build_instruction_opcode_address_datatype_datasize(
            $crate::vm::opcode::OpCode::OpOutput, 
            rt_address, 
            u16::from(&$crate::vm::data::DataType::BoolType),
            1
        );
        $stack.bytecode.instructions.extend_from_slice(&rt_ints);
    };
}


pub fn build_instruction_opcode_address_datatype_datasize(op: OpCode, address: u32, data_type: u16, data_size: u32) -> [u16; 6] {
    [make_op(op), (address >> 16) as u16, address as u16, data_type, (data_size >> 16) as u16, data_size as u16]
}

pub fn build_instruction_opcode_address_datasize(op: OpCode, address: u32, data_size: u32) -> [u16; 5] {
    [make_op(op), (address >> 16) as u16, address as u16, (data_size >> 16) as u16, data_size as u16]
}

pub fn build_instruction_opcode_datatype(op: OpCode, data_type: u16, data_size: u32) -> [u16; 4] {
    [make_op(op), data_type, (data_size >> 16) as u16, data_size as u16]
}

pub fn get_usize_from_u16s(index_one: u16, index_two: u16) -> usize {
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


pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (1..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}