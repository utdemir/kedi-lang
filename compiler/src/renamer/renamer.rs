use bimap::BiHashMap;
use std::collections::HashMap;
use std::fmt::Debug;

use super::error::{DuplicateIdentifierError, Error, IdentifierNotFoundError};
use super::plain::Return;
use crate::parser::syntax;
use crate::renamer::plain;
use crate::util::ax::{ax, Ax};
use crate::util::loc::LocLike;

pub fn rename<L: LocLike + Debug>(input: &syntax::Module<L>) -> Result<plain::Module<L>, Error<L>> {
    let mut ret = vec![];

    for syn_input in input.statements.v.iter() {
        let input = rename_statement(syn_input)?;
        ret.push(input);
    }

    Ok(plain::Module { statements: ret })
}

fn rename_statement<L: LocLike + Debug>(
    input: &syntax::TopLevelStmt<L>,
) -> Result<plain::TopLevelStmt<L>, Error<L>> {
    match input {
        syntax::TopLevelStmt::FunDef(fun) => {
            let fun = fun.as_ref().map(|f| rename_function(&f)).transpose()?;
            Ok(plain::TopLevelStmt::FunDef(fun.clone_a()))
        }
    }
}

fn rename_function<L: LocLike + Debug>(
    input: &syntax::FunDef<L>,
) -> Result<plain::FunDef<L>, Error<L>> {
    let mut env = RenamerEnv::new();

    let params = input
        .params
        .as_ref()
        .map(|ps| {
            ps.iter()
                .map(|p| {
                    let pid = env.mk_new_local(p)?;
                    // let predicate = p.predicate.map(|p| rename_expr(&mut env, &p));
                    Ok::<_, Error<_>>(pid)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .clone_a();

    let body = input
        .body
        .as_ref()
        .map(|body| {
            body.iter()
                .map(|stmt| rename_fun_statement(&mut env, stmt))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .clone_a();

    return Ok(plain::FunDef {
        name: input.name.clone(),
        implementation: plain::FunImpl {
            params,
            body,
            preds: ax(input.preds.a.clone(), vec![]),
        },
        refs: env.globals.iter().map(|(k, v)| (*v, k.clone())).collect(),
    });
}

fn rename_fun_statement<L: LocLike + Debug>(
    env: &mut RenamerEnv<L>,
    input: &syntax::FunStmt<L>,
) -> Result<plain::FunStmt<L>, Error<L>> {
    match input {
        syntax::FunStmt::LetDecl(decl) => {
            let ret = decl
                .as_ref()
                .clone_a()
                .map(|decl| {
                    let pid = env.mk_new_local(&decl.name)?;
                    let expr = rename_expr(env, &decl.value);
                    Ok::<_, Error<_>>(plain::LetDecl {
                        name: pid,
                        value: expr,
                    })
                })
                .transpose()?;

            Ok(plain::FunStmt::LetDecl(ret))
        }

        syntax::FunStmt::Return(ret) => {
            let ret = ret
                .as_ref()
                .clone_a()
                .map(|ret| Return(rename_expr(env, &ret.0)));
            Ok(plain::FunStmt::Return(ret))
        }

        syntax::FunStmt::While(while_stmt) => {
            let ret = while_stmt
                .as_ref()
                .map(|while_stmt| {
                    let condition = rename_expr(env, &while_stmt.condition);
                    let body = while_stmt
                        .body
                        .as_ref()
                        .map(|body| {
                            body.iter()
                                .map(|stmt| rename_fun_statement(env, stmt))
                                .collect::<Result<Vec<_>, _>>()
                        })
                        .transpose()?
                        .clone_a();
                    Ok::<_, Error<_>>(plain::While { condition, body })
                })
                .transpose()?
                .clone_a();

            Ok(plain::FunStmt::While(ret))
        }

        syntax::FunStmt::Assignment(assignment) => {
            let ret = assignment
                .as_ref()
                .map(|assignment| {
                    let id = assignment
                        .name
                        .as_ref()
                        .map(|lhs| match env.resolve_local(&lhs) {
                            Some(x) => Ok(x),
                            None => Err(IdentifierNotFoundError {
                                identifier: assignment.name.clone(),
                            }),
                        })
                        .transpose()?
                        .clone_a();
                    let value = rename_expr(env, &assignment.value);
                    Ok::<_, Error<_>>(plain::Assignment { id, value })
                })
                .transpose()?
                .clone_a();

            Ok(plain::FunStmt::Assignment(ret))
        }

        syntax::FunStmt::If(if_stmt) => {
            let ret = if_stmt
                .as_ref()
                .map(|if_stmt| {
                    let condition = rename_expr(env, &if_stmt.condition);
                    let then = if_stmt
                        .then
                        .as_ref()
                        .map(|then| {
                            then.iter()
                                .map(|stmt| rename_fun_statement(env, stmt))
                                .collect::<Result<Vec<_>, _>>()
                        })
                        .transpose()?
                        .clone_a();
                    let else_ = if_stmt
                        .else_
                        .as_ref()
                        .map(|else_| {
                            else_
                                .as_ref()
                                .clone_a()
                                .map(|else_| {
                                    else_
                                        .iter()
                                        .map(|stmt| rename_fun_statement(env, stmt))
                                        .collect::<Result<Vec<_>, _>>()
                                })
                                .transpose()
                        })
                        .transpose()?;
                    Ok::<_, Error<_>>(plain::If {
                        condition,
                        then,
                        else_,
                    })
                })
                .transpose()?;

            Ok(plain::FunStmt::If(ret.clone_a()))
        }

        otherwise => unimplemented!("{:?}", otherwise),
    }
}

fn rename_expr<L: LocLike + Debug>(
    env: &mut RenamerEnv<L>,
    input: &syntax::Expr<L>,
) -> plain::Expr<L> {
    match input {
        syntax::Expr::LitNum(x) => plain::Expr::LitNum(x.clone()),
        syntax::Expr::LitStr(x) => plain::Expr::LitStr(x.clone()),
        syntax::Expr::Ident(x) => plain::Expr::Ident(env.resolve(x)),
        syntax::Expr::FunCall(x) => plain::Expr::FunCall(rename_fun_call(env, x)),
    }
}

fn rename_fun_call<L: LocLike + Debug>(
    env: &mut RenamerEnv<L>,
    input: &syntax::FunCall<L>,
) -> plain::FunCall<L> {
    let name = input.name.as_ref().map(|x| env.get_global(x)).clone_a();
    let args = input
        .args
        .as_ref()
        .map(|x| x.iter().map(|x| rename_expr(env, x)).collect::<Vec<_>>())
        .clone_a();
    plain::FunCall { name, args }
}

struct RenamerEnv<L> {
    next_local_id: u32,
    next_global_id: u32,

    locals: BiHashMap<syntax::Ident, plain::LocalIdent>,
    globals: BiHashMap<syntax::Ident, plain::UnresolvedIdent>,

    local_locs: HashMap<syntax::Ident, L>,
    _marker: std::marker::PhantomData<L>,
}

impl<L: LocLike + Debug> RenamerEnv<L> {
    fn new() -> Self {
        RenamerEnv {
            next_local_id: 0,
            next_global_id: 0,
            locals: BiHashMap::new(),
            globals: BiHashMap::new(),

            local_locs: HashMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    fn mk_new_local(
        &mut self,
        input: &Ax<L, syntax::Ident>,
    ) -> Result<Ax<L, plain::LocalIdent>, DuplicateIdentifierError<L>> {
        if self.locals.get_by_left(&input.v).is_some() {
            return Err(DuplicateIdentifierError {
                error: input.clone(),
                original_loc: self.local_locs.get(&input.v).unwrap().clone(),
            });
        }

        let id = self.next_local_id;
        self.next_local_id += 1;
        let pid = plain::LocalIdent { id };
        self.locals.insert(input.v.clone(), pid);
        self.local_locs.insert(input.v.clone(), input.a.clone());
        Ok(ax(input.a.clone(), pid))
    }

    fn get_global(&mut self, input: &syntax::Ident) -> plain::UnresolvedIdent {
        match self.globals.get_by_left(input) {
            Some(x) => *x,
            None => {
                let id = self.next_global_id;
                self.next_global_id += 1;
                let pid = plain::UnresolvedIdent { id };
                self.globals.insert(input.clone(), pid);
                pid
            }
        }
    }

    fn resolve_local(&self, input: &syntax::Ident) -> Option<plain::LocalIdent> {
        self.locals.get_by_left(input).copied()
    }

    fn resolve(&mut self, input: &Ax<L, syntax::Ident>) -> plain::Ident<L> {
        match self.resolve_local(&input.v) {
            Some(x) => plain::Ident::Local(ax(input.a.clone(), x)),
            None => plain::Ident::Global(ax(input.a.clone(), self.get_global(&input.v))),
        }
    }
}
