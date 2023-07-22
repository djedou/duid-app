#[derive(Debug)]
pub enum Number {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64)
}

#[derive(Debug)]
pub enum Vector {
    V128(i128)
}

#[derive(Debug)]
pub enum Reference {
    FuncRef(u32),
    ExternRef(u32)
}

#[derive(Debug)]
pub enum Value {
    Number(Number),
    Vector(Vector),
    Ref(Reference)
}

#[derive(Debug)]
pub struct Result {
    value: Result<Vec<Vec<Value>>, u32>
}

#[derive(Debug)]
pub struct Option {
    value: Option<Vec<Vec<Value>>>
}

#[derive(Debug)]
pub enum ErrorHandler {
    Option(self::Option),
    Result(self::Result)
}

#[derive(Debug)]
pub struct Function {
    args: ErrorHandler,
    return: ErrorHandler
}

#[derive(Debug)]
pub struct Limit {
    min: u32,
    max: u32
}

#[derive(Debug)]
pub struct Memory {
    limit: Limit
}

#[derive(Debug)]
pub struct Table {
    limit: Limit,
    ref: Reference
}

#[derive(Debug)]
pub enum Mut {
    Const,
    Var
}

#[derive(Debug)]
pub struct Global {
    mut: Mut,
    value: Value
}


#[derive(Debug)]
pub enum External {
    Func(Function),
    Table(Table),
    Memory(Memory),
    Global(Global)
}

#[derive(Debug)]
pub struct String {
    value: Vec<u8>
}



/*
DataStructure
0 => enum
1 => struct

enum fields index or struct properties index 
000 => 
*/