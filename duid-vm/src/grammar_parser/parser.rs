#![allow(clippy::upper_case_acronyms)]

use pest::{self, Parser, pratt_parser::PrattParser};
use crate::compiler::vm::data::{Data, DataType};
use crate::ast::{Module, Visibility, Declaration/*, Statement, Expression, , */};

// ANCHOR: parser
#[derive(pest_derive::Parser)]
#[grammar = "./src/grammar_parser/literals.pest"]
#[grammar = "./src/grammar_parser/items.pest"]
#[grammar = "./src/grammar_parser/names.pest"]
#[grammar = "./src/grammar_parser/expression.pest"]
#[grammar = "./src/grammar_parser/statement.pest"]
struct DuidParser;


pub fn parse(source: &str) -> std::result::Result<Module, pest::error::Error<Rule>> {
    let mut ast = Module::new();
    //let pratt = PrattParser::new();

    match DuidParser::parse(Rule::ModuleFile, source) {
        Ok(pairs) => {
            println!("pairs: {:#?}", pairs);
            for pair in pairs {
                match pair.as_rule() {
                    Rule::Item => {
                        //build_ast_from_declaration(pair.into_inner().next().unwrap(), &mut ast);
                    },
                    Rule::Expression => {

                    },
                    _ => {}
                }
            }
        },
        Err(e) => {
            println!("Error: {:#?}", e);
            return Ok(ast);
        }
    }
    
    println!("module loaded: {:#?}", ast);
    Ok(ast)
}
/*
fn build_ast_from_declaration(pair: pest::iterators::Pair<Rule>, module: &mut Module) {
    match pair.as_rule() {
        Rule::ModDecl => {
            for p in pair.clone().into_inner() {
                match p.as_rule() {
                    Rule::ModDeclVisPub => {
                        module.visible = Visibility::Public;
                    },
                    Rule::ModDeclVisPri => {
                        module.visible = Visibility::Privite;
                    },
                    Rule::ModuleNamespace => {
                        module.namespace = p.as_str().to_string();
                    },
                    res => {
                        println!("in module declaration: {:#?}", res);
                    }
                }
            }
        },
        Rule::TypeDecl => {
            for td in pair.clone().into_inner() {
                match td.as_rule() {
                    Rule::UnitStructTyDecl => {
                        let mut data = Data::default();

                        for utd in td.clone().into_inner() {
                            match utd.as_rule() {
                                Rule::StructVisPub => {
                                    data.visible = Visibility::Public;
                                },
                                Rule::StructVisPri => {
                                    data.visible = Visibility::Privite;
                                },
                                Rule::Type => {
                                    match utd.as_str() {
                                        "Int8" => {
                                            data.type_ = DataType::Int8;
                                        },
                                        value => {
                                            panic!("UnDeclare type: {}", value);
                                        } 
                                    }
                                },
                                res => {
                                    println!("in module declaration: {:#?}", res);
                                }
                            }
                        }

                        module.declarations.push(Declaration::TypeDecl{data});
                    },
                    res => {
                        println!("in module declaration: {:#?}", res);
                    }
                }
            }
        },
        res => {
            println!("rest: {:#?}", res);
        }
    }
}*/

#[cfg(test)]
mod parser_tests {
    use super::*;

    
    #[test]
    fn module_declaration() {
        assert!(parse("let var: Int = ").is_ok());
    }
    
}
