use std::fmt;
use crate::ast::{Visibility};
use crate::compiler::vm::data::{DataType, DataValue};



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogExpr {
    And,
    Or,
    Caret,
    Shl,
    Shr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithExpr {
    Plus,
    Minus,
    Star,
    Slash,
    Percent
}

impl From<&str> for LogExpr {
    fn from(value: &str) -> Self {
        match value {
            "&" => LogExpr::And,
            "|" => LogExpr::Or,
            "^" => LogExpr::Caret,
            "<<" => LogExpr::Shl,
            ">>" => LogExpr::Shr,
            o => panic!("Unknowed operator: {}", o)
        }
    }
}

impl From<&str> for ArithExpr {
    fn from(value: &str) -> Self {
        match value {
            "+" => ArithExpr::Plus,
            "-" => ArithExpr::Minus,
            "*" => ArithExpr::Star,
            "/" => ArithExpr::Slash,
            "%" => ArithExpr::Percent,
            o => panic!("Unknowed operator: {}", o)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOps {
    None,
    Arith(ArithExpr),
    Log(LogExpr)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpr {
    pub lhs: DataValue,
    pub op: BinaryOps,
    pub rhs: DataValue
}

impl BinaryExpr {
    pub fn new() -> Self {
        BinaryExpr {
            lhs: DataValue::None,
            op: BinaryOps::None,
            rhs: DataValue::None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpExpr {
    BorrowExpression,
    DereferenceExpression,
    ErrorPropagationExpression,
    NegationExpression,
    ArithOrLogExpr(BinaryExpr), // done
    ComparisonExpression,
    LazyBooleanExpression,
    TypeCastExpression,
    AssignmentExpression,
    CompoundAssignmentExpression
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionWithBlock {
    BlockExpression,
    LoopExpression,
    IfExpression,
    IfLetExpression,
    MatchExpression,
}


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
    OpExpr(OpExpr),
    IndexExpression,
    AwaitExpression,
    TupleIndexingExpression,
    CallExpression,
    MethodCallExpression,
    FieldExpression,
    RangeExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Struct {
    UnitStruct(DataType),
    StructStruct(String),
    TupleStruct(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    None,
    //UseDeclaration(Ast),
    Struct(Struct),
    //Enum(Ast),
    //Impl(Ast),
    //Func(Ast),
    //ConstItem(Ast),
    //StaticItem(Ast)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    WithBlock(ExpressionWithBlock),
    WithoutBlock(ExprWithoutBlck)
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleHeader {
    pub visible: Visibility,
    pub namespace: Option<String>,
    pub exported: Vec<DataType>
}

impl ModuleHeader {
    pub fn new() -> Self {
        ModuleHeader {
            visible: Visibility::Privite,
            namespace: None,
            exported: Vec::with_capacity(0)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleContent {
    None,
    Item(Item),
    Expr(Expression)
}

impl fmt::Display for ModuleContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            ModuleContent::None => write!(f, "None"),
            ModuleContent::Item(item) => write!(f, "{:#?}", item),
            ModuleContent::Expr(expr) => write!(f, "{:#?}", expr)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub header: ModuleHeader,
    pub contents: Vec<ModuleContent>
}

impl Module {
    pub fn new() -> Self {
        Module {
            header: ModuleHeader::new(),
            contents: Vec::with_capacity(0)
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:#?} {:#?}", &self.header, &self.contents)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub module: Module,
}

impl Ast {
    pub fn extend_module_content(&mut self, contents: &[ModuleContent]) { 
        self.module.contents.extend_from_slice(contents);
    }
}
