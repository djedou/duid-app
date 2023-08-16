#![allow(clippy::upper_case_acronyms)]

use pest::{self, Parser, error::LineColLocation};
use crate::{
    vm::data::{DataType, Data, DataValue},
    ast::{*},
    utils::{decode_hex, boolean_into_bits}
};

// ANCHOR: parser
#[derive(pest_derive::Parser)]
#[grammar = "./src/grammar_parser/literals.pest"]
#[grammar = "./src/grammar_parser/data.pest"]
#[grammar = "./src/grammar_parser/items.pest"]
#[grammar = "./src/grammar_parser/expression.pest"]
#[grammar = "./src/grammar_parser/names.pest"]
#[grammar = "./src/grammar_parser/statement.pest"]
#[grammar = "./src/grammar_parser/entry_point.pest"]
struct DuidParser;


pub fn parse(source: &str) -> std::result::Result<Ast, pest::error::Error<Rule>> {
    
    let mut ast = Ast {
        module: Module::new()
    };

    match DuidParser::parse(Rule::ModuleFile, source) {
        Ok(pairs) => {
            //println!("pairs: {:#?}", pairs);
            for pair in pairs {
                match pair.as_rule() {
                    Rule::Module => {
                        //ast = build_ast_from_module(pair);
                    },
                    Rule::Item => {
                        //ast.extend_module_content(&build_ast_from_item(pair.clone()));
                    },
                    Rule::Expression => {
                        ast.extend_module_content(&build_ast_from_module_content(pair.clone()));
                    },
                    _ => {}
                }
            }
        },
        Err(e) => {
            //println!("Err: {:#?}", e);
            let LineColLocation::Pos((line, col)) = e.line_col else {
                return Ok(ast);
            };

            println!("Parsing Error: line: {:?} and column: {:?}", line, col);
        }
    }
    
    Ok(ast)
}
/*
fn build_ast_from_module(pair: pest::iterators::Pair<Rule>) -> Ast {
    
    let mut module_header = ModuleHeader::new();
    let mut module_contents: Vec<_> = vec![];

    for pa in pair.clone().into_inner() {
        match pa.as_rule() {
            Rule::ModuleHeader => {
                for p in pa.clone().into_inner() {
                    match p.as_rule() {
                        Rule::ItemVis => {
                            module_header.visible = Visibility::Public;
                        },
                        Rule::ModulePath => {
                            module_header.namespace = Some(p.as_str().trim_start_matches("mod ").to_string());
                        },
                        Rule::ModuleItemExported => {
                            for exp in p.clone().into_inner() {
                                match exp.as_rule() {
                                    Rule::SimplePathSegment => {
                                        module_header.exported.push(DataType::from(exp.as_str()));
                                    },
                                    _ => {}
                                }
                            }
                        },
                        _ => {}
                    }
                }
            },
            Rule::ModuleContent => {
                for p in pa.clone().into_inner() {
                    match p.as_rule() {
                        Rule::Item => {
                            module_contents.extend_from_slice(&build_ast_from_item(p.clone()));
                        },
                        Rule::Expression => {
                            module_contents.extend_from_slice(&build_ast_from_module_content(p.clone()));
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }

    Ast {
        module: Module {
            header: module_header,
            contents: module_contents
        }
    }
}


fn build_ast_from_item(pair: pest::iterators::Pair<Rule>) -> Vec<ModuleContent> {
    pair.into_inner()
    .into_iter()
    .map(|p| {
        match p.as_rule() {
            Rule::UnitStruct => {
                match p.clone().into_inner().next() {
                    Some(d) => {
                        match d.as_str() {
                            "Int8" => {
                                let data_type = DataType::from("Int8");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "Int16" => {
                                let data_type = DataType::from("Int16");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "Int32" => {
                                let data_type = DataType::from("Int32");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "Int64" => {
                                let data_type = DataType::from("Int64");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "Int128" => {
                                let data_type = DataType::from("Int128");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "UInt8" => {
                                let data_type = DataType::from("UInt8");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "UInt16" => {
                                let data_type = DataType::from("UInt16");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "UInt32" => {
                                let data_type = DataType::from("UInt32");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "UInt64" => {
                                let data_type = DataType::from("UInt64");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "UInt128" => {
                                let data_type = DataType::from("UInt128");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "Float32" => {
                                let data_type = DataType::from("Float32");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "Float64" => {
                                let data_type = DataType::from("Float64");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "Byte" => {
                                let data_type = DataType::from("Byte");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "Bool" => {
                                let data_type = DataType::from("Bool");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "String" => {
                                let data_type = DataType::from("String");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            "Chr" => {
                                let data_type = DataType::from("Chr");
                                ModuleContent::Item(Item::Struct(Struct::UnitStruct(data_type)))
                            },
                            _ => ModuleContent::None
                        }
                    },
                    None => ModuleContent::None
                }
            },
            _ => ModuleContent::None
        }
    })
    .filter(|p| p != &ModuleContent::None)
    .collect()
}
*/

fn build_ast_from_module_content(pair: pest::iterators::Pair<Rule>) -> Vec<ModuleContent> {
    pair.into_inner()
    .into_iter()
    .map(|p| {
        match p.as_rule() {
            Rule::ExpressionWithoutBlock => {
                ModuleContent::Expr(
                    match p.clone().into_inner().next() {
                        Some(d) => {
                            build_ast_from_expr(d)
                        },
                        None => {
                            panic!("!!");
                        }
                    }
                )
            },
            Rule::ExpressionWithBlock => {
                ModuleContent::None
            },
            _ => {
                panic!("eee!");
            }
        }
    })
    .filter(|p| p != &ModuleContent::None)
    .collect()
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::OperatorExpression => {
            let (return_type, expr) = build_operator_expression(pair.clone());
            Expression {
                return_type,
                expr: Expr::WithoutBlock(expr)
            }
        },
        _ => {
            panic!("");
        }
    }
}


fn build_operator_expression(pair: pest::iterators::Pair<Rule>) -> (DataType, ExprWithoutBlck) {
    match pair.into_inner().next() {
        Some(p) => {
            match p.as_rule() {
                Rule::UnaryExpr => {
                    let (data_type, unary_expr) = build_ast_unary(p.clone());
                    (data_type, ExprWithoutBlck::OpExpr(OpExpr::UnaryExpr(unary_expr)))
                },
                Rule::BinaryExpr => {
                    let (data_type, binary_expr) = build_ast_binary(p.clone());
                    (data_type, ExprWithoutBlck::OpExpr(OpExpr::BinaryExpr(binary_expr)))
                },
                _ => {
                    panic!("Missing Expression without block!");
                }
            }
        },
        None => {
            panic!("Missing Expression without block!");
        }
    }
}


fn build_ast_binary(pair: pest::iterators::Pair<Rule>) -> (DataType, BinaryExpr) { 
    let mut data_type = DataType::default();
    let mut binary_expr = BinaryExpr::new();

    for i in pair.into_inner() {
        match i.as_rule() {
            Rule::UnaryExpr => {
                let (new_data_type, unary_expr) = build_ast_unary(i.clone());
                if binary_expr.lhs.data == UnaryData::None {
                    binary_expr.lhs = unary_expr;
                    data_type = new_data_type;
                }
                else {
                    binary_expr.rhs = unary_expr;
                    data_type = new_data_type;
                }
            },
            Rule::BinaryOps => {
                match i.into_inner().next() {
                    Some(next) => {
                        match next.as_rule() {
                            Rule::Plus => {
                                binary_expr.op = BinaryOps::Plus;
                            },
                            Rule::Minus => {
                                binary_expr.op = BinaryOps::Minus;
                            },
                            Rule::Star => {
                                binary_expr.op = BinaryOps::Star;
                            },
                            Rule::Slash => {
                                binary_expr.op = BinaryOps::Slash;
                            },
                            Rule::Percent => {
                                binary_expr.op = BinaryOps::Percent;
                            },
                            Rule::Caret => {
                                binary_expr.op = BinaryOps::Caret;
                            },
                            Rule::And => {
                                binary_expr.op = BinaryOps::And;
                            },
                            Rule::Or => {
                                binary_expr.op = BinaryOps::Or;
                            },
                            Rule::AndAnd => {
                                binary_expr.op = BinaryOps::AndAnd;
                            },
                            Rule::OrOr => {
                                binary_expr.op = BinaryOps::OrOr;
                            },
                            Rule::Shl => {
                                binary_expr.op = BinaryOps::Shl;
                            },
                            Rule::Shr => {
                                binary_expr.op = BinaryOps::Shr;
                            },
                            Rule::EqEq => {
                                binary_expr.op = BinaryOps::EqEq;
                            },
                            Rule::Ne => {
                                binary_expr.op = BinaryOps::Ne;
                            },
                            Rule::Gt => {
                                binary_expr.op = BinaryOps::Gt;
                            },
                            Rule::Lt => {
                                binary_expr.op = BinaryOps::Lt;
                            },
                            Rule::Ge => {
                                binary_expr.op = BinaryOps::Ge;
                            },
                            Rule::Le => {
                                binary_expr.op = BinaryOps::Le;
                            },
                            _ => {
                                binary_expr.op = BinaryOps::None;
                            }
                        }
                    },
                    None => {}
                }
            },
            r => {
                panic!("Rule {:?} is not yet implemented!!", r);
            }
        }
    }

    (data_type, binary_expr)
}


fn build_ast_unary(pair: pest::iterators::Pair<Rule>) -> (DataType, UnaryExpr) { 
    let mut data_type = DataType::default();
    let mut unary_expr = UnaryExpr::new();

    for i in pair.into_inner() {
        match i.as_rule() {
            Rule::UnaryOps => {
                match i.into_inner().next() {
                    Some(next) => {
                        match next.as_rule() {
                            Rule::Minus => {
                                unary_expr.op = UnaryOps::Minus;
                            },
                            Rule::Not => {
                                unary_expr.op = UnaryOps::Not;
                            },
                            _ => {
                                unary_expr.op = UnaryOps::Plus;
                            }
                        }
                    },
                    None => {}
                }
            },
            Rule::Data => {
                let data = build_ast_data(i.clone());
                data_type = data.type_.clone();
                unary_expr.data = UnaryData::Data(data)
            },
            Rule::Expression => {
                match i.clone().into_inner().next() {
                    Some(d) => {
                        let expr = build_ast_from_expr(d);
                        data_type = expr.return_type.clone();
                        unary_expr.data = UnaryData::Expr(Box::new(expr))
                    },
                    None => {
                        panic!("!!");
                    }
                }
            },
            r => {
                panic!("Rule {:?} is not yet implemented!!", r);
            }
        }
    }

    (data_type, unary_expr)
}

fn build_ast_data(pair: pest::iterators::Pair<Rule>) -> Data { 
    let mut data = Data::new();
    let mut value = "";

    for i in pair.into_inner() {
        match i.as_rule() {
            Rule::DecInt => {
                data.value = match i.as_str().parse::<i32>() {
                    Ok(v) => DataValue::I32Value(v),
                    Err(_) => DataValue::None
                };
                data.type_ = DataType::I32Type;
                data.size = 4;
                value = i.as_str();
            },
            Rule::FloatValue => {
                data.value = match i.as_str().parse::<f64>() {
                    Ok(v) => DataValue::F64Value(eq_float::F64(v)),
                    Err(_) => DataValue::None
                };
                data.type_ = DataType::F64Type;
                data.size = 8;
                value = i.as_str();
            },
            Rule::Variable => {
                data.value = DataValue::VariableValue(i.as_str().as_bytes().to_vec());
            },
            Rule::ByteValue => {
                data.value = match decode_hex(i.as_str()) {
                    Ok(v) => DataValue::ByteValue(v[0]),
                    Err(_) => DataValue::None
                };
                data.type_ = DataType::ByteType;
                data.size = 1;
            },
            Rule::ByteType => {
                data.type_ = DataType::ByteType;
                data.size = 1;
            },
            Rule::BoolValue => {
                data.value = match i.as_str().parse::<bool>() {
                    Ok(v) => DataValue::BoolValue(boolean_into_bits(&v)),
                    Err(_) => DataValue::None
                };
                data.type_ = DataType::BoolType;
                data.size = 1;
            },
            Rule::BoolType => {
                data.type_ = DataType::BoolType;
                data.size = 1;
            },
            Rule::F32Type => {
                if let Ok(v) = value.parse::<f32>() {
                    data.value = DataValue::F32Value(eq_float::F32(v))
                };
                data.type_ = DataType::F32Type;
                data.size = 4;
            },
            Rule::F64Type => {
                data.type_ = DataType::F64Type;
                data.size = 8;
            },
            Rule::I8Type => {
                if let Ok(v) = value.parse::<i8>() {
                    data.value = DataValue::I8Value(v)
                };
                data.type_ = DataType::I8Type;
                data.size = 1;
            },
            Rule::I16Type => {
                if let Ok(v) = value.parse::<i16>() {
                    data.value = DataValue::I16Value(v)
                };
                data.type_ = DataType::I16Type;
                data.size = 2;
            },
            Rule::I32Type => {
                if let Ok(v) = value.parse::<i32>() {
                    data.value = DataValue::I32Value(v)
                };
                data.type_ = DataType::I32Type;
                data.size = 4;
            },
            Rule::I64Type => {
                if let Ok(v) = value.parse::<i64>() {
                    data.value = DataValue::I64Value(v)
                };
                data.type_ = DataType::I64Type;
                data.size = 8;
            },
            Rule::I128Type => {
                if let Ok(v) = value.parse::<i128>() {
                    data.value = DataValue::I128Value(v)
                };
                data.type_ = DataType::I128Type;
                data.size = 16;
            },
            Rule::U8Type => {
                if let Ok(v) = value.parse::<u8>() {
                    data.value = DataValue::U8Value(v)
                };
                data.type_ = DataType::U8Type;
                data.size = 1;
            },
            Rule::U16Type => {
                if let Ok(v) = value.parse::<u16>() {
                    data.value = DataValue::U16Value(v)
                };
                data.type_ = DataType::U16Type;
                data.size = 2;
            },
            Rule::U32Type => {
                if let Ok(v) = value.parse::<u32>() {
                    data.value = DataValue::U32Value(v)
                };
                data.type_ = DataType::U32Type;
                data.size = 4;
            },
            Rule::U64Type => {
                if let Ok(v) = value.parse::<u64>() {
                    data.value = DataValue::U64Value(v)
                };
                data.type_ = DataType::U64Type;
                data.size = 8;
            },
            Rule::U128Type => {
                if let Ok(v) = value.parse::<u128>() {
                    data.value = DataValue::U128Value(v)
                };
                data.type_ = DataType::U128Type;
                data.size = 16;
            },
            Rule::StringValue => {
                let value = i.as_str().as_bytes().to_vec();
                data.size = value.len() as u32;
                data.value = DataValue::StringValue(value);
                data.type_ = DataType::StringType;
            },
            Rule::StringType => {
                data.type_ = DataType::StringType;
            },
            Rule::CharValue => {
                let char_vec: Vec<char> = i.as_str().chars().collect();
                let char_value: char = char_vec[1];
                data.size = 4;
                data.value = DataValue::CharValue(u32::from(char_value));
                data.type_ = DataType::CharType;
            },
            Rule::CharType => {
                data.type_ = DataType::CharType;
            },
            Rule::UnitType => {
                data.type_ = DataType::UnitType;
            },
            /*
            | FnType 
            | RefType
            | TupleType
            | ArrayType
            | VecType
            | CustomerType
            */
            _ => {}
        }
    }

    data
}


#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn data_comment_one_line() {
        assert!(parse("/* xFF */").is_ok());
    }

    #[test]
    fn data_comment_blockline() {
        assert!(parse(r#"/* first line 
                        next line */
                    "#).is_ok());
    }

    #[test]
    fn data_byte_without_annotation() {
        assert!(parse("xFF").is_ok());
    }

    #[test]
    fn data_byte_with_annotation() {
        assert!(parse("x00:byte").is_ok());
        assert!(parse("value:byte").is_ok());
    }

    #[test]
    fn data_bool_without_annotation() {
        assert!(parse("true").is_ok());
    }

    #[test]
    fn data_bool_with_annotation() {
        assert!(parse("false:bool").is_ok());
        assert!(parse("value:bool").is_ok());
    }

    #[test]
    fn data_float_without_annotation() {
        assert!(parse("25.58").is_ok());
        assert!(parse("25.").is_ok());
    }

    #[test]
    fn data_float_with_annotation() {
        assert!(parse("25.8:f32").is_ok());
        assert!(parse("value:f64").is_ok());
    }
    
    #[test]
    fn data_int_without_annotation() {
        assert!(parse("25").is_ok());
    }

    #[test]
    fn data_int_with_annotation() {
        assert!(parse("25:i8").is_ok());
        assert!(parse("25:i16").is_ok());
        assert!(parse("25:i32").is_ok());
        assert!(parse("25:i64").is_ok());
        assert!(parse("25:i128").is_ok());
        assert!(parse("25:u8").is_ok());
        assert!(parse("25:u16").is_ok());
        assert!(parse("25:u32").is_ok());
        assert!(parse("25:u64").is_ok());
        assert!(parse("25:u128").is_ok());
        assert!(parse("value:u128").is_ok());
    }

    #[test]
    fn data_string_and_char_without_annotation() {
        assert!(parse("\"25\"").is_ok());
        assert!(parse("\'a\'").is_ok());
        //assert!(parse("r#\"djedou\"#").is_ok());
    }

    #[test]
    fn data_string_and_char_with_annotation() {
        assert!(parse("value: str").is_ok());
    }

    #[test]
    fn data_unit_without_annotation() {
        assert!(parse("()").is_ok());
    }

    #[test]
    fn data_unit_with_annotation() {
        assert!(parse("value: unit").is_ok());
    }

    #[test]
    fn data_tuple_with_annotation() {
        assert!(parse("value:(u8, u16)").is_ok());
    }

    #[test]
    fn data_array_with_annotation() {
        assert!(parse("value: [u8; 16]").is_ok());
    }

    #[test]
    fn data_ref_with_annotation() {
        assert!(parse("value: &u8").is_ok());
    }

    #[test]
    fn data_func_with_annotation() {
        assert!(parse("value: Fn(u8, u32) -> u16").is_ok());
    }

    #[test]
    fn data_generic_with_annotation() {
        assert!(parse("value: Custom").is_ok());
        assert!(parse("value: Custom<A>").is_ok());
        assert!(parse("value: Custom<A:u8>").is_ok());
        assert!(parse("value: Custom<A:u8, B: u16, C>").is_ok());
    }

    #[test]
    fn data_vec_with_annotation() {
        assert!(parse("value: Vec<Custom>").is_ok());
        assert!(parse("value: Vec<A>").is_ok());
        assert!(parse("value: Vec<u16>").is_ok());
    }

    #[test]
    fn data_module_with_annotation() {
        assert!(parse(r#"value: Vec<Custom>;"#).is_ok());
    }

    #[test]
    fn data_binary_with_annotation() {
        assert!(parse("+3 + (5 + 8)").is_ok());
    }
    
}
