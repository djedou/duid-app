mod binary_expr;
mod op_expr;
mod unary_expr;
mod with_block;
mod without_block;
use crate::vm::data::DataType;


pub use binary_expr::*;
pub use op_expr::*;
pub use unary_expr::*;
pub use with_block::*;
pub use without_block::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    WithBlock(ExprWithBlock), // TODO
    WithoutBlock(ExprWithoutBlck) // Doing
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub return_type: DataType,
    pub expr: Expr
}