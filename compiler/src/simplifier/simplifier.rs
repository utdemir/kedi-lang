use super::simple;
use crate::renamer::plain;

pub fn run(fun: &plain::Module) -> simple::Module {
    let mut statements = Vec::new();
    for stmt in fun.statements.iter() {
        statements.push(simplify_top_level_stmt(stmt));
    }

    return simple::Module { statements };
}

fn simplify_top_level_stmt(stmt: &plain::TopLevelStatement) -> simple::TopLevelStmt {
    match stmt {
        plain::TopLevelStatement::FunDecl(fun) => {
            simple::TopLevelStmt::FunDecl(simplify_fun_decl(fun))
        }
    }
}

fn simplify_fun_decl(fun: &plain::FunDecl) -> simple::FunDecl {
    return simple::FunDecl {
        name: fun.name.clone(),
        implementation: simply_fun_impl(&fun.implementation),
        refs: fun.refs.clone(),
    };
}

pub fn simply_fun_impl(fun: &plain::FunImpl) -> simple::FunImpl {
    let mut state = SimplifyFunImplState::new();

    for stmt in fun.body.iter() {
        match &stmt {
            plain::FunStatement::Return(expr) => {
                let body = state.push_expr(expr);
                state.instructions.push(simple::Statement::Return(body));
            }
            plain::FunStatement::LetDecl(id, expr) => {
                let body = state.push_expr(expr);
                state
                    .instructions
                    .push(simple::Statement::Assignment(simple::Assignment {
                        target: simple::Identifier::Plain(id.widen()),
                        value: simple::AssignmentValue::Identifier(body),
                    }));
            }
            _ => unimplemented!("{:?}", stmt),
        }
    }

    return simple::FunImpl {
        parameters: fun.parameters.iter().map(|x| x.name).collect(),
        body: state.instructions,
    };
}

struct SimplifyFunImplState {
    next_single_use_identifier: u32,
    instructions: Vec<simple::Statement>,
}

impl SimplifyFunImplState {
    fn new() -> SimplifyFunImplState {
        SimplifyFunImplState {
            next_single_use_identifier: 1,
            instructions: Vec::new(),
        }
    }

    fn get_single_use_identifier(&mut self) -> simple::SingleUseIdentifier {
        let id = simple::SingleUseIdentifier {
            id: self.next_single_use_identifier,
        };
        self.next_single_use_identifier += 1;
        return id;
    }

    fn push_expr(&mut self, expr: &plain::Expr) -> simple::Identifier {
        match expr {
            plain::Expr::LitNumber(n) => {
                let id = self.get_single_use_identifier();
                let sid = simple::Identifier::SingleUse(id);

                let val = simple::Assignment {
                    target: sid,
                    value: simple::AssignmentValue::LiteralNumber(*n),
                };
                self.instructions.push(simple::Statement::Assignment(val));
                return sid;
            }
            plain::Expr::ValueIdentifier(id) => {
                return simple::Identifier::Plain(*id);
            }
            &plain::Expr::FunCall(ref fun) => {
                let mut ps = vec![];
                for arg in fun.arguments.iter() {
                    ps.push(self.push_expr(arg));
                }

                let target = self.get_single_use_identifier();
                let starget = simple::Identifier::SingleUse(target);

                self.instructions
                    .push(simple::Statement::Assignment(simple::Assignment {
                        target: starget,
                        value: simple::AssignmentValue::Call(fun.name, ps),
                    }));

                return starget;
            }
            _ => todo!(),
        }
    }
}
