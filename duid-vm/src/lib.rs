//mod hardware;
pub mod ast;
pub mod compiler;
pub mod grammar_parser;

pub use crate::ast::{Node, Operator};
pub use crate::compiler::interpreter::Interpreter;
pub use crate::compiler::vm::{self, vm::VM};

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

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        println!("Compiling the source: {}", source);
        let ast: Vec<Node> = grammar_parser::parse(source).unwrap();
        //println!("{:?}", ast);
        Self::from_ast(ast)
    }
}