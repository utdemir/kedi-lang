use bimap::BiHashMap;
use std::collections::HashMap;

use crate::parser::syntax;
use crate::renamer::plain;
use crate::util::loc::SrcLoc;

use super::plain::Return;

pub fn rename(input: &syntax::Module) -> Result<plain::Module, Error> {
    let mut ret = vec![];

    for syn_input in input.statements.value.iter() {
        let input = rename_statement(&syn_input)?;
        ret.push(input);
    }

    Ok(plain::Module { statements: ret })
}

fn rename_statement(input: &syntax::TopLevelStmt) -> Result<plain::TopLevelStmt, Error> {
    match input {
        syntax::TopLevelStmt::FunDef(fun) => {
            let fun = fun.map_result(|fun| rename_function(fun))?;
            Ok(plain::TopLevelStmt::FunDef(fun))
        }
    }
}

fn rename_function(input: &syntax::FunDef) -> Result<plain::FunDef, Error> {
    let mut env = RenamerEnv::new();

    let params = input.params.map_result::<_, Error, _>(|ps| {
        ps.iter()
            .map(|p| {
                let pid = env.mk_new_local(p)?;
                // let predicate = p.predicate.map(|p| rename_expr(&mut env, &p));
                Ok(pid)
            })
            .collect()
    })?;

    let body = input.body.map_result(|body| {
        body.iter()
            .map(|stmt| rename_fun_statement(&mut env, &stmt))
            .collect()
    })?;

    let impl_loc = SrcLoc::enclosing(&params.location, &body.location);

    return Ok(plain::FunDef {
        name: input.name.clone(),
        implementation: impl_loc.attach(plain::FunImpl {
            params,
            body,
            preds: input.preds.map(|_x| vec![]),
        }),
        refs: env
            .globals
            .iter()
            .map(|(k, v)| (v.clone(), k.clone()))
            .collect(),
    });
}

fn rename_fun_statement(
    env: &mut RenamerEnv,
    input: &syntax::FunStmt,
) -> Result<plain::FunStmt, Error> {
    match input {
        syntax::FunStmt::LetDecl(decl) => {
            let ret = decl.map_result(|decl| {
                let pid = env.mk_new_local(&decl.name)?;
                let expr = rename_expr(env, &decl.value);
                Ok::<_, Error>(plain::LetDecl {
                    name: pid,
                    value: expr,
                })
            })?;

            Ok(plain::FunStmt::LetDecl(ret))
        }

        syntax::FunStmt::Return(ret) => {
            let ret = ret.map(|ret| Return(rename_expr(env, &ret.0)));
            Ok(plain::FunStmt::Return(ret))
        }

        syntax::FunStmt::While(while_stmt) => {
            let ret = while_stmt.map_result(|while_stmt| {
                let condition = rename_expr(env, &while_stmt.condition);
                let body = while_stmt.body.map_result(|body| {
                    body.into_iter()
                        .map(|stmt| rename_fun_statement(env, &stmt))
                        .collect::<Result<Vec<_>, _>>()
                })?;
                Ok::<_, Error>(plain::While { condition, body })
            })?;

            Ok(plain::FunStmt::While(ret))
        }

        syntax::FunStmt::Assignment(assignment) => {
            let ret = assignment.map_result(|assignment| {
                let id = assignment
                    .name
                    .map_result(|lhs| match env.resolve_local(&lhs) {
                        Some(x) => Ok(x),
                        None => Err(IdentifierNotFoundError {
                            identifier: assignment.name.clone(),
                        }),
                    })?;
                let value = rename_expr(env, &assignment.value);
                Ok::<_, Error>(plain::Assignment { id, value })
            })?;

            Ok(plain::FunStmt::Assignment(ret))
        }

        otherwise => unimplemented!("{:?}", otherwise),
    }
}

fn rename_expr(env: &mut RenamerEnv, input: &syntax::Expr) -> plain::Expr {
    match input {
        syntax::Expr::LitNum(x) => plain::Expr::LitNum(x.clone()),
        syntax::Expr::LitStr(x) => plain::Expr::LitStr(x.clone()),
        syntax::Expr::Ident(x) => plain::Expr::Ident(env.resolve(&x)),
        syntax::Expr::FunCall(x) => plain::Expr::FunCall(rename_fun_call(env, x)),
    }
}

fn rename_fun_call(env: &mut RenamerEnv, input: &syntax::FunCall) -> plain::FunCall {
    let name = input.name.map(|x| env.get_global(x));
    let args = input
        .args
        .map(|x| x.iter().map(|x| rename_expr(env, x)).collect());
    plain::FunCall { name, args }
}

struct RenamerEnv {
    next_local_id: u32,
    next_global_id: u32,

    locals: BiHashMap<syntax::Ident, plain::LocalIdent>,
    globals: BiHashMap<syntax::Ident, plain::GlobalIdent>,

    local_locs: HashMap<syntax::Ident, SrcLoc>,
}

impl RenamerEnv {
    fn new() -> Self {
        RenamerEnv {
            next_local_id: 0,
            next_global_id: 0,
            locals: BiHashMap::new(),
            globals: BiHashMap::new(),

            local_locs: HashMap::new(),
        }
    }

    fn mk_new_local(
        &mut self,
        input: &syntax::LIdent,
    ) -> Result<plain::LLocalIdent, DuplicateIdentifierError> {
        if let Some(_) = self.locals.get_by_left(&input.value) {
            return Err(DuplicateIdentifierError {
                error: input.clone(),
                original_loc: self.local_locs.get(&input.value).unwrap().clone(),
            });
        }

        let id = self.next_local_id;
        self.next_local_id += 1;
        let pid = plain::LocalIdent { id };
        self.locals.insert(input.value.clone(), pid.clone());
        self.local_locs.insert(input.value.clone(), input.location);
        Ok(input.location.attach(pid))
    }

    fn get_global(&mut self, input: &syntax::Ident) -> plain::GlobalIdent {
        match self.globals.get_by_left(input) {
            Some(x) => x.clone(),
            None => {
                let id = self.next_global_id;
                self.next_global_id += 1;
                let pid = plain::GlobalIdent { id };
                self.globals.insert(input.clone(), pid.clone());
                pid
            }
        }
    }

    fn resolve_local(&self, input: &syntax::Ident) -> Option<plain::LocalIdent> {
        self.locals.get_by_left(input).map(|x| x.clone())
    }

    fn resolve(&mut self, input: &syntax::LIdent) -> plain::Ident {
        match self.resolve_local(&input.value) {
            Some(x) => plain::Ident::Local(input.location.attach(x)),
            None => plain::Ident::Global(input.location.attach(self.get_global(&input.value))),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IdentifierNotFound(IdentifierNotFoundError),
    DuplicateIdentifier(DuplicateIdentifierError),
}

#[derive(Debug)]
pub struct IdentifierNotFoundError {
    pub identifier: syntax::LIdent,
}

impl From<DuplicateIdentifierError> for Error {
    fn from(e: DuplicateIdentifierError) -> Self {
        Error::DuplicateIdentifier(e)
    }
}

#[derive(Debug)]
pub struct DuplicateIdentifierError {
    pub error: syntax::LIdent,
    pub original_loc: SrcLoc,
}

impl From<IdentifierNotFoundError> for Error {
    fn from(e: IdentifierNotFoundError) -> Self {
        Error::IdentifierNotFound(e)
    }
}
