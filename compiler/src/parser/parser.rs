use core::panic;

use crate::parser::syntax::{self as syntax};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/kedi-lang.pest"]
pub struct KediParser;

pub fn parse(src: &str) -> Result<syntax::Module, String> {
    let successful_parse = KediParser::parse(Rule::module, src);
    let module = p_module(successful_parse.unwrap().into_iter().next().unwrap());
    Ok(module)
}

fn p_module(pair: Pair<Rule>) -> syntax::Module {
    let inner = get_inner_pairs(Rule::module, pair);

    let mut statements = vec![];
    for i in inner {
        match i.as_rule() {
            Rule::EOI => continue,
            Rule::stmt => statements.push(p_stmt(i)),
            otherwise => panic!("Unexpected rule in module: {:?}", otherwise),
        }
    }
    syntax::Module { statements }
}

fn p_stmt(pair: Pair<Rule>) -> syntax::Statement {
    let inner = get_inner_pair(Rule::stmt, pair);

    match inner.as_rule() {
        Rule::fun_decl => {
            let mut inner = inner.into_inner();

            let _name = inner.next().unwrap();
            let name = p_value_identifier(_name);

            let _parameters = inner.next().unwrap();
            let parameters = match _parameters.as_rule() {
                Rule::fun_arg_list => p_fun_arg_list(_parameters),
                otherwise => panic!(
                    "Unexpected rule in fun_decl (expecting fun_arg_list): {:?}",
                    otherwise
                ),
            };

            let _return_predicate = inner.next().unwrap();
            let return_predicate = match _return_predicate.as_rule() {
                Rule::expr => p_expr(_return_predicate),
                otherwise => panic!(
                    "Unexpected rule in fun_decl (expecting expr): {:?}",
                    otherwise
                ),
            };

            let _body = inner.next().unwrap();
            let body = match _body.as_rule() {
                Rule::block => {
                    let mut body = vec![];
                    for stmt in _body.into_inner() {
                        body.push(p_stmt(stmt));
                    }
                    body
                }
                otherwise => panic!(
                    "Unexpected rule in fun_decl (expecting block): {:?}",
                    otherwise
                ),
            };

            return syntax::Statement::FunDecl(syntax::FunDecl {
                name,
                parameters,
                return_predicate,
                body,
            });
        }
        Rule::r#return => {
            let _expr = inner.into_inner().next().unwrap();
            let expr = match _expr.as_rule() {
                Rule::expr => p_expr(_expr),
                otherwise => panic!(
                    "Unexpected rule in return (expecting expr): {:?}",
                    otherwise
                ),
            };

            return syntax::Statement::Return(expr);
        }
        Rule::inv => {
            let _expr = inner.into_inner().next().unwrap();
            let expr = match _expr.as_rule() {
                Rule::expr => p_expr(_expr),
                otherwise => panic!("Unexpected rule in inv (expecting expr): {:?}", otherwise),
            };

            return syntax::Statement::Inv(expr);
        }
        Rule::let_decl => {
            let mut inner = inner.into_inner();
            let name = p_value_identifier(inner.next().unwrap());

            let _predicate = inner.next().unwrap();
            let predicate = match _predicate.as_rule() {
                Rule::expr => p_expr(_predicate),
                otherwise => panic!(
                    "Unexpected rule in let_decl (expecting expr): {:?}",
                    otherwise
                ),
            };

            return syntax::Statement::LetDecl(name, predicate);
        }
        Rule::r#while => {
            let mut inner = inner.into_inner();

            let _predicate = inner.next().unwrap();
            let predicate = match _predicate.as_rule() {
                Rule::expr => p_expr(_predicate),
                otherwise => panic!("Unexpected rule in while (expecting expr): {:?}", otherwise),
            };

            let _body = inner.next().unwrap();
            let body = match _body.as_rule() {
                Rule::block => {
                    let mut body = vec![];
                    for stmt in _body.into_inner() {
                        body.push(p_stmt(stmt));
                    }
                    body
                }
                otherwise => panic!(
                    "Unexpected rule in while (expecting block): {:?}",
                    otherwise
                ),
            };

            return syntax::Statement::While(predicate, body);
        }
        Rule::assignment => {
            let mut inner = inner.into_inner();
            let name = p_value_identifier(inner.next().unwrap());

            let _predicate = inner.next().unwrap();
            let predicate = match _predicate.as_rule() {
                Rule::expr => p_expr(_predicate),
                otherwise => panic!(
                    "Unexpected rule in assignment (expecting expr): {:?}",
                    otherwise
                ),
            };

            return syntax::Statement::Assignment(name, predicate);
        }

        r => todo!("stmt: {:?}", r),
    }
}

fn p_value_identifier(pair: Pair<Rule>) -> syntax::Identifier {
    match pair.as_rule() {
        Rule::value_identifier => {
            let value = pair.as_str().to_string();
            debug_assert!(
                !value.contains(" "),
                "invalid value_identifier: {:?}",
                value
            );
            syntax::Identifier { name: value }
        }
        otherwise => panic!("Unexpected rule in value_identifier: {:?}", otherwise),
    }
}

fn p_fun_arg_list(pair: Pair<Rule>) -> Vec<syntax::FunParam> {
    let inner = get_inner_pairs(Rule::fun_arg_list, pair);

    let mut params = vec![];

    for i in inner {
        match i.as_rule() {
            Rule::fun_arg => params.push(p_fun_arg(i)),
            otherwise => panic!("Unexpected rule in fun_arg_list: {:?}", otherwise),
        }
    }

    params
}

fn p_fun_arg(pair: Pair<Rule>) -> syntax::FunParam {
    let mut inner = get_inner_pairs(Rule::fun_arg, pair);

    let name = p_value_identifier(inner.next().unwrap());
    let predicate = p_expr(inner.next().unwrap());

    syntax::FunParam { name, predicate }
}

fn p_expr(pair: Pair<Rule>) -> syntax::Expr {
    let inner = get_inner_pair(Rule::expr, pair);

    match inner.as_rule() {
        Rule::pl_expr => p_pl_expr(inner),
        Rule::op_expr => p_op_expr(inner),
        otherwise => panic!("Unexpected rule in expr: {:?}", otherwise),
    }
}

fn p_pl_expr(pair: Pair<Rule>) -> syntax::Expr {
    let inner = get_inner_pair(Rule::pl_expr, pair);

    match inner.as_rule() {
        Rule::value_identifier => syntax::Expr::ValueIdentifier(p_value_identifier(inner)),
        Rule::literal => p_literal(inner),
        Rule::func_call => syntax::Expr::FunCall(p_func_call(inner)),
        _ => todo!("p_pl_expr: {:?}", inner),
    }
}

fn p_op_expr(pair: Pair<Rule>) -> syntax::Expr {
    let mut inner = get_inner_pairs(Rule::op_expr, pair);

    let left = p_pl_expr(inner.next().unwrap());
    let op = inner.next().unwrap();
    let right = p_expr(inner.next().unwrap());

    assert!(inner.next().is_none());

    let id = op.into_inner().as_str().to_string();
    debug_assert!(!id.contains(" "));

    return syntax::Expr::Op(
        Box::new(left),
        syntax::Identifier { name: id },
        Box::new(right),
    );
}

fn p_literal(pair: Pair<Rule>) -> syntax::Expr {
    let inner = get_inner_pair(Rule::literal, pair);

    match inner.as_rule() {
        Rule::number_literal => syntax::Expr::LitNumber(inner.as_str().parse().unwrap()),
        Rule::string_literal => syntax::Expr::LitString(inner.as_str().to_string()),
        otherwise => panic!("Unexpected rule in literal: {:?}", otherwise),
    }
}

fn p_func_call(pair: Pair<Rule>) -> syntax::FunCall {
    let mut inner = get_inner_pairs(Rule::func_call, pair);

    let name = p_value_identifier(inner.next().unwrap());
    let arguments = p_func_call_arg_list(inner.next().unwrap());

    syntax::FunCall { name, arguments }
}

fn p_func_call_arg_list(pair: Pair<Rule>) -> Vec<syntax::Expr> {
    let inner = get_inner_pairs(Rule::func_call_arg_list, pair);

    let mut args = vec![];

    for i in inner {
        match i.as_rule() {
            Rule::expr => args.push(p_expr(i)),
            otherwise => panic!("Unexpected rule in fun_arg_list: {:?}", otherwise),
        }
    }

    args
}

// Utils

fn get_inner_pairs(expect_rule: Rule, pair: Pair<Rule>) -> Pairs<Rule> {
    match pair.as_rule() {
        r if r == expect_rule => pair.into_inner(),
        otherwise => panic!("Expecting rule {:?}, got {:?}", expect_rule, otherwise),
    }
}

fn get_inner_pair(expect_rule: Rule, pair: Pair<Rule>) -> Pair<Rule> {
    let mut pairs = get_inner_pairs(expect_rule, pair);
    if pairs.len() != 1 {
        panic!(
            "Expecting exactly one inner pair ([{:?}]), got {:?}",
            expect_rule, pairs,
        )
    }
    pairs.next().unwrap()
}
