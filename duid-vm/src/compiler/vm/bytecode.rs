use crate::compiler::vm::{make_op, OpCode};
use crate::{Compile, Ast};
use crate::ast::*;
use crate::vm::data::{DataValue, DataType};

pub type ModuleKey = String;

#[derive(Debug, Clone, PartialEq, Eq)]
// ANCHOR: bytecode
pub struct Bytecode {
    pub code: Vec<u8>
}
// ANCHOR_END: bytecode

impl Bytecode {
    fn new() -> Self {
        Self {
            code: Vec::new()
        }
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
            ExprWithoutBlck::OpExpr(OpExpr::ArithOrLogExpr(op_expr)) => {
                match op_expr.op {
                    ArithOrLogExpr::Plus => {
                        match (&op_expr.lhs, op_expr.rhs) {
                            (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpAdd);
                            },
                            (_, _) => {
                                println!("lhs and rhs should have the same Datatype!");
                            }
                        }
                    },
                    ArithOrLogExpr::Minus => {
                        match (&op_expr.lhs, op_expr.rhs) {
                            (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpMinus);
                            },
                            (_, _) => {
                                println!("lhs and rhs should have the same Datatype!");
                            }
                        }
                    },
                    ArithOrLogExpr::Star => {
                        match (&op_expr.lhs, op_expr.rhs) {
                            (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpStar);
                            },
                            (_, _) => {
                                println!("lhs and rhs should have the same Datatype!");
                            }
                        }
                    },
                    ArithOrLogExpr::Slash => {
                        match (&op_expr.lhs, op_expr.rhs) {
                            (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpSlash);
                            },
                            (_, _) => {
                                println!("lhs and rhs should have the same Datatype!");
                            }
                        }
                    },
                    ArithOrLogExpr::Percent => {
                        match (&op_expr.lhs, op_expr.rhs) {
                            (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::Float32(eq_float::F32(lhs)), DataValue::Float32(eq_float::F32(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (DataValue::Float64(eq_float::F64(lhs)), DataValue::Float64(eq_float::F64(rhs))) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_bits().to_be_bytes(), lhs.to_bits().to_be_bytes(), op_expr.lhs, OpCode::OpPercent);
                            },
                            (_, _) => {
                                println!("lhs and rhs should have the same Datatype!");
                            }
                        }
                    },
                    ArithOrLogExpr::And => {
                        match (&op_expr.lhs, op_expr.rhs) {
                            (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitAnd);
                            },
                            (DataValue::Float32(_), DataValue::Float32(_)) => {
                                println!("bitAnd operator is not implemented for Float32!");
                            },
                            (DataValue::Float64(_), DataValue::Float64(_)) => {
                                println!("bitAnd operator is not implemented for Float64!");
                            },
                            (_, _) => {
                                println!("lhs and rhs should have the same Datatype!");
                            }
                        }
                    },
                    ArithOrLogExpr::Or => {
                        match (&op_expr.lhs, op_expr.rhs) {
                            (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitOr);
                            },
                            (DataValue::Float32(_), DataValue::Float32(_)) => {
                                println!("bitAnd operator is not implemented for Float32!");
                            },
                            (DataValue::Float64(_), DataValue::Float64(_)) => {
                                println!("bitAnd operator is not implemented for Float64!");
                            },
                            (_, _) => {
                                println!("lhs and rhs should have the same Datatype!");
                            }
                        }
                    },
                    ArithOrLogExpr::Caret => {
                        match (&op_expr.lhs, op_expr.rhs) {
                            (DataValue::Int8(lhs), DataValue::Int8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::Int16(lhs), DataValue::Int16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::Int32(lhs), DataValue::Int32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::Int64(lhs), DataValue::Int64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::Int128(lhs), DataValue::Int128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::UInt8(lhs), DataValue::UInt8(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::UInt16(lhs), DataValue::UInt16(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::UInt32(lhs), DataValue::UInt32(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::UInt64(lhs), DataValue::UInt64(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::UInt128(lhs), DataValue::UInt128(rhs)) => {
                                crate::ByteCodeFromDataValue!(self, rhs.to_be_bytes(), lhs.to_be_bytes(), op_expr.lhs, OpCode::OpBitXor);
                            },
                            (DataValue::Float32(_), DataValue::Float32(_)) => {
                                println!("bitAnd operator is not implemented for Float32!");
                            },
                            (DataValue::Float64(_), DataValue::Float64(_)) => {
                                println!("bitAnd operator is not implemented for Float64!");
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
