use crate::renamer::renamed;
use crate::renamer::renamed::RenamedIdentifier;

use std::collections::{HashMap, LinkedList};

use crate::parser::syntax::{self, Identifier};
pub fn rename(module: syntax::Module) -> renamed::Module {
    let mut env = RenamerEnv::new();
    let mut statements = vec![];
    for stmt in module.statements {
        statements.push(rename_top_level_stmt(&mut env, stmt));
    }
    renamed::Module { statements }
}

struct RenamerEnv {
    root: HashMap<Identifier, RenamedIdentifier>,
    scopes: LinkedList<HashMap<Identifier, RenamedIdentifier>>,
    next_id: u32,
}

impl RenamerEnv {
    fn new() -> RenamerEnv {
        let mut root = HashMap::new();

        for (i, op) in [
            "Number",
            "Bool",
            "String",
            "Object",
            "hasAttribute",
            ">",
            "<",
            "&&",
            "||",
            "Array",
            "Any",
            "length",
            "result",
            "-",
            "+",
            "==",
            "<=",
        ]
        .iter()
        .enumerate()
        {
            root.insert(
                Identifier {
                    name: op.to_string(),
                },
                RenamedIdentifier { uid: i as u32 },
            );
        }

        RenamerEnv {
            root,
            scopes: LinkedList::new(),
            next_id: 100,
        }
    }

    // gets the current scope or root if topmost
    fn current_scope(&mut self) -> &mut HashMap<Identifier, RenamedIdentifier> {
        if let Some(scope) = self.scopes.back_mut() {
            scope
        } else {
            &mut self.root
        }
    }

    fn resolve_identifier(&self, id: &Identifier) -> Option<RenamedIdentifier> {
        for varmap in self.scopes.iter().rev() {
            if let Some(renamed_id) = varmap.get(id) {
                return Some(renamed_id.clone());
            }
        }
        self.root.get(id).cloned()
    }

    fn resolve_identifier_or_fail(&self, id: &Identifier) -> RenamedIdentifier {
        self.resolve_identifier(id)
            .expect(format!("Could not resolve identifier {:?}", id).as_str())
    }

    fn new_identifier(&mut self, id: Identifier) -> RenamedIdentifier {
        let curr_id = self.next_id.clone();

        let ret = {
            let curr = self.current_scope();

            let existing = curr.get(&id);
            if let Some(existing) = existing {
                panic!("Identifier {:?} already exists as {:?}", id, existing)
            }

            let renamed_id = RenamedIdentifier { uid: curr_id };
            curr.insert(id, renamed_id);

            renamed_id
        };

        self.next_id += 1;
        ret
    }

    fn with_scope<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.scopes.push_back(HashMap::new());
        let r = f(self);
        self.scopes.pop_back();
        r
    }
}

fn rename_stmts(env: &mut RenamerEnv, stmts: Vec<syntax::Statement>) -> Vec<renamed::Statement> {
    let mut renamed = vec![];
    for stmt in stmts {
        renamed.push(rename_stmt(env, stmt));
    }
    renamed
}

fn rename_top_level_stmt(
    env: &mut RenamerEnv,
    stmt: syntax::Statement,
) -> renamed::TopLevelStatement {
    match stmt {
        syntax::Statement::FunDecl(fun_decl) => {
            let fun_decl = rename_fun_decl(env, fun_decl);
            renamed::TopLevelStatement::FunDecl(fun_decl)
        }
        _ => unimplemented!(),
    }
}

fn rename_stmt(env: &mut RenamerEnv, stmt: syntax::Statement) -> renamed::Statement {
    match stmt {
        syntax::Statement::FunDecl(fun_decl) => {
            super::Statement::FunDecl(rename_fun_decl(env, fun_decl))
        }
        syntax::Statement::Return(expr) => {
            let expr = rename_expr(env, expr);
            renamed::Statement::Return(expr)
        }
        syntax::Statement::Inv(expr) => {
            let expr = rename_expr(env, expr);
            renamed::Statement::Inv(expr)
        }
        syntax::Statement::LetDecl(id, expr) => {
            let renamed_id = env.new_identifier(id);
            let expr = rename_expr(env, expr);
            renamed::Statement::LetDecl(renamed_id, expr)
        }
        syntax::Statement::While(expr, body) => {
            let expr = rename_expr(env, expr);
            let body = rename_stmts(env, body);
            renamed::Statement::While(expr, body)
        }
        syntax::Statement::Assignment(id, expr) => {
            let renamed_id = env.resolve_identifier_or_fail(&id);
            let expr = rename_expr(env, expr);
            renamed::Statement::Assignment(renamed_id, expr)
        }
    }
}

fn rename_fun_decl(env: &mut RenamerEnv, fun_decl: syntax::FunDecl) -> renamed::FunDecl {
    let name = env.new_identifier(fun_decl.name);

    env.with_scope(|env| {
        let mut parameters = Vec::new();

        for param in fun_decl.parameters {
            let parameter = renamed::FunParam {
                predicate: rename_expr(env, param.predicate),
                name: env.new_identifier(param.name),
            };
            parameters.push(parameter);
        }

        let return_predicate = rename_expr(env, fun_decl.return_predicate);

        let body = rename_stmts(env, fun_decl.body);

        renamed::FunDecl {
            name,
            parameters,
            return_predicate,
            body,
        }
    })
}

fn rename_expr(env: &RenamerEnv, expr: syntax::Expr) -> renamed::Expr {
    match expr {
        syntax::Expr::LitNumber(n) => renamed::Expr::LitNumber(n),
        syntax::Expr::LitString(s) => renamed::Expr::LitString(s),
        syntax::Expr::ValueIdentifier(id) => {
            let renamed_id = env.resolve_identifier_or_fail(&id);
            renamed::Expr::ValueIdentifier(renamed_id)
        }
        syntax::Expr::FunCall(fun_call) => {
            let name = env.resolve_identifier_or_fail(&fun_call.name);
            let arguments = fun_call
                .arguments
                .into_iter()
                .map(|expr| rename_expr(env, expr))
                .collect();
            renamed::Expr::FunCall(renamed::FunCall { name, arguments })
        }
        syntax::Expr::Op(lhs, op, rhs) => {
            let lhs = Box::new(rename_expr(env, *lhs));
            let rhs = Box::new(rename_expr(env, *rhs));
            let op = env.resolve_identifier_or_fail(&op);
            renamed::Expr::Op(lhs, op, rhs)
        }
    }
}
