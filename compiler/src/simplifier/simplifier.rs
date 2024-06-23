use crate::{
    renamer::plain,
    simplifier::simple,
    util::loc::{self, Located, Tagged},
};

use super::optimizations;

type Instrs = Vec<Tagged<simple::Statement>>;

pub fn run(fun: &plain::Module) -> simple::Module {
    let statements = fun
        .statements
        .iter()
        .map(|stmt| stmt.map(|stmt| simplify_top_level_stmt(&stmt)))
        .collect();
    let mut initial = simple::Module { statements };

    optimizations::run(&mut initial);

    return initial;
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
    let mut instrs = vec![];

    simplify_block(&mut state, &mut instrs, &fun.body.value);

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
                .attach(instrs)
                .to_tagged(&mut state.tag_map),
        },
        state.tag_map,
    );
}

fn simplify_block(
    state: &mut SimplifyFunImplState,
    instrs: &mut Instrs,
    stmt: &Vec<Located<plain::FunStatement>>,
) {
    for stmt in stmt.iter() {
        match &stmt.value {
            plain::FunStatement::Return(expr) => {
                let body = state.compile_expr(instrs, &expr.value);
                instrs.push(body.map(|i| simple::Statement::Return(*i)));
            }
            plain::FunStatement::LetDecl(l_decl) => {
                let tag = l_decl.location.to_tag(&mut state.tag_map);
                let body = state.compile_expr(instrs, &l_decl.value.value.value);
                instrs.push(
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
            plain::FunStatement::While(l_while) => {
                let tag = l_while.location.to_tag(&mut state.tag_map);
                let while_ = &l_while.value;

                let label = state.get_next_label();
                let mut loop_instrs = vec![];

                // Add the break condition
                let condition = state.compile_expr(&mut loop_instrs, &while_.condition.value);
                loop_instrs.push(condition.tag.attach(simple::Statement::If(simple::If {
                    condition,
                    then: condition.tag.attach(vec![]),
                    else_: condition.tag.attach(vec![
                        condition.tag.attach(simple::Statement::Break(label.clone())),
                    ]),
                })));

                // Add the rest of the body
                simplify_block(state, &mut loop_instrs, &while_.body.value);

                // Insert the loop
                let loop_ = simple::Loop {
                    label,
                    body: tag.attach(loop_instrs),
                };

                instrs.push(tag.attach(simple::Statement::Loop(loop_)));
            }
            plain::FunStatement::Assignment(l_assign) => {
                let tag = l_assign.location.to_tag(&mut state.tag_map);
                let assign = &l_assign.value;

                let body = state.compile_expr(instrs, &assign.value.value);
                instrs.push(
                    tag.attach(simple::Statement::Assignment(simple::Assignment {
                        target: assign
                            .id
                            .map(|id| simple::Identifier::Plain(id.widen()))
                            .to_tagged(&mut state.tag_map),
                        value: body.map(|b| simple::AssignmentValue::Identifier(*b)),
                    })),
                );
            }
            plain::FunStatement::InlineWasm(l_inline_wasm) => {
                let tag = l_inline_wasm.location.to_tag(&mut state.tag_map);
                let inline_wasm = &l_inline_wasm.value;

                let input_stack =
                    map_located_vec(&mut state.tag_map, &inline_wasm.input_stack, |x| {
                        simple::Identifier::Plain(x.widen())
                    });

                let output_stack =
                    map_located_vec(&mut state.tag_map, &inline_wasm.output_stack, |x| {
                        simple::Identifier::Plain(x.widen())
                    });

                instrs.push(
                    tag.attach(simple::Statement::InlineWasm(simple::InlineWasm {
                        input_stack,
                        output_stack,
                        wasm: inline_wasm.wasm.clone().to_tagged(&mut state.tag_map),
                    })),
                )
            }
            _ => todo!(),
        }
    }
}

struct SimplifyFunImplState {
    next_single_use_identifier: u32,
    next_loop_label: u32,
    tag_map: loc::TagMap,
}

impl SimplifyFunImplState {
    fn new() -> SimplifyFunImplState {
        SimplifyFunImplState {
            next_single_use_identifier: 1,
            tag_map: loc::TagMap::new(),
            next_loop_label: 1,
        }
    }

    fn get_single_use_identifier(&mut self) -> simple::SingleUseIdentifier {
        let id = simple::SingleUseIdentifier {
            id: self.next_single_use_identifier,
        };
        self.next_single_use_identifier += 1;
        return id;
    }

    fn compile_expr(
        &mut self,
        instrs: &mut Instrs,
        expr: &plain::Expr,
    ) -> Tagged<simple::Identifier> {
        match expr {
            plain::Expr::LitNumber(n) => {
                let id = self.get_single_use_identifier();
                let sid = simple::Identifier::SingleUse(id);

                let tag = self.tag_map.get_tag(n.location);

                let instr = simple::Assignment {
                    target: tag.attach(sid),
                    value: tag.attach(simple::AssignmentValue::LiteralNumber(n.value)),
                };

                instrs.push(tag.attach(simple::Statement::Assignment(instr)));

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
                    ps.push(self.compile_expr(instrs, &arg.value));
                }

                let target = self.get_single_use_identifier();
                let starget = simple::Identifier::SingleUse(target);

                instrs.push(
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

    fn get_next_label(&mut self) -> simple::Label {
        let id = self.next_loop_label;
        self.next_loop_label += 1;
        return simple::Label { id };
    }
}

// Utils

fn map_located_vec<T, K, F>(
    tag_map: &mut loc::TagMap,
    input: &Located<Vec<Located<T>>>,
    f: F,
) -> Tagged<Vec<Tagged<K>>>
where
    F: Fn(&T) -> K,
{
    input
        .map(|xs| {
            xs.iter()
                .map(|x| x.map(|x| f(x)).to_tagged(tag_map))
                .collect()
        })
        .to_tagged(tag_map)
}
