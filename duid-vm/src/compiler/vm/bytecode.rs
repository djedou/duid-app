use crate::compiler::vm::{OpCode};
use crate::{Compile, Ast};
use crate::ast::*;
use crate::vm::data::DataValue;
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
        let mut interpreter = Interpreter {
            bytecode: Bytecode::new(),
        };

        for content in ast.module.contents {
            match content {
                ModuleContent::None => {},
                ModuleContent::Item(_) => {},
                ModuleContent::Expr(Expression::WithoutBlock(expr)) => {
                    interpreter.add_without_block(expr);
                },
                _ => {}
            }
        }

        interpreter.bytecode
    }
}

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
                            ArithExpr::Plus => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpAdd, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpAdd, 8);
                                    },
                                    (DataValue::Byte(_), DataValue::Byte(_)) => {},
                                    (DataValue::Bool(_), DataValue::Bool(_)) => {},
                                    (DataValue::String(_), DataValue::String(_)) => {},
                                    (DataValue::Chr(_), DataValue::Chr(_)) => {},
                                    (DataValue::Variable(_), DataValue::Variable(_)) => {},
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
                            },
                            ArithExpr::Minus => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpMinus, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpMinus, 8);
                                    },
                                    (DataValue::Byte(_), DataValue::Byte(_)) => {},
                                    (DataValue::Bool(_), DataValue::Bool(_)) => {},
                                    (DataValue::String(_), DataValue::String(_)) => {},
                                    (DataValue::Chr(_), DataValue::Chr(_)) => {},
                                    (DataValue::Variable(_), DataValue::Variable(_)) => {},
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
                            },
                            ArithExpr::Star => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpStar, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpStar, 8);
                                    },
                                    (DataValue::Byte(_), DataValue::Byte(_)) => {},
                                    (DataValue::Bool(_), DataValue::Bool(_)) => {},
                                    (DataValue::String(_), DataValue::String(_)) => {},
                                    (DataValue::Chr(_), DataValue::Chr(_)) => {},
                                    (DataValue::Variable(_), DataValue::Variable(_)) => {},
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
                            },
                            ArithExpr::Slash => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpSlash, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpSlash, 8);
                                    },
                                    (DataValue::Byte(_), DataValue::Byte(_)) => {},
                                    (DataValue::Bool(_), DataValue::Bool(_)) => {},
                                    (DataValue::String(_), DataValue::String(_)) => {},
                                    (DataValue::Chr(_), DataValue::Chr(_)) => {},
                                    (DataValue::Variable(_), DataValue::Variable(_)) => {},
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
                            },
                            ArithExpr::Percent => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpPercent, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpArithIntoInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpPercent, 8);
                                    },
                                    (DataValue::Byte(_), DataValue::Byte(_)) => {},
                                    (DataValue::Bool(_), DataValue::Bool(_)) => {},
                                    (DataValue::String(_), DataValue::String(_)) => {},
                                    (DataValue::Chr(_), DataValue::Chr(_)) => {},
                                    (DataValue::Variable(_), DataValue::Variable(_)) => {},
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
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
