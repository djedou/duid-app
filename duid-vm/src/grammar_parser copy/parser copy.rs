#![allow(clippy::upper_case_acronyms)]

use pest::{self, Parser};

use crate::ast::{Module/*, Statement, Expression, Declaration, */};

// ANCHOR: parser
#[derive(pest_derive::Parser)]
#[grammar = "./src/grammar_parser/value_grammar.pest"]
//#[grammar = "./src/grammar_parser/grammar.pest"]
struct DuidParser;


pub fn parse(source: &str) -> std::result::Result<Module, pest::error::Error<Rule>> {
    let mut ast = Module { path: String::new()};
    let pairs = DuidParser::parse(Rule::Module, source)?;
    println!("pairs: {:#?}", pairs);
    /*for pair in pairs {
        if let Rule::Statement = pair.as_rule() {
            ast.statements.push(build_ast_from_statement(pair));
        }
    }*/
    Ok(ast)
}
/*
fn build_ast_from_statement(pair: pest::iterators::Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::Expr => Statement::Expr(build_ast_from_expr(pair.into_inner().next().unwrap())),
        Rule::Declaration => Statement::Decl(build_ast_from_decl(pair.into_inner().next().unwrap())),
        Rule::Comment => todos!()
    }
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::AssignBinaryExpr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::ArithBinaryExpr => build_ast_from_expr(pair.into_inner().next().unwrap()), 
        Rule::ArithUnaryExpr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::LogicBinaryExpr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::LogicUnaryExpr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::BitwiseBinaryExpr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::ComparisonBinaryExpr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::Value => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::FuncCall  => build_ast_from_expr(pair.into_inner().next().unwrap()),
    }
}

fn build_ast_from_decl(pair: pest::iterators::Pair<Rule>) -> Declaration {
    println!("pair: {:#?}", pair);
    
    match pair.as_rule() {
        Rule::Bool => build_ast_from_expr(pair.into_inner().next().unwrap()),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
    
    Statement::None
}
*/

/*
fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::UnaryExpr => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = pair.next().unwrap();
            let child = build_ast_from_term(child);
            parse_unary_expr(op, child)
        }
        Rule::BinaryExpr => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let mut lhs = build_ast_from_term(lhspair);
            let mut op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let mut rhs = build_ast_from_term(rhspair);
            let mut retval = parse_binary_expr(op, lhs, rhs);
            loop {
                let pair_buf = pair.next();
                if pair_buf != None {
                    op = pair_buf.unwrap();
                    lhs = retval;
                    rhs = build_ast_from_term(pair.next().unwrap());
                    retval = parse_binary_expr(op, lhs, rhs);
                } else {
                    return retval;
                }
            }
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Int => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "-" => (-1, &istr[1..]),
                _ => (1, istr),
            };
            let int: i32 = istr.parse().unwrap();
            Node::Int(sign * int)
        }
        Rule::Expr => build_ast_from_expr(pair),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn parse_unary_expr(pair: pest::iterators::Pair<Rule>, child: Node) -> Node {
    Node::UnaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => unreachable!(),
        },
        child: Box::new(child),
    }
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs: Node) -> Node {
    Node::BinaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => unreachable!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}
*/

#[cfg(test)]
mod parser_tests {
    use super::*;

    /*
    #[test]
    fn module_declaration() {
        assert!(parse("mod path1::path2::path3;").is_ok());
    }

    #[test]
    fn module_declaration_pub() {
        assert!(parse("pub mod path1::path2::path3;").is_ok());
    }

    #[test]
    fn module_re_export() {
        assert!(parse("use path1::path2::path3;").is_ok());
    }

    #[test]
    fn module_re_export_multy() {
        assert!(parse("use path1::path2::path3::{M1,M2,M3};").is_ok());
    }

    #[test]
    fn module_re_export_multy_pub() {
        assert!(parse("pub use path1::path2::path3::{M1, M2, M3};").is_ok());
    }

   #[test]
    fn module_type_tuple() {
        assert!(parse("tuple Djedou (qfd, qfsqfd ,qdf);").is_ok());
    }

    #[test]
    fn module_type_list() {
        assert!(parse("list Djedou [qfd, qfsqfd ,qdf];").is_ok());
    }*/

    
    #[test]
    fn literals_int() {
        assert!(parse("45").is_ok());
    }
/*
    #[test]
    fn literals_inti8() {
        assert!(parse("45:i8").is_ok());
    }

    #[test]
    fn literals_bin_inti8() {
        assert!(parse("0b10101110:i8").is_ok());
    }

    #[test]
    fn literals_hex_inti8() {
        assert!(parse("0x0FCD:i8").is_ok());
    }

    #[test]
    fn literals_inti16() {
        assert!(parse("45:i16").is_ok());
    }

    #[test]
    fn literals_inti32() {
        assert!(parse("45:i32").is_ok());
    }

    #[test]
    fn literals_inti64() {
        assert!(parse("45:i64").is_ok());
    }

    #[test]
    fn literals_inti128() {
        assert!(parse("45:i128").is_ok());
    }

    #[test]
    fn literals_sign_inti8() {
        assert!(parse("-45:i8").is_ok());
    }

    #[test]
    fn literals_sign_inti16() {
        assert!(parse("-45:i16").is_ok());
    }

    #[test]
    fn literals_sign_inti32() {
        assert!(parse("-45:i32").is_ok());
    }

    #[test]
    fn literals_sign_inti64() {
        assert!(parse("-45:i64").is_ok());
    }

    #[test]
    fn literals_sign_inti128() {
        assert!(parse("-45:i128").is_ok());
    }

    #[test]
    fn literals_intu8() {
        assert!(parse("45:u8").is_ok());
    }

    #[test]
    fn literals_intu16() {
        assert!(parse("45:u16").is_ok());
    }

    #[test]
    fn literals_intu32() {
        assert!(parse("45:u32").is_ok());
    }

    #[test]
    fn literals_intu64() {
        assert!(parse("45:u64").is_ok());
    }

    #[test]
    fn literals_intu128() {
        assert!(parse("45:u128").is_ok());
    }

    #[test]
    fn literals_float() {
        assert!(parse("45.").is_ok());
    }

    #[test]
    fn literals_float_exp() {
        assert!(parse("-45.10e8").is_ok());
    }

    #[test]
    fn literals_float32_exp() {
        assert!(parse("45.10E-8").is_ok());
    }

    #[test]
    fn literals_bool_true() {
        assert!(parse("true").is_ok());
    }

    #[test]
    fn literals_bool_true_up() {
        assert!(parse("True").is_ok());
    }

    #[test]
    fn literals_bool_false() {
        assert!(parse("false").is_ok());
    }

    #[test]
    fn literals_bool_false_up() {
        assert!(parse("False").is_ok());
    }

    #[test]
    fn literals_string_double_quotes() {
        assert!(parse("\"djedou Double Quotes\"").is_ok());
    }

    #[test]
    fn literals_string_raw() {
        assert!(parse(r##"r#"djedou Double Quotesr"#"##).is_ok());
    }

    #[test]
    fn literals_string_back() {
        assert!(parse("`djedou`").is_ok());
    }

    #[test]
    fn literals_char() {
        assert!(parse("'u'").is_ok());
    }

    #[test]
    fn literals_byte_hex() {
        assert!(parse("xFF").is_ok());
    }

    #[test]
    fn literals_byte_not() {
        assert!(parse("x00F").is_err());
    }

    #[test]
    fn function_declaration() {
        assert!(parse("fn fnName(n:i16) -> i16 {458;}").is_ok());
    }

    #[test]
    fn function_declaration_pub() {
        assert!(parse("pub fn fnName(n:i16) -> i16 {458;}").is_ok());
    }

    #[test]
    fn function_call() {
        assert!(parse("fnName(name)").is_ok());
    }

    #[test]
    fn function_call_by_value() {
        assert!(parse("fnName(49:i64)").is_ok());
    }

    #[test]
    fn variable_declaration() {
        assert!(parse("let djed: u8=36;").is_ok());
    }

    #[test]
    fn variable_declaration_mut() {
        assert!(parse("let mut djed:i32 = 78 ;").is_ok());
    }

    #[test]
    fn variable_declaration_mut_no_assign() {
        assert!(parse("let mut djed: u8;").is_ok());
    }

    #[test]
    fn operators_minus() {
        assert!(parse("47 - 48;").is_ok());
    }

    #[test]
    fn operators_plus() {
        assert!(parse("47 + 48;").is_ok());
    }

    #[test]
    fn operators_times() {
        assert!(parse("47 * 48;").is_ok());
    }

    #[test]
    fn operators_div() {
        assert!(parse("47 / 48;").is_ok());
    }

    #[test]
    fn operators_modulo() {
        assert!(parse("47 % 48;").is_ok());
    }

    #[test]
    fn assign_plus() {
        assert!(parse("47 += 48;").is_ok());
    }

    #[test]
    fn assign_assign() {
        assert!(parse("name = 48;").is_ok());
    }

    #[test]
    fn assign_minus() {
        assert!(parse("name -= 48;").is_ok());
    }

    #[test]
    fn assign_div() {
        assert!(parse("name /= name;").is_ok());
    }

    #[test]
    fn assign_modulo() {
        assert!(parse("name %= 48;").is_ok());
    }

    #[test]
    fn logic_and_err() {
        assert!(parse("name && 48;").is_ok());
    }

    #[test]
    fn logic_and() {
        assert!(parse("name && true;").is_ok());
    }

    #[test]
    fn logic_or() {
        assert!(parse("name || true;").is_ok());
    }

    #[test]
    fn logic_not() {
        assert!(parse("!true;").is_ok());
    }

    #[test]
    fn bitwise_and() {
        assert!(parse("name & value;").is_ok());
    }

    #[test]
    fn comparaison_equal() {
        assert!(parse("name == value;").is_ok());
    }

    
    #[test]
    fn literals_assignment() {
        assert!(parse("djedou = 48;").is_ok());
    }

    #[test]
    fn literals_assignment_with_type() {
        assert!(parse("djedou: u8 = 48;").is_ok());
    }

    

    
    #[test]
    fn literals_ident() {
        assert!(parse("leticia").is_ok());
    }

    #[test]
    fn literals_let_key_ident() {
        assert!(parse("let leticia").is_ok());
    }

    

    #[test]
    fn unary_expr() {
        let plus_one = parse("+1");
        assert!(plus_one.is_ok());
        assert_eq!(
            plus_one.clone().unwrap(),
            vec![Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Int(1))
            }]
        );
        assert_eq!(format!("{}", plus_one.unwrap()[0]), "+1");

        let neg_two = parse("-2");
        assert!(neg_two.is_ok());
        assert_eq!(
            neg_two.clone().unwrap(),
            vec![Node::UnaryExpr {
                op: Operator::Minus,
                child: Box::new(Node::Int(2))
            }]
        );
        assert_eq!(format!("{}", neg_two.unwrap()[0]), "-2");
    }
    #[test]
    fn binary_expr() {
        let sum = parse("1 + 2");
        assert!(sum.is_ok());
        assert_eq!(
            sum.clone().unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(Node::Int(1)),
                rhs: Box::new(Node::Int(2))
            }]
        );
        assert_eq!(format!("{}", sum.unwrap()[0]), "1 + 2");
        let minus = parse("1   -  \t  2");
        assert!(minus.is_ok());
        assert_eq!(
            minus.clone().unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Minus,
                lhs: Box::new(Node::Int(1)),
                rhs: Box::new(Node::Int(2))
            }]
        );
        assert_eq!(format!("{}", minus.unwrap()[0]), "1 - 2");
        // fails as there's no rhs:
        // let paran_sum = parse("(1 + 2)");
        // assert!(paran_sum.is_ok());
    }

    #[test]
    fn nested_expr() {
        fn test_expr(expected: &str, src: &str) {
            assert_eq!(
                expected,
                parse(src)
                    .unwrap()
                    .iter()
                    .fold(String::new(), |acc, arg| acc + &format!("{}", &arg))
            );
        }

        test_expr("1 + 2 + 3", "(1 + 2) + 3");
        test_expr("1 + 2 + 3", "1 + (2 + 3)");
        test_expr("1 + 2 + 3 + 4", "1 + (2 + (3 + 4))");
        test_expr("1 + 2 + 3 - 4", "(1 + 2) + (3 - 4)");
    }

    #[test]
    fn multiple_operators() {
        assert_eq!(
            parse("1+2+3").unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(Node::BinaryExpr {
                    op: Operator::Plus,
                    lhs: Box::new(Node::Int(1)),
                    rhs: Box::new(Node::Int(2)),
                }),
                rhs: Box::new(Node::Int(3)),
            }]
        )
    }*/
}
