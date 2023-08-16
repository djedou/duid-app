pub mod ast;
pub mod compiler;
pub mod vm;
pub mod grammar_parser;
pub mod utils;

pub use crate::ast::{Ast};
pub use crate::vm::vm::DuidVm;


use duid_core::{
    console::info
};



pub fn run_vm() {
    info!("Running from VM");
}

// ANCHOR: compile_trait
pub trait Compile {
    type Output;

    fn from_ast(ast: Ast) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        //println!("Compiling the source: {}", source);
        let ast: Ast = grammar_parser::parse(source).unwrap();
        //println!("{:#?}", ast);
        Self::from_ast(ast)
    }
}