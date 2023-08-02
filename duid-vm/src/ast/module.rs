use std::fmt;
use crate::ast::{
    /*AssignOps, ArithOps, SignOps, LogicOps, BitwiseOps, ComparisonOps,
    Mutability, */Visibility//, Type, Parenthesis, Bracket
};
use crate::compiler::vm::data::Data;
/*




#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub open: Bracket,
    pub items: Vec<Statement>,
    pub close: Bracket
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:#?} {:#?} {:#?}", &self.open, &self.items, &self.close)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Expr {
        paren_open: Parenthesis,
        expr: Box<Expression>,
        paren_close: Parenthesis
    
    },

    Byte(u8),
    IntI8(i8),
    IntI16(i16),
    IntI32(i32),
    IntI64(i64),
    IntI128(i128),
    UintU8(u8),
    UintU16(u16),
    UintU32(u32),
    UintU128(u128),
    Float32(eq_float::F32),
    Float64(eq_float::F64),
    Int(i32),
    Bool(bool),
    String(String),
    Char(char)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Value::Expr {paren_open, expr, paren_close} => write!(f, "{:#?}{:#?}{:#?}", paren_open, expr, paren_close),
            Value::Byte(n) => write!(f, "{}", n),
            Value::IntI8(n) => write!(f, "{}", n),
            Value::IntI16(n) => write!(f, "{}", n),
            Value::IntI32(n) => write!(f, "{}", n),
            Value::IntI64(n) => write!(f, "{}", n),
            Value::IntI128(n) => write!(f, "{}", n),
            Value::UintU8(n) => write!(f, "{}", n),
            Value::UintU16(n) => write!(f, "{}", n),
            Value::UintU32(n) => write!(f, "{}", n),
            Value::UintU128(n) => write!(f, "{}", n),
            Value::Float32(n) => write!(f, "{}", n),
            Value::Float64(n) => write!(f, "{}", n),
            Value::Int(n) => write!(f, "{}", n),
            Value::Bool(n) => write!(f, "{}", n),
            Value::String(n) => write!(f, "{}", n),
            Value::Char(n) => write!(f, "{}", n)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncParam {
    Expr(Statement),
    Identifier(String)
}

impl fmt::Display for FuncParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            FuncParam::Expr(expr) => write!(f, "{}", expr),
            FuncParam::Identifier(ident) => write!(f, "{}", ident)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncParams {
    pub paren_open: Parenthesis,
    pub exprs: Vec<FuncParam>,
    pub paren_close: Parenthesis
}

impl fmt::Display for FuncParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:#?} {:#?} {:#?}", &self.paren_open, &self.exprs, &self.paren_close)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    AssignBinaryExpr {
        op: AssignOps,
        lhs: Box<Expression>,
        rhs: Box<Expression>
    },
    ArithBinaryExpr {
        op: ArithOps,
        lhs: Box<Expression>,
        rhs: Box<Expression>
    },
    ArithUnaryExpr {
        op: SignOps,
        expr: Box<Expression>
    },
    LogicBinaryExpr {
        op: LogicOps,
        lhs: Box<Expression>,
        rhs: Box<Expression>
    },
    LogicUnaryExpr {
        op: LogicOps,
        expr: Box<Expression>
    },
    BitwiseBinaryExpr {
        op: BitwiseOps,
        lhs: Box<Expression>,
        rhs: Box<Expression>
    },
    ComparisonBinaryExpr {
        op: ComparisonOps,
        lhs: Box<Expression>,
        rhs: Box<Expression>
    },
    Value(Value),
    FuncCall {
        name: String,
        params: FuncParams
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Expression::AssignBinaryExpr { op, lhs, rhs } => write!(f, "{:#?}{:#?}{:#?}", op, lhs, rhs),
            Expression::ArithBinaryExpr { op, lhs, rhs } => write!(f, "{:#?}{:#?}{:#?}", op, lhs, rhs),
            Expression::ArithUnaryExpr { op, expr } => write!(f, "{:#?}{:#?}", op, expr),
            Expression::LogicBinaryExpr { op, lhs, rhs } => write!(f, "{:#?}{:#?}{:#?}", op, lhs, rhs),
            Expression::LogicUnaryExpr { op, expr } => write!(f, "{:#?}{:#?}", op, expr),
            Expression::BitwiseBinaryExpr { op, lhs, rhs } => write!(f, "{:#?}{:#?}{:#?}", op, lhs, rhs),
            Expression::ComparisonBinaryExpr { op, lhs, rhs } => write!(f, "{:#?}{:#?}{:#?}", op, lhs, rhs),
            Expression::Value(v) => write!(f, "{}", v),
            Expression::FuncCall { name, params } => write!(f, "{:#?} {:#?}", name, params) 
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncDef {
    pub name: String,
    pub args: FuncParams,
    pub return_type: Type,
    pub block: Block
}


impl fmt::Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:#?} {:#?} {:#?} {:#?}", &self.name, &self.args, &self.return_type, &self.block)
    }
}



// ANCHOR: Statement
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Decl(Declaration),
    Expr(Expression)
}
// ANCHOR_END: Statement

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Statement::Decl(d) => write!(f, "{}", d),
            Statement::Expr(e) => write!(f, "{}", e),
            Statement::None => todo!()
        }
    }
}
*/


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    TypeDecl {
        data: Data
    },
    ModuleReExpDecl {
        visible: Visibility,
        items: Vec<String>
    }/*,
    FuncDecl {
        visible: Visibility,
        definition: FuncDef
    },
    VariableDecl {
        mutability: Mutability,
        identifier: String,
        var_type: Type,
        expr: Option<Expression>
    }*/
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Declaration::TypeDecl { data } => write!(f, "{:#?}", data),
            Declaration::ModuleReExpDecl { visible, items } => write!(f, "{:#?} {:#?}", visible, items),
            //Declaration::FuncDecl { visible, definition } => write!(f, "{:#?} {:#?}", visible, definition),
            //Declaration::VariableDecl { mutability, identifier, var_type, expr } => write!(f, "{:#?} {:#?} {:#?} {:#?}", mutability, identifier, var_type, expr)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub visible: Visibility,
    pub namespace: String,
    pub declarations: Vec<Declaration>
}

impl Module {
    pub fn new() -> Self {
        Module {
            visible: Visibility::Privite,
            namespace: String::with_capacity(0),
            declarations: Vec::with_capacity(0)
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        //write!(f, "{} {:#?}", &self.path, &self.statements)
        write!(f, "{} {:#?} {:#?}", &self.namespace, &self.visible, &self.declarations)
    }
}