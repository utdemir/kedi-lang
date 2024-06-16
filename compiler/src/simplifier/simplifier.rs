use crate::{
    renamer::plain,
    simplifier::simple,
    util::loc::{self, Tagged},
};

pub fn run(fun: &plain::Module) -> simple::Module {
    let statements = fun
        .statements
        .iter()
        .map(|stmt| stmt.map(|stmt| simplify_top_level_stmt(&stmt)))
        .collect();
    return simple::Module { statements };
}

fn simplify_top_level_stmt(stmt: &plain::TopLevelStatement) -> simple::TopLevelStmt {
    match stmt {
        plain::TopLevelStatement::FunDecl(fun) => simple::TopLevelStmt::FunDecl(fun.map(|fun| {
            return simplify_fun_decl(&fun);
        })),
    }
}

fn simplify_fun_decl(fun: &plain::FunDecl) -> simple::FunDecl {
    let impl_loc = fun.implementation.location;

    let (simpl, tag_map) = simply_fun_impl(&fun.implementation.value);

    return simple::FunDecl {
        name: fun.name.clone(),
        implementation: impl_loc.attach(simpl),
        tag_map,
        refs: fun.refs.clone(),
    };
}

pub fn simply_fun_impl(fun: &plain::FunImpl) -> (simple::FunImpl, loc::TagMap) {
    let mut state = SimplifyFunImplState::new();

    for stmt in fun.body.value.iter() {
        let stmt_loc = stmt.location;
        match &stmt.value {
            plain::FunStatement::Return(expr) => {
                let body = state.push_expr(&expr.value);
                state
                    .instructions
                    .push(body.map(|i| simple::Statement::Return(*i)));
            }
            plain::FunStatement::LetDecl(l_decl) => {
                let tag = l_decl.location.to_tag(&mut state.tag_map);
                let body = state.push_expr(&l_decl.value.value.value);
                state.instructions.push(
                    tag.attach(simple::Statement::Assignment(simple::Assignment {
                        target: l_decl
                            .value
                            .name
                            .map(|id| simple::Identifier::Plain(id.widen()))
                            .to_tagged(&mut state.tag_map),
                        value: body.map(|b| simple::AssignmentValue::Identifier(*b)),
                    })),
                );
            }
            _ => unimplemented!("{:?}", stmt),
        }
    }

    return (
        simple::FunImpl {
            parameters: fun
                .parameters
                .map(|ps| {
                    ps.iter()
                        .map(|x| x.map(|x| x.name).to_tagged(&mut state.tag_map))
                        .collect()
                })
                .to_tagged(&mut state.tag_map),
            body: fun
                .body
                .location
                .attach(state.instructions)
                .to_tagged(&mut state.tag_map),
        },
        state.tag_map,
    );
}

struct SimplifyFunImplState {
    next_single_use_identifier: u32,
    instructions: Vec<Tagged<simple::Statement>>,
    tag_map: loc::TagMap,
}

impl SimplifyFunImplState {
    fn new() -> SimplifyFunImplState {
        SimplifyFunImplState {
            next_single_use_identifier: 1,
            instructions: Vec::new(),
            tag_map: loc::TagMap::new(),
        }
    }

    fn get_single_use_identifier(&mut self) -> simple::SingleUseIdentifier {
        let id = simple::SingleUseIdentifier {
            id: self.next_single_use_identifier,
        };
        self.next_single_use_identifier += 1;
        return id;
    }

    fn push_expr(&mut self, expr: &plain::Expr) -> Tagged<simple::Identifier> {
        match expr {
            plain::Expr::LitNumber(n) => {
                let id = self.get_single_use_identifier();
                let sid = simple::Identifier::SingleUse(id);

                let tag = self.tag_map.get_tag(n.location);

                let instr = simple::Assignment {
                    target: tag.attach(sid),
                    value: tag.attach(simple::AssignmentValue::LiteralNumber(n.value)),
                };

                self.instructions
                    .push(tag.attach(simple::Statement::Assignment(instr)));

                return tag.attach(sid);
            }
            plain::Expr::ValueIdentifier(id) => {
                return id
                    .map(|id| simple::Identifier::Plain(*id))
                    .to_tagged(&mut self.tag_map);
            }
            plain::Expr::FunCall(fun) => {
                let tag = fun.location.to_tag(&mut self.tag_map);

                let mut ps = vec![];
                for arg in fun.value.arguments.value.iter() {
                    ps.push(self.push_expr(&arg.value));
                }

                let target = self.get_single_use_identifier();
                let starget = simple::Identifier::SingleUse(target);

                self.instructions
                    .push(
                        tag.attach(simple::Statement::Assignment(simple::Assignment {
                            target: tag.attach(starget),
                            value: tag.attach(simple::AssignmentValue::Call(simple::Call {
                                fun_name: fun.value.name.to_tagged(&mut self.tag_map),
                                arguments: tag.attach(ps),
                            })),
                        })),
                    );

                return tag.attach(starget);
            }
            _ => todo!(),
        }
    }
}
