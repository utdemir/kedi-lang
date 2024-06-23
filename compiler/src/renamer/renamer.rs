use bimap::BiHashMap;
use std::collections::HashMap;

use crate::parser::syntax;
use crate::renamer::plain;
use crate::util::loc::{Located, SrcLoc};

pub fn rename(input: &syntax::Module) -> Result<plain::Module, Error> {
    let mut ret = vec![];

    for syn_input in input.statements.iter() {
        let input = syn_input.map_result(|i| rename_statement(&i))?;
        ret.push(input);
    }

    Ok(plain::Module { statements: ret })
}

fn rename_statement(input: &syntax::TopLevelStatement) -> Result<plain::TopLevelStatement, Error> {
    match input {
        syntax::TopLevelStatement::FunDecl(fun) => {
            let fun = fun.map_result(|fun| rename_function(&fun))?;
            Ok(plain::TopLevelStatement::FunDecl(fun))
        }
    }
}

fn rename_function(input: &syntax::FunDecl) -> Result<plain::FunDecl, Error> {
    let mut env = RenamerEnv::new();

    let parameters = input.parameters.map_result::<_, Error, _>(|ps| {
        ps.iter()
            .map(|p| {
                p.map_result(|p| {
                    let pid = env.mk_new_local(&p.name)?;
                    // let predicate = p.predicate.map(|p| rename_expr(&mut env, &p));
                    Ok(plain::FunParam {
                        name: pid,
                        predicate: None,
                    })
                })
            })
            .collect()
    })?;

    let body = input.body.map_result(|body| {
        body.iter()
            .map(|stmt| stmt.map_result(|stmt| rename_fun_statement(&mut env, &stmt)))
            .collect()
    })?;

    let impl_loc = SrcLoc::enclosing(&parameters.location, &body.location);

    return Ok(plain::FunDecl {
        name: input.name.clone(),
        implementation: impl_loc.attach(plain::FunImpl {
            parameters,
            body,
            return_predicate: None,
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
    input: &syntax::FunStatement,
) -> Result<plain::FunStatement, Error> {
    match input {
        syntax::FunStatement::LetDecl(decl) => {
            let ret = decl.map_result(|decl| {
                let pid = decl.name.location.attach(env.mk_new_local(&decl.name)?);
                let expr = decl.value.map(|v| rename_expr(env, v));
                Ok::<_, Error>(plain::LetDecl {
                    name: pid,
                    value: expr,
                })
            })?;

            Ok(plain::FunStatement::LetDecl(ret))
        }

        syntax::FunStatement::Return(ret) => {
            let ret = ret.map(|ret| rename_expr(env, &ret.value.value));
            Ok(plain::FunStatement::Return(ret))
        }

        syntax::FunStatement::While(while_stmt) => {
            let ret = while_stmt.map_result(|while_stmt| {
                let condition = while_stmt.condition.map(|c| rename_expr(env, &c));
                let body = while_stmt.body.map_result(|body| {
                    body.into_iter()
                        .map(|stmt| stmt.map_result(|stmt| rename_fun_statement(env, &stmt)))
                        .collect::<Result<Vec<_>, _>>()
                })?;
                Ok::<_, Error>(plain::While { condition, body })
            })?;

            Ok(plain::FunStatement::While(ret))
        }

        syntax::FunStatement::Assignment(assignment) => {
            let ret = assignment.map_result(|assignment| {
                let id = assignment
                    .name
                    .map_result(|lhs| match env.resolve_local(&lhs) {
                        Some(x) => Ok(x),
                        None => Err(IdentifierNotFoundError {
                            identifier: assignment.name.clone(),
                        }),
                    })?;
                let value = assignment.value.map(|rhs| rename_expr(env, &rhs));
                Ok::<_, Error>(plain::Assignment { id, value })
            })?;

            Ok(plain::FunStatement::Assignment(ret))
        }

        syntax::FunStatement::InlineWasm(inline_wasm) => {
            let ret = inline_wasm.map_result::<_, IdentifierNotFoundError, _>(|inline_wasm| {
                let inputs = inline_wasm.input_stack.map_result(|xs| {
                    xs.iter()
                        .map(|l_x| {
                            l_x.map_result(|x| {
                                env.resolve_local(&x).ok_or(IdentifierNotFoundError {
                                    identifier: l_x.clone(),
                                })
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?;

                let outputs = inline_wasm.output_stack.map_result(|xs| {
                    xs.iter()
                        .map(|l_x| {
                            l_x.map_result(|x| {
                                env.resolve_local(&x).ok_or(IdentifierNotFoundError {
                                    identifier: l_x.clone(),
                                })
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?;

                Ok(plain::InlineWasm {
                    input_stack: inputs,
                    output_stack: outputs,
                    wasm: inline_wasm.wasm.clone(),
                })
            })?;

            Ok(plain::FunStatement::InlineWasm(ret))
        }

        otherwise => unimplemented!("{:?}", otherwise),
    }
}

fn rename_expr(env: &mut RenamerEnv, input: &syntax::Expr) -> plain::Expr {
    match input {
        syntax::Expr::LitNumber(x) => plain::Expr::LitNumber(*x),
        syntax::Expr::LitString(x) => plain::Expr::LitString(x.clone()),
        syntax::Expr::ValueIdentifier(x) => plain::Expr::ValueIdentifier(x.map(|x| env.resolve(x))),
        syntax::Expr::FunCall(x) => plain::Expr::FunCall(x.map(|x| rename_fun_call(env, x))),
        syntax::Expr::Op(lhs, op, rhs) => {
            let loc = SrcLoc::all_enclosing(&[lhs.location, op.location, rhs.location]);
            let fun = syntax::FunCall {
                name: op.clone(),
                arguments: SrcLoc::enclosing(&lhs.location, &rhs.location)
                    .attach(vec![*lhs.to_owned(), *rhs.to_owned()]),
            };
            plain::Expr::FunCall(loc.attach(rename_fun_call(env, &fun)))
        }
    }
}

fn rename_fun_call(env: &mut RenamerEnv, input: &syntax::FunCall) -> plain::FunCall {
    let name = input.name.map(|x| env.get_global(x));
    let arguments = input
        .arguments
        .map(|x| x.iter().map(|x| x.map(|x| rename_expr(env, x))).collect());
    plain::FunCall { name, arguments }
}

struct RenamerEnv {
    next_local_id: u32,
    next_global_id: u32,

    locals: BiHashMap<syntax::Identifier, plain::LocalIdentifier>,
    globals: BiHashMap<syntax::Identifier, plain::GlobalIdentifier>,

    local_locs: HashMap<syntax::Identifier, SrcLoc>,
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
        input: &Located<syntax::Identifier>,
    ) -> Result<plain::LocalIdentifier, DuplicateIdentifierError> {
        if let Some(_) = self.locals.get_by_left(&input.value) {
            return Err(DuplicateIdentifierError {
                error: input.clone(),
                original_loc: self.local_locs.get(&input.value).unwrap().clone(),
            });
        }

        let id = self.next_local_id;
        self.next_local_id += 1;
        let pid = plain::LocalIdentifier { id };
        self.locals.insert(input.value.clone(), pid.clone());
        self.local_locs.insert(input.value.clone(), input.location);
        Ok(pid)
    }

    fn get_global(&mut self, input: &syntax::Identifier) -> plain::GlobalIdentifier {
        match self.globals.get_by_left(input) {
            Some(x) => x.clone(),
            None => {
                let id = self.next_global_id;
                self.next_global_id += 1;
                let pid = plain::GlobalIdentifier { id };
                self.globals.insert(input.clone(), pid.clone());
                pid
            }
        }
    }

    fn resolve_local(&self, input: &syntax::Identifier) -> Option<plain::LocalIdentifier> {
        self.locals.get_by_left(input).map(|x| x.clone())
    }

    fn resolve(&mut self, input: &syntax::Identifier) -> plain::Identifier {
        match self.resolve_local(input) {
            Some(x) => plain::Identifier::Local(x.clone()),
            None => plain::Identifier::Global(self.get_global(input)),
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
    pub identifier: Located<syntax::Identifier>,
}

impl From<DuplicateIdentifierError> for Error {
    fn from(e: DuplicateIdentifierError) -> Self {
        Error::DuplicateIdentifier(e)
    }
}

#[derive(Debug)]
pub struct DuplicateIdentifierError {
    pub error: Located<syntax::Identifier>,
    pub original_loc: SrcLoc,
}

impl From<IdentifierNotFoundError> for Error {
    fn from(e: IdentifierNotFoundError) -> Self {
        Error::IdentifierNotFound(e)
    }
}
