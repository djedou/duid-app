use crate::ast::{UnaryExpr, BinaryExpr}; 


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpExpr {
    BorrowExpression,
    DereferenceExpression,
    UnaryExpr(UnaryExpr), // doing
    BinaryExpr(BinaryExpr), // doing
    TypeCastExpression,
    AssignmentExpr,
    CompoundAssignmentExpression
}
