#![allow(clippy::upper_case_acronyms)]

use pest::{self, Parser};
use crate::vm::data::{DataType, DataValue, Variable};
use crate::ast::{*};

// ANCHOR: parser
#[derive(pest_derive::Parser)]
#[grammar = "./src/grammar_parser/literals.pest"]
#[grammar = "./src/grammar_parser/items.pest"]
#[grammar = "./src/grammar_parser/names.pest"]
#[grammar = "./src/grammar_parser/expression.pest"]
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
                        ast = build_ast_from_module(pair);
                    },
                    Rule::Item => {
                        ast.extend_module_content(&build_ast_from_item(pair.clone()));
                    },
                    Rule::Expression => {
                        ast.extend_module_content(&build_ast_from_expr(pair.clone()));
                    },
                    _ => {}
                }
            }
        },
        Err(e) => {
            println!("Error: {:#?}", e);
            return Ok(ast);
        }
    }
    
    Ok(ast)
}

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
                            module_contents.extend_from_slice(&build_ast_from_expr(p.clone()));
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
    pair.clone().into_inner()
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

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Vec<ModuleContent> {
    pair.clone().into_inner()
    .into_iter()
    .map(|p| {
        match p.as_rule() {
            Rule::ExpressionWithoutBlock => {
                ModuleContent::Expr(Expression::WithoutBlock(
                    match p.clone().into_inner().next() {
                        Some(d) => {
                            match d.as_rule() {
                                Rule::OperatorExpression => build_operator_expression(d),
                                _ => {
                                    panic!("");
                                }
                            }
                        },
                        None => {
                            panic!("!!");
                        }
                }))
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

fn build_operator_expression(pair: pest::iterators::Pair<Rule>) -> ExprWithoutBlck {
    match pair.into_inner().next() {
        Some(p) => {
            ExprWithoutBlck::OpExpr(
                match p.as_rule() {
                    Rule::ArithmeticOrLogicalExpression => {
                        let mut data = BinaryExpr::new();
                        for i in p.into_inner() {
                            match i.as_rule() {
                                Rule::DecInt => {
                                    if data.lhs == DataValue::None {
                                        data.lhs = DataValue::Int32(i.as_str().parse::<i32>().unwrap());
                                    }
                                    else {
                                        data.rhs = DataValue::Int32(i.as_str().parse::<i32>().unwrap());
                                    }
                                },
                                Rule::Float => {
                                    if data.lhs == DataValue::None {
                                        data.lhs = DataValue::Float64(eq_float::F64(i.as_str().parse::<f64>().unwrap()));
                                    }
                                    else {
                                        data.rhs = DataValue::Float64(eq_float::F64(i.as_str().parse::<f64>().unwrap()));
                                    }
                                },
                                Rule::Bool => {
                                    if data.lhs == DataValue::None {
                                        data.lhs = DataValue::Bool(i.as_str().parse::<bool>().unwrap());
                                    }
                                    else {
                                        data.rhs = DataValue::Bool(i.as_str().parse::<bool>().unwrap());
                                    }
                                }
                                Rule::Plus => {
                                    data.op = BinaryOps::Arith(ArithExpr::Plus);
                                },
                                Rule::Minus => {
                                    data.op = BinaryOps::Arith(ArithExpr::Minus);
                                },
                                Rule::Star => {
                                    data.op = BinaryOps::Arith(ArithExpr::Star);
                                },
                                Rule::Slash => {
                                    data.op = BinaryOps::Arith(ArithExpr::Slash);
                                },
                                Rule::Percent => {
                                    data.op = BinaryOps::Arith(ArithExpr::Percent);
                                },
                                Rule::And => {
                                    data.op =  BinaryOps::Log(LogExpr::And);
                                },
                                Rule::Or => {
                                    data.op = BinaryOps::Log(LogExpr::Or);
                                },
                                Rule::Caret => {
                                    data.op = BinaryOps::Log(LogExpr::Caret);
                                },
                                Rule::Shl => {
                                    data.op = BinaryOps::Log(LogExpr::Shl);
                                },
                                Rule::Shr => {
                                    data.op = BinaryOps::Log(LogExpr::Shr);
                                },
                                Rule::Identifier => {
                                    if data.lhs == DataValue::None {
                                        data.lhs = DataValue::Variable(Variable {
                                            value: i.as_str().to_string(),
                                            data_type: DataType::None
                                        });
                                    }
                                    else {
                                        data.rhs = DataValue::Variable(Variable {
                                            value: i.as_str().to_string(),
                                            data_type: DataType::None
                                        });
                                    }
                                },
                                Rule::Annotated => {
                                    let mut value = "";
                                    let mut data_type = DataType::None;

                                    for m in i.into_inner() {
                                        match m.as_rule() {
                                            Rule::Identifier => {
                                                value = m.as_str();
                                                data_type = DataType::Variable;
                                            },
                                            Rule::Type => {
                                                data_type = DataType::from(m.as_str())
                                            },
                                            _ => {
                                                value = m.as_str();
                                            }
                                        }
                                    }

                                    match data_type {
                                        DataType::Int8 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<i8>() {
                                                                Ok(v) => DataValue::Int8(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::Int8
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<i8>() {
                                                    Ok(v) => DataValue::Int8(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::Int8
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::Int16 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<i16>() {
                                                                Ok(v) => DataValue::Int16(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::Int16
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<i16>() {
                                                    Ok(v) => DataValue::Int16(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::Int16
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::Int32 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<i32>() {
                                                                Ok(v) => DataValue::Int32(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::Int32
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<i32>() {
                                                    Ok(v) => DataValue::Int32(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::Int32
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::Int64 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<i64>() {
                                                                Ok(v) => DataValue::Int64(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::Int64
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<i64>() {
                                                    Ok(v) => DataValue::Int64(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::Int64
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::Int128 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<i128>() {
                                                                Ok(v) => DataValue::Int128(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::Int128
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<i128>() {
                                                    Ok(v) => DataValue::Int128(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::Int128
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::UInt8 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<u8>() {
                                                                Ok(v) => DataValue::UInt8(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::UInt8
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<u8>() {
                                                    Ok(v) => DataValue::UInt8(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::UInt8
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::UInt16 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<u16>() {
                                                                Ok(v) => DataValue::UInt16(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::UInt16
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<u16>() {
                                                    Ok(v) => DataValue::UInt16(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::UInt16
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::UInt32 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<u32>() {
                                                                Ok(v) => DataValue::UInt32(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::UInt32
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<u32>() {
                                                    Ok(v) => DataValue::UInt32(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::UInt32
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::UInt64 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<u64>() {
                                                                Ok(v) => DataValue::UInt64(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::UInt64
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<u64>() {
                                                    Ok(v) => DataValue::UInt64(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::UInt64
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::UInt128 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<u128>() {
                                                                Ok(v) => DataValue::UInt128(v),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::UInt128
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<u128>() {
                                                    Ok(v) => DataValue::UInt128(v),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::UInt128
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::Float32 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<f32>() {
                                                                Ok(v) => DataValue::Float32(eq_float::F32(v)),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::Float32
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<f32>() {
                                                    Ok(v) => DataValue::Float32(eq_float::F32(v)),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::Float32
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::Float64 => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = match value.parse::<f64>() {
                                                                Ok(v) => DataValue::Float64(eq_float::F64(v)),
                                                                Err(_) => DataValue::Variable(Variable {
                                                                    value: value.to_string(),
                                                                    data_type: DataType::Float64
                                                                })
                                                            };
                                            }
                                            else {
                                                data.rhs = match value.parse::<f64>() {
                                                    Ok(v) => DataValue::Float64(eq_float::F64(v)),
                                                    Err(_) => DataValue::Variable(Variable {
                                                        value: value.to_string(),
                                                        data_type: DataType::Float64
                                                    })
                                                };
                                                
                                            }
                                        },
                                        DataType::Variable => {
                                            if data.lhs == DataValue::None {
                                                data.lhs = DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::None
                                                });
                                            }
                                            else {
                                                data.rhs = DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::None
                                                });
                                            }
                                        },
                                        _ => {}
                                    }
                                },
                                r => {
                                    panic!("Rule {:?} is not yet implemented!!", r);
                                }
                            }
                        }

                        OpExpr::ArithOrLogExpr(data)
                    },
                    Rule::NegationExpression => {
                        let mut data = UnaryExpr::new();
                        for i in p.into_inner() {
                            match i.as_rule() {
                                Rule::DecInt => {
                                    data.rhs = DataValue::Int32(i.as_str().parse::<i32>().unwrap());
                                },
                                Rule::Bool => {
                                    data.rhs = DataValue::Bool(i.as_str().parse::<bool>().unwrap());
                                }
                                Rule::Minus => {
                                    data.op = UnaryOps::Minus;
                                },
                                Rule::Not => {
                                    data.op = UnaryOps::Not;
                                },
                                Rule::Identifier => {
                                    data.rhs = DataValue::Variable(Variable {
                                        value: i.as_str().to_string(),
                                        data_type: DataType::None
                                    });
                                },
                                Rule::Annotated => {
                                    let mut value = "";
                                    let mut data_type = DataType::None;

                                    for m in i.into_inner() {
                                        match m.as_rule() {
                                            Rule::Identifier => {
                                                value = m.as_str();
                                                data_type = DataType::Variable;
                                            },
                                            Rule::Type => {
                                                data_type = DataType::from(m.as_str())
                                            },
                                            _ => {
                                                value = m.as_str();
                                            }
                                        }
                                    }

                                    match data_type {
                                        DataType::Int8 => {
                                            data.rhs = match value.parse::<i8>() {
                                                Ok(v) => DataValue::Int8(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::Int8
                                                })
                                            };
                                        },
                                        DataType::Int16 => {
                                            data.rhs = match value.parse::<i16>() {
                                                Ok(v) => DataValue::Int16(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::Int16
                                                })
                                            };
                                        },
                                        DataType::Int32 => {
                                            data.rhs = match value.parse::<i32>() {
                                                Ok(v) => DataValue::Int32(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::Int32
                                                })
                                            };
                                        },
                                        DataType::Int64 => {
                                            data.rhs = match value.parse::<i64>() {
                                                Ok(v) => DataValue::Int64(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::Int64
                                                })
                                            };
                                        },
                                        DataType::Int128 => {
                                            data.rhs = match value.parse::<i128>() {
                                                Ok(v) => DataValue::Int128(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::Int128
                                                })
                                            };
                                        },
                                        DataType::UInt8 => {
                                            data.rhs = match value.parse::<u8>() {
                                                Ok(v) => DataValue::UInt8(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::UInt8
                                                })
                                            };
                                        },
                                        DataType::UInt16 => {
                                            data.rhs = match value.parse::<u16>() {
                                                Ok(v) => DataValue::UInt16(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::UInt16
                                                })
                                            };
                                        },
                                        DataType::UInt32 => {
                                            data.rhs = match value.parse::<u32>() {
                                                Ok(v) => DataValue::UInt32(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::UInt32
                                                })
                                            };
                                        },
                                        DataType::UInt64 => {
                                            data.rhs = match value.parse::<u64>() {
                                                Ok(v) => DataValue::UInt64(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::UInt64
                                                })
                                            };
                                        },
                                        DataType::UInt128 => {
                                            data.rhs = match value.parse::<u128>() {
                                                Ok(v) => DataValue::UInt128(v),
                                                Err(_) => DataValue::Variable(Variable {
                                                    value: value.to_string(),
                                                    data_type: DataType::UInt128
                                                })
                                            };
                                        },
                                        DataType::Variable => {
                                            data.rhs = DataValue::Variable(Variable {
                                                value: value.to_string(),
                                                data_type: DataType::None
                                            });
                                        },
                                        _ => {}
                                    }
                                },
                                r => {
                                    panic!("Rule {:?} is not yet implemented!!", r);
                                }
                            }
                        }

                        OpExpr::NegationExpr(data)
                    }
                    _ => {
                        panic!("Missing Expression without block!");
                    }
                }
            )
        },
        None => {
            panic!("Missing Expression without block!");
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    
    #[test]
    fn module_declaration() {
        assert!(parse("let var: Int = ").is_ok());
    }
    
}
