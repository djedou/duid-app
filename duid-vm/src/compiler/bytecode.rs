use crate::vm::opcode::OpCode;
use crate::{Compile, Ast};
use crate::ast::*;
use crate::vm::data::DataValue;
use crate::utils::{boolean_into_bits};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bytecode {
    pub code: Vec<u8>,
    pub instructions: Vec<u16>
}

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

impl fmt::Display for Bytecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?} {:?}", self.code, self.instructions)
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
            ExprWithoutBlck::OpExpr(OpExpr::NegationExpr(op_unary)) => {
                match op_unary.op {
                    UnaryOps::Minus => {
                        match op_unary.rhs {
                            DataValue::Int8(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUMinus, 1);
                            },
                            DataValue::Int16(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUMinus, 2);
                            },
                            DataValue::Int32(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUMinus, 4);
                            },
                            DataValue::Int64(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUMinus, 8);
                            },
                            DataValue::Int128(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUMinus, 16);
                            },
                            DataValue::UInt8(_) => {
                                panic!("cannot apply unary operator `-` to type `UInt8`");
                            },
                            DataValue::UInt16(_) => {
                                panic!("cannot apply unary operator `-` to type `UInt16`");
                            },
                            DataValue::UInt32(_) => {
                                panic!("cannot apply unary operator `-` to type `UInt32`");
                            },
                            DataValue::UInt64(_) => {
                                panic!("cannot apply unary operator `-` to type `UInt64`");
                            },
                            DataValue::UInt128(_) => {
                                panic!("cannot apply unary operator `-` to type `UInt128`");
                            },
                            DataValue::Float32(eq_float::F32(rhs)) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUMinus, 4);
                            },
                            DataValue::Float64(eq_float::F64(rhs)) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUMinus, 8);
                            },
                            DataValue::Byte(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUMinus, 1);
                            },
                            DataValue::Bool(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, boolean_into_bits(&rhs).to_be_bytes(), OpCode::OpUMinus, 1);
                            },
                            DataValue::String(_) => {},
                            DataValue::Chr(_) => {},
                            DataValue::Variable(_) => {},
                            DataValue::None => {}
                        }
                    },
                    UnaryOps::Not => {
                        match op_unary.rhs {
                            DataValue::Int8(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 1);
                            },
                            DataValue::Int16(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 2);
                            },
                            DataValue::Int32(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 4);
                            },
                            DataValue::Int64(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 8);
                            },
                            DataValue::Int128(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 16);
                            },
                            DataValue::UInt8(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 1);
                            },
                            DataValue::UInt16(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 2);
                            },
                            DataValue::UInt32(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 4);
                            },
                            DataValue::UInt64(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 8);
                            },
                            DataValue::UInt128(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 16);
                            },
                            DataValue::Float32(eq_float::F32(rhs)) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_bits().to_be_bytes(), OpCode::OpUNot, 4);
                            },
                            DataValue::Float64(eq_float::F64(rhs)) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_bits().to_be_bytes(), OpCode::OpUNot, 8);
                            },
                            DataValue::Byte(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, rhs.to_be_bytes(), OpCode::OpUNot, 1);
                            },
                            DataValue::Bool(rhs) => {
                                crate::OpUnaryInstructions!(self, op_unary.rhs, boolean_into_bits(&rhs).to_be_bytes(), OpCode::OpUNot, 1);
                            },
                            DataValue::String(_) => {},
                            DataValue::Chr(_) => {},
                            DataValue::Variable(_) => {}
                            DataValue::None => {}
                        }
                    },
                    UnaryOps::None => {}
                }
            },
            ExprWithoutBlck::OpExpr(OpExpr::ArithOrLogExpr(op_binary)) => {
                match op_binary.op {
                    BinaryOps::Arith(arith) => {
                        match arith {
                            ArithExpr::Plus => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpAdd, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpAdd, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpAdd, 8);
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
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpMinus, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpMinus, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpMinus, 8);
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
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpStar, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpStar, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpStar, 8);
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
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpSlash, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpSlash, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpSlash, 8);
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
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpPercent, 16);
                                    },
                                    (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpPercent, 4);
                                    },
                                    (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_bits().to_be_bytes(), rhs.to_bits().to_be_bytes(), OpCode::OpPercent, 8);
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
                    BinaryOps::Log(logic) => {
                        match logic {
                            LogExpr::And => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 16);
                                    },
                                    (DataValue::Float32(_), DataValue::Float32(_)) => {},
                                    (DataValue::Float64(_), DataValue::Float64(_)) => {},
                                    (DataValue::Byte(lhs), DataValue::Byte(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitAnd, 1);
                                    },
                                    (DataValue::Bool(lhs), DataValue::Bool(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, boolean_into_bits(lhs).to_be_bytes(), boolean_into_bits(&rhs).to_be_bytes(), OpCode::OpBitAnd, 1);
                                    },
                                    (DataValue::String(_), DataValue::String(_)) => {},
                                    (DataValue::Chr(_), DataValue::Chr(_)) => {},
                                    (DataValue::Variable(_), DataValue::Variable(_)) => {},
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
                            },
                            LogExpr::Or => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 16);
                                    },
                                    (DataValue::Float32(_), DataValue::Float32(_)) => {},
                                    (DataValue::Float64(_), DataValue::Float64(_)) => {},
                                    (DataValue::Byte(lhs), DataValue::Byte(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitOr, 1);
                                    },
                                    (DataValue::Bool(lhs), DataValue::Bool(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, boolean_into_bits(lhs).to_be_bytes(), boolean_into_bits(&rhs).to_be_bytes(), OpCode::OpBitOr, 1);
                                    },
                                    (DataValue::String(_), DataValue::String(_)) => {},
                                    (DataValue::Chr(_), DataValue::Chr(_)) => {},
                                    (DataValue::Variable(_), DataValue::Variable(_)) => {},
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
                            },
                            LogExpr::Caret => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 16);
                                    },
                                    (DataValue::Float32(_), DataValue::Float32(_)) => {},
                                    (DataValue::Float64(_), DataValue::Float64(_)) => {},
                                    (DataValue::Byte(lhs), DataValue::Byte(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpBitXor, 1);
                                    },
                                    (DataValue::Bool(lhs), DataValue::Bool(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, boolean_into_bits(lhs).to_be_bytes(), boolean_into_bits(&rhs).to_be_bytes(), OpCode::OpBitXor, 1);
                                    },
                                    (DataValue::String(_), DataValue::String(_)) => {},
                                    (DataValue::Chr(_), DataValue::Chr(_)) => {},
                                    (DataValue::Variable(_), DataValue::Variable(_)) => {},
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
                            },
                            LogExpr::Shl => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 16);
                                    },
                                    (DataValue::Float32(_), DataValue::Float32(_)) => {},
                                    (DataValue::Float64(_), DataValue::Float64(_)) => {},
                                    (DataValue::Byte(lhs), DataValue::Byte(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShl, 1);
                                    },
                                    (DataValue::Bool(lhs), DataValue::Bool(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, boolean_into_bits(lhs).to_be_bytes(), boolean_into_bits(&rhs).to_be_bytes(), OpCode::OpShl, 1);
                                    },
                                    (DataValue::String(_), DataValue::String(_)) => {},
                                    (DataValue::Chr(_), DataValue::Chr(_)) => {},
                                    (DataValue::Variable(_), DataValue::Variable(_)) => {},
                                    (_, _) => {
                                        println!("lhs and rhs should have the same Datatype!");
                                    }
                                }
                            },
                            LogExpr::Shr => {
                                match (&op_binary.lhs, op_binary.rhs) {
                                    (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 1);
                                    },
                                    (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 2);
                                    },
                                    (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 4);
                                    },
                                    (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 8);
                                    },
                                    (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 16);
                                    },
                                    (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 1);
                                    },
                                    (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 2);
                                    },
                                    (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 4);
                                    },
                                    (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 8);
                                    },
                                    (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 16);
                                    },
                                    (DataValue::Float32(_), DataValue::Float32(_)) => {},
                                    (DataValue::Float64(_), DataValue::Float64(_)) => {},
                                    (DataValue::Byte(lhs), DataValue::Byte(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, lhs.to_be_bytes(), rhs.to_be_bytes(), OpCode::OpShr, 1);
                                    },
                                    (DataValue::Bool(lhs), DataValue::Bool(rhs)) => {
                                        crate::OpBinaryInstructions!(self, op_binary.lhs, boolean_into_bits(&lhs).to_be_bytes(), boolean_into_bits(&rhs).to_be_bytes(), OpCode::OpShr, 1);
                                    },
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
