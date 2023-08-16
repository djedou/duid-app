mod struct_item;


pub use struct_item::*;

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
