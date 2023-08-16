use crate::ast::{UnaryExpr, BinaryExpr}; 


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpExpr {
    BorrowExpression,
    DereferenceExpression,
    ErrorPropagationExpression,
    NegationExpr(UnaryExpr), // to be reviews
    ArithOrLogExpr(BinaryExpr), // done
    ComparisonExpr(BinaryExpr), // done
    LazyBoolExpr(BinaryExpr), // done
    TypeCastExpression,
    AssignmentExpr,
    CompoundAssignmentExpression
}
