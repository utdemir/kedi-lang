use bimap::BiHashMap;

use crate::parser::syntax;
use crate::renamer::plain;

use std::collections::HashMap;

pub fn rename(input: &syntax::Module) -> plain::Module {
    let mut ret = vec![];

    for input in input.statements.iter() {
        let input = rename_statement(input);
        ret.push(input);
    }

    plain::Module { statements: ret }
}

fn rename_statement(input: &syntax::TopLevelStatement) -> plain::TopLevelStatement {
    match input {
        syntax::TopLevelStatement::FunDecl(fun) => {
            let fun = rename_function(fun);
            plain::TopLevelStatement::FunDecl(fun)
        }
    }
}

fn rename_function(input: &syntax::FunDecl) -> plain::FunDecl {
    let mut env = RenamerEnv::new();

    let mut parameters = vec![];
    let mut body = vec![];

    for param in input.parameters.iter() {
        let pid = env.mk_new_local(&param.name).unwrap();
        parameters.push(plain::FunParam {
            name: pid,
            predicate: None,
        });
    }

    for stmt in input.body.iter() {
        let stmt = rename_fun_statement(&mut env, &stmt);
        body.push(stmt);
    }

    return plain::FunDecl {
        name: input.name.clone(),
        implementation: plain::FunImpl {
            parameters,
            body,
            return_predicate: None,
        },
        refs: HashMap::new(),
    };
}

fn rename_fun_statement(env: &mut RenamerEnv, input: &syntax::FunStatement) -> plain::FunStatement {
    match input {
        syntax::FunStatement::LetDecl(id, expr) => {
            let pid = env.mk_new_local(id).unwrap();
            let expr = rename_expr(env, expr);
            plain::FunStatement::LetDecl(pid, expr)
        }

        syntax::FunStatement::Return(expr) => {
            let expr = rename_expr(env, expr);
            plain::FunStatement::Return(expr)
        }

        otherwise => unimplemented!("{:?}", otherwise),
    }
}

fn rename_expr(env: &mut RenamerEnv, input: &syntax::Expr) -> plain::Expr {
    match input {
        syntax::Expr::LitNumber(x) => plain::Expr::LitNumber(*x),
        syntax::Expr::LitString(x) => plain::Expr::LitString(x.clone()),
        syntax::Expr::ValueIdentifier(x) => plain::Expr::ValueIdentifier(env.resolve(x)),
        syntax::Expr::FunCall(x) => plain::Expr::FunCall(rename_fun_call(env, x)),
        otherwise => unimplemented!("rename_expr: {:?}", otherwise),
    }
}

fn rename_fun_call(env: &mut RenamerEnv, input: &syntax::FunCall) -> plain::FunCall {
    let name = env.get_global(&input.name);
    let arguments = input
        .arguments
        .iter()
        .map(|x| rename_expr(env, x))
        .collect();
    plain::FunCall { name, arguments }
}

struct RenamerEnv {
    next_local_id: u32,
    next_global_id: u32,

    locals: BiHashMap<syntax::Identifier, plain::LocalIdentifier>,
    globals: BiHashMap<syntax::Identifier, plain::GlobalIdentifier>,
}

impl RenamerEnv {
    fn new() -> Self {
        RenamerEnv {
            next_local_id: 0,
            next_global_id: 0,
            locals: BiHashMap::new(),
            globals: BiHashMap::new(),
        }
    }

    fn mk_new_local(&mut self, input: &syntax::Identifier) -> Result<plain::LocalIdentifier, ()> {
        if let Some(_) = self.locals.get_by_left(input) {
            return Err(());
        }

        let id = self.next_local_id;
        self.next_local_id += 1;
        let pid = plain::LocalIdentifier { id };
        self.locals.insert(input.clone(), pid.clone());
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

    fn resolve(&mut self, input: &syntax::Identifier) -> plain::Identifier {
        match self.locals.get_by_left(input) {
            Some(x) => plain::Identifier::Local(x.clone()),
            None => plain::Identifier::Global(self.get_global(input)),
        }
    }
}
