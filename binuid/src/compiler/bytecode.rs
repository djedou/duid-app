use crate::vm::opcode::OpCode;
use crate::{Compile, Ast};
use crate::ast::*;
use crate::vm::data::DataType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bytecode {
    pub code: Vec<u8>,
    pub instructions: Vec<u16>,
    pub return_address: u32
}

impl Bytecode {
    fn new() -> Self {
        Self {
            code: Vec::new(),
            instructions: Vec::new(),
            return_address: 0
        }
    }

    pub fn get_address(&self) -> u32 {
        self.code.len() as u32
    }

    pub fn set_return_address(&mut self, value: u32) {
        self.return_address = value;
    }

    pub fn get_return_address(&self) -> u32 {
        self.return_address
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
                ModuleContent::Expr(ex) => {
                    interpreter.add_expression(&ex);
                }
            }
        }

        interpreter.bytecode
    }
}

impl Interpreter {
    fn add_expression(&mut self, expr: &Expression) {
        if expr.return_type == DataType::None {
            panic!("Any Expression should have a return type defined!");
        }

        match &expr.expr {
            Expr::WithoutBlock(ex) => {
                self.add_without_block(ex);
            },
            Expr::WithBlock(_) => {
                //self.add_without_block(expr);
            },
        }
    }

    fn add_without_block(&mut self, without_block: &ExprWithoutBlck) {
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
            ExprWithoutBlck::OpExpr(OpExpr::UnaryExpr(unary_expr)) => {
                crate::UnaryBytecodes!(self, unary_expr);
            },
            ExprWithoutBlck::OpExpr(OpExpr::BinaryExpr(binary_ops)) => {
                match binary_ops.op {
                    BinaryOps::Plus => {crate::OpBinaryBytecodes!(self, OpCode::OpAdd, binary_ops);},
                    BinaryOps::Minus => {crate::OpBinaryBytecodes!(self, OpCode::OpMinus, binary_ops);},
                    BinaryOps::Star => {crate::OpBinaryBytecodes!(self, OpCode::OpStar, binary_ops);},
                    BinaryOps::Slash => {crate::OpBinaryBytecodes!(self, OpCode::OpSlash, binary_ops);},
                    BinaryOps::Percent => {crate::OpBinaryBytecodes!(self, OpCode::OpPercent, binary_ops);},
                    BinaryOps::Caret => {crate::OpBinaryBytecodes!(self, OpCode::OpBitXor, binary_ops);},
                    BinaryOps::And => {crate::OpBinaryBytecodes!(self, OpCode::OpBitAnd, binary_ops);},
                    BinaryOps::Or => {crate::OpBinaryBytecodes!(self, OpCode::OpBitOr, binary_ops);},
                    BinaryOps::AndAnd => {crate::OpBinaryBytecodes!(self, OpCode::OpAndAnd, binary_ops);},
                    BinaryOps::OrOr => {crate::OpBinaryBytecodes!(self, OpCode::OpOrOr, binary_ops);},
                    BinaryOps::Shl => {crate::OpBinaryBytecodes!(self, OpCode::OpShl, binary_ops);},
                    BinaryOps::Shr => {crate::OpBinaryBytecodes!(self, OpCode::OpShr, binary_ops);},
                    BinaryOps::EqEq => {crate::OpCompBinaryBytecodes!(self, OpCode::OpEqEq, binary_ops);},
                    BinaryOps::Ne => {crate::OpCompBinaryBytecodes!(self, OpCode::OpNe, binary_ops);},
                    BinaryOps::Gt => {crate::OpCompBinaryBytecodes!(self, OpCode::OpGt, binary_ops);},
                    BinaryOps::Lt => {crate::OpCompBinaryBytecodes!(self, OpCode::OpLt, binary_ops);},
                    BinaryOps::Ge => {crate::OpCompBinaryBytecodes!(self, OpCode::OpGe, binary_ops);},
                    BinaryOps::Le => {crate::OpCompBinaryBytecodes!(self, OpCode::OpLe, binary_ops);},
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
