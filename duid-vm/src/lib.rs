pub mod ast;
pub mod compiler;
pub mod grammar_parser;

pub use crate::ast::{Module};
//pub use crate::compiler::interpreter::Interpreter;
pub use crate::compiler::vm::{self, vm::DuidVm};

pub type Result<T> = anyhow::Result<T>;


use duid_core::{
    console::info
};



pub fn run_vm() {
    info!("Running from VM");
}

// ANCHOR: compile_trait
pub trait Compile {
    type Output;

    fn from_ast(ast: Module) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        //println!("Compiling the source: {}", source);
        let ast: Module = grammar_parser::parse(source).unwrap();
        //println!("{:?}", ast);
        Self::from_ast(ast)
    }
}