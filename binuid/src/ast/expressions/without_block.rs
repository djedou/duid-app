use crate::ast::{OpExpr};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprWithoutBlck {
    GroupedExpression,
    ArrayExpression,
    TupleExpression,
    StructExpression,
    ClosureExpression,
    AsyncBlockExpression,
    ContinueExpression,
    BreakExpression,
    ReturnExpression,
    UnderscoreExpression,
    OpExpr(OpExpr), // Doing
    IndexExpression,
    AwaitExpression,
    TupleIndexingExpression,
    CallExpression,
    MethodCallExpression,
    FieldExpression,
    RangeExpression,
}
