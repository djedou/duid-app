use crate::ast::{UnaryExpr, BinaryExpr}; 


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpExpr {
    BorrowExpression,
    DereferenceExpression,
    ErrorPropagationExpression,
    NegationExpr(UnaryExpr), // done
    ArithOrLogExpr(BinaryExpr), // done
    ComparisonExpr(BinaryExpr), // done
    LazyBoolExpr(BinaryExpr), // done
    TypeCastExpression,
    AssignmentExpr,
    CompoundAssignmentExpression
}
