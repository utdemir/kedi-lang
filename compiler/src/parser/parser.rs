use core::panic;

use crate::parser::syntax;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::util::loc::*;

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
            Rule::top_level_stmt => statements.push(p_top_level_stmt(i)),
            otherwise => panic!("Unexpected rule in module: {:?}", otherwise),
        };
    }
    syntax::Module { statements }
}

fn p_top_level_stmt(pair: Pair<Rule>) -> Located<syntax::TopLevelStatement> {
    let top_loc = pair_to_loc(&pair);
    let inner = get_inner_pair(Rule::top_level_stmt, pair);

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
            let body_loc = pair_to_loc(&_body);
            let body = match _body.as_rule() {
                Rule::block => {
                    let mut body = vec![];
                    for stmt in _body.into_inner() {
                        body.push(p_fun_stmt(stmt));
                    }
                    body
                }
                otherwise => panic!(
                    "Unexpected rule in fun_decl (e∂xpecting block): {:?}",
                    otherwise
                ),
            };

            return top_loc.attach(syntax::TopLevelStatement::FunDecl(top_loc.attach(
                syntax::FunDecl {
                    name,
                    parameters,
                    return_predicate: Some(return_predicate),
                    body: body_loc.attach(body),
                },
            )));
        }
        otherwise => panic!("Unexpected rule in module: {:?}", otherwise),
    }
}

fn p_fun_stmt(pair: Pair<Rule>) -> Located<syntax::FunStatement> {
    let top_loc = pair_to_loc(&pair);
    let inner = get_inner_pair(Rule::fun_stmt, pair);

    let stmt = match inner.as_rule() {
        Rule::r#return => {
            let _expr = inner.into_inner().next().unwrap();
            let loc = pair_to_loc(&_expr);
            let expr = match _expr.as_rule() {
                Rule::expr => p_expr(_expr),
                otherwise => panic!(
                    "Unexpected rule in return (expecting expr): {:?}",
                    otherwise
                ),
            };

            syntax::FunStatement::Return(loc.attach(syntax::Return { value: expr }))
        }
        Rule::inv => {
            let _expr = inner.into_inner().next().unwrap();
            let loc = pair_to_loc(&_expr);
            let expr = match _expr.as_rule() {
                Rule::expr => p_expr(_expr),
                otherwise => panic!("Unexpected rule in inv (expecting expr): {:?}", otherwise),
            };

            syntax::FunStatement::Inv(loc.attach(syntax::Inv { value: expr }))
        }
        Rule::let_decl => {
            let mut inner = inner.into_inner();
            let loc = pairs_to_loc(&inner);

            let name = p_value_identifier(inner.next().unwrap());

            let _value = inner.next().unwrap();
            let value = match _value.as_rule() {
                Rule::expr => p_expr(_value),
                otherwise => panic!(
                    "Unexpected rule in let_decl (expecting expr): {:?}",
                    otherwise
                ),
            };

            syntax::FunStatement::LetDecl(loc.attach(syntax::LetDecl { name, value }))
        }
        Rule::r#while => {
            let mut inner = inner.into_inner();
            let loc = pairs_to_loc(&inner);

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
                        body.push(p_fun_stmt(stmt));
                    }
                    body
                }
                otherwise => panic!(
                    "Unexpected rule in while (expecting block): {:?}",
                    otherwise
                ),
            };

            syntax::FunStatement::While(loc.attach(syntax::While {
                condition: predicate,
                body: body,
            }))
        }
        Rule::assignment => {
            let mut inner = inner.into_inner();
            let loc = pairs_to_loc(&inner);

            let name = p_value_identifier(inner.next().unwrap());

            let _value = inner.next().unwrap();
            let value = match _value.as_rule() {
                Rule::expr => p_expr(_value),
                otherwise => panic!(
                    "Unexpected rule in assignment (expecting expr): {:?}",
                    otherwise
                ),
            };

            syntax::FunStatement::Assignment(loc.attach(syntax::Assignment { name, value }))
        }

        r => todo!("stmt: {:?}", r),
    };

    top_loc.attach(stmt)
}

fn p_value_identifier(pair: Pair<Rule>) -> Located<syntax::Identifier> {
    match pair.as_rule() {
        Rule::value_identifier => {
            let value = pair.as_str().to_string();
            let loc = pair_to_loc(&pair);
            debug_assert!(
                !value.contains(" "),
                "invalid value_identifier: {:?}",
                value
            );
            loc.attach(syntax::Identifier { name: value })
        }
        otherwise => panic!("Unexpected rule in value_identifier: {:?}", otherwise),
    }
}

fn p_fun_arg_list(pair: Pair<Rule>) -> Located<Vec<Located<syntax::FunParam>>> {
    let loc = pair_to_loc(&pair);
    let inner = get_inner_pairs(Rule::fun_arg_list, pair);

    let mut params = vec![];

    for i in inner {
        match i.as_rule() {
            Rule::fun_arg => params.push(p_fun_arg(i)),
            otherwise => panic!("Unexpected rule in fun_arg_list: {:?}", otherwise),
        }
    }

    loc.attach(params)
}

fn p_fun_arg(pair: Pair<Rule>) -> Located<syntax::FunParam> {
    let loc = pair_to_loc(&pair);
    let mut inner = get_inner_pairs(Rule::fun_arg, pair);

    let name = p_value_identifier(inner.next().unwrap());
    let predicate = p_expr(inner.next().unwrap());

    loc.attach(syntax::FunParam { name, predicate })
}

fn p_expr(pair: Pair<Rule>) -> Located<syntax::Expr> {
    let inner = get_inner_pair(Rule::expr, pair);

    match inner.as_rule() {
        Rule::pl_expr => p_pl_expr(inner),
        Rule::op_expr => p_op_expr(inner),
        otherwise => panic!("Unexpected rule in expr: {:?}", otherwise),
    }
}

fn p_pl_expr(pair: Pair<Rule>) -> Located<syntax::Expr> {
    let loc = pair_to_loc(&pair);
    let inner = get_inner_pair(Rule::pl_expr, pair);

    match inner.as_rule() {
        Rule::value_identifier => {
            loc.attach(syntax::Expr::ValueIdentifier(p_value_identifier(inner)))
        }
        Rule::literal => p_literal(inner),
        Rule::func_call => loc.attach(syntax::Expr::FunCall(p_func_call(inner))),
        _ => todo!("p_pl_expr: {:?}", inner),
    }
}

fn p_op_expr(pair: Pair<Rule>) -> Located<syntax::Expr> {
    let loc = pair_to_loc(&pair);
    let mut inner = get_inner_pairs(Rule::op_expr, pair);

    let left = p_pl_expr(inner.next().unwrap());
    let op = inner.next().unwrap();
    let right = p_expr(inner.next().unwrap());

    assert!(inner.next().is_none());

    let id = op.into_inner().as_str().to_string();
    debug_assert!(!id.contains(" "));

    return loc.attach(syntax::Expr::Op(
        Box::new(left),
        syntax::Identifier { name: id },
        Box::new(right),
    ));
}

fn p_literal(pair: Pair<Rule>) -> Located<syntax::Expr> {
    let loc = pair_to_loc(&pair);
    let inner = get_inner_pair(Rule::literal, pair);

    let ret = match inner.as_rule() {
        Rule::number_literal => {
            syntax::Expr::LitNumber(loc.attach(inner.as_str().parse().unwrap()))
        }
        Rule::string_literal => syntax::Expr::LitString(loc.attach(inner.as_str().to_string())),
        otherwise => panic!("Unexpected rule in literal: {:?}", otherwise),
    };

    loc.attach(ret)
}

fn p_func_call(pair: Pair<Rule>) -> Located<syntax::FunCall> {
    let loc = pair_to_loc(&pair);
    let mut inner = get_inner_pairs(Rule::func_call, pair);

    let name = p_value_identifier(inner.next().unwrap());
    let arguments = p_func_call_arg_list(inner.next().unwrap());

    loc.attach(syntax::FunCall { name, arguments })
}

fn p_func_call_arg_list(pair: Pair<Rule>) -> Located<Vec<Located<syntax::Expr>>> {
    let loc = pair_to_loc(&pair);
    let inner = get_inner_pairs(Rule::func_call_arg_list, pair);

    let mut args = vec![];

    for i in inner {
        match i.as_rule() {
            Rule::expr => args.push(p_expr(i)),
            otherwise => panic!("Unexpected rule in fun_arg_list: {:?}", otherwise),
        }
    }

    loc.attach(args)
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

fn pairs_to_loc(pairs: &Pairs<Rule>) -> SrcLoc {
    let start = pairs.clone().next().unwrap().as_span().start_pos().pos();
    let end = pairs.clone().last().unwrap().as_span().end_pos().pos();
    let len = end - start;

    SrcLoc::Known(Span {
        start: Pos {
            offset: start as usize,
        },
        length: len as usize,
    })
}

fn pair_to_loc(pair: &Pair<Rule>) -> SrcLoc {
    let span = pair.as_span();
    span_to_loc(&span)
}

fn span_to_loc(span: &pest::Span) -> SrcLoc {
    let start = span.start_pos().pos();
    let end = span.end_pos().pos();
    let len = end - start;

    SrcLoc::Known(Span {
        start: Pos {
            offset: start as usize,
        },
        length: len as usize,
    })
}
