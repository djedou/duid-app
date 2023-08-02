pub mod bytecode;
pub mod opcode;
pub mod instruction;
pub mod data;

#[allow(clippy::module_inception)]
pub mod vm;

pub use crate::compiler::vm::{
    bytecode::Bytecode,
    opcode::{make_op, OpCode},
};
