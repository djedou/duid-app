use crate::compiler::vm::{make_op, OpCode};
use crate::{Compile, Ast};
use crate::ast::*;
use crate::vm::data::{DataValue, DataType};

pub type ModuleKey = String;

#[derive(Debug, Clone, PartialEq, Eq)]
// ANCHOR: bytecode
pub struct Bytecode {
    pub code: Vec<u8>,
    pub instructions: Vec<u16>
}
// ANCHOR_END: bytecode

impl Bytecode {
    fn new() -> Self {
        Self {
            code: Vec::new(),
            instructions: Vec::new()
        }
    }

    pub fn get_size(&self) -> u32 {
        self.code.len() as u32
    }
}

#[derive(Debug)]
pub struct Interpreter {
    bytecode: Bytecode,
}

impl Compile for Interpreter {
    type Output = Bytecode;

    fn from_ast(ast: Ast) -> Self::Output {
        //println!("Builded Ast: {:#?}", ast);
        let mut interpreter = Interpreter {
            bytecode: Bytecode::new(),
        };

        for content in ast.module.contents {
            match content {
                ModuleContent::None => {

                },
                ModuleContent::Item(_) => {

                },
                ModuleContent::Expr(Expression::WithoutBlock(expr)) => {
                    interpreter.add_without_block(expr);
                },
                _ => {

                }
            }
        }

        interpreter.bytecode
    }
}
// ANCHOR_END: bytecode_interpreter

impl Interpreter {
    fn add_without_block(&mut self, without_block: ExprWithoutBlck) {
        match without_block {
            ExprWithoutBlck::GroupedExpression => {

            },
            ExprWithoutBlck::ArrayExpression => {

            },
            ExprWithoutBlck::TupleExpression => {

            },
            ExprWithoutBlck::StructExpression => {

            },
            ExprWithoutBlck::ClosureExpression => {

            },
            ExprWithoutBlck::AsyncBlockExpression => {

            },
            ExprWithoutBlck::ContinueExpression => {

            },
            ExprWithoutBlck::BreakExpression => {

            },
            ExprWithoutBlck::ReturnExpression => {

            },
            ExprWithoutBlck::UnderscoreExpression => {

            },
            ExprWithoutBlck::OpExpr(OpExpr::ArithOrLogExpr(op_binary)) => {
                match op_binary.op {
                    BinaryOps::Arith(arith) => {
                        match arith {
                            ArithExpr::Minus => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        
                                        let rhs_address = self.bytecode.get_size();
                                        self.bytecode.code.extend_from_slice(&rhs.to_be_bytes());
                                        let rhs_ints = crate::utils::build_instruction_op_add_datatype(OpCode::PUSH, rhs_address, u16::from(&op_binary.lhs));
                                        self.bytecode.instructions.extend_from_slice(&rhs_ints);
                                        
                                        let lhs_address = self.bytecode.get_size();
                                        self.bytecode.code.extend_from_slice(&lhs.to_be_bytes());
                                        let lhs_ints = crate::utils::build_instruction_op_add_datatype(OpCode::PUSH, lhs_address, u16::from(&op_binary.lhs));
                                        self.bytecode.instructions.extend_from_slice(&lhs_ints);
                                        
                                        let op_add = crate::utils::build_instruction_op_datatype(OpCode::OpMinus, u16::from(&op_binary.lhs));
                                        self.bytecode.instructions.extend_from_slice(&op_add);
                                        
                                        // ### return
                                        let rt_address = self.bytecode.get_size();
                                        self.bytecode.code.extend_from_slice(&[0u8; 1]);
                                        let rt_ints = crate::utils::build_instruction_op_add_datatype(OpCode::OpReturn, rt_address, u16::from(&op_binary.lhs));
                                        self.bytecode.instructions.extend_from_slice(&rt_ints);
        
                                        // ### output
                                        let rt_ints = crate::utils::build_instruction_op_add_datatype(OpCode::OpOutput, rt_address, u16::from(&op_binary.lhs));
                                        self.bytecode.instructions.extend_from_slice(&rt_ints);
                                    },
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
                            },
                            _ => {
        
                            }
                        }
                    },
                    BinaryOps::Log(_) => {

                    },
                    BinaryOps::None => {}
                }
            },
            ExprWithoutBlck::IndexExpression => {

            },
            ExprWithoutBlck::AwaitExpression => {

            },
            ExprWithoutBlck::TupleIndexingExpression => {

            },
            ExprWithoutBlck::CallExpression => {

            },
            ExprWithoutBlck::MethodCallExpression => {

            },
            ExprWithoutBlck::FieldExpression => {

            },
            ExprWithoutBlck::RangeExpression => {

            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    /*use super::*;

    #[test]
    fn basics() {
        infix_template("+", OpCode::OpAdd);
        infix_template("-", OpCode::OpSub);
    }

    fn infix_template(infix_str: &str, op_code: OpCode) {
        let input = format!("1 {} 2;", infix_str);
        let bytecode = Interpreter::from_source(&input);

        let expected_instructions = vec![
            OpCode::OpConstant(0),
            OpCode::OpConstant(1),
            op_code,
            OpCode::OpPop,
        ]
        .into_iter()
        .flat_map(make_op)
        .collect();

        assert_eq!(
            Bytecode {
                instructions: expected_instructions,
                constants: vec![Node::Int(1), Node::Int(2)]
            },
            bytecode
        );
    }*/
}
