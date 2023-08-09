pub mod opcode;
pub mod data;
pub mod operations;


pub use operations::*;

#[allow(clippy::module_inception)]
pub mod vm;
/*
pub use crate::compiler::vm::{
    bytecode::Bytecode,
    opcode::{make_op, OpCode},
};
*/