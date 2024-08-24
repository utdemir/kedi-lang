use crate::{
    renamer::plain,
    simplifier::simple,
    util::loc::{self, Located, Tagged},
};

use super::optimizations;

type Instrs = Vec<simple::FunStmt>;

pub fn run(fun: &plain::Module) -> simple::Module {
    let statements = fun
        .statements
        .iter()
        .map(|stmt| simplify_top_level_stmt(&stmt))
        .collect();
    let mut initial = simple::Module { statements };

    optimizations::run(&mut initial);

    return initial;
}

fn simplify_top_level_stmt(stmt: &plain::TopLevelStmt) -> simple::TopLevelStmt {
    match stmt {
        plain::TopLevelStmt::FunDef(fun) => {
            simple::TopLevelStmt::FunDecl(fun.map(simplify_fun_decl))
        }
    }
}

fn simplify_fun_decl(fun: &plain::FunDef) -> simple::FunDecl {
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
                .params
                .map(|ps| ps.iter().map(|x| x.to_tagged(&mut state.tag_map)).collect())
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
    stmt: &Vec<plain::FunStmt>,
) {
    for stmt in stmt.iter() {
        match &stmt {
            plain::FunStmt::Return(expr) => {
                let body = state.compile_expr(instrs, &expr.value.0);
                instrs.push(simple::FunStmt::Return(body));
            }
            plain::FunStmt::LetDecl(l_decl) => {
                let tag = l_decl.location.to_tag(&mut state.tag_map);
                let body = state.compile_expr(instrs, &l_decl.value.value);
                instrs.push(simple::FunStmt::Assignment(tag.attach(
                    simple::Assignment {
                        target: state.widen_plain_ident(&l_decl.value.name.widen().value),
                        value: simple::AssignmentValue::Ident(body),
                    },
                )));
            }
            plain::FunStmt::While(l_while) => {
                let tag = l_while.location.to_tag(&mut state.tag_map);
                let while_ = &l_while.value;

                let label = state.get_next_label();
                let mut loop_instrs = vec![];

                // Add the break condition
                let condition = state.compile_expr(&mut loop_instrs, &while_.condition);
                loop_instrs.push(simple::FunStmt::If(condition.tag().attach(simple::If {
                    condition,
                    then: condition.tag().attach(vec![]),
                    else_: condition.tag().attach(vec![condition.tag().attach(
                        simple::FunStmt::Break(condition.tag().attach(label.clone())),
                    )]),
                })));

                // Add the rest of the body
                simplify_block(state, &mut loop_instrs, &while_.body.value);

                // Insert the loop
                let loop_ = simple::Loop {
                    label,
                    body: tag.attach(loop_instrs),
                };

                instrs.push(simple::FunStmt::Loop(tag.attach(loop_)));
            }
            plain::FunStmt::Assignment(l_assign) => {
                let tag = l_assign.location.to_tag(&mut state.tag_map);
                let assign = &l_assign.value;

                let body = state.compile_expr(instrs, &assign.value);
                instrs.push(simple::FunStmt::Assignment(tag.attach(
                    simple::Assignment {
                        target: state.widen_plain_ident(&assign.id.widen().value),
                        value: simple::AssignmentValue::Ident(body),
                    },
                )));
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

    fn get_single_use_identifier(&mut self) -> simple::SingleUseIdent {
        let id = simple::SingleUseIdent {
            id: self.next_single_use_identifier,
        };
        self.next_single_use_identifier += 1;
        return id;
    }

    fn compile_expr(&mut self, instrs: &mut Instrs, expr: &plain::Expr) -> simple::Ident {
        match expr {
            plain::Expr::LitNum(n) => {
                let tag = self.tag_map.get_tag(n.location);

                let id = self.get_single_use_identifier();
                let sid = simple::Ident::SingleUse(tag.attach(id));

                let instr = simple::Assignment {
                    target: sid,
                    value: simple::AssignmentValue::LitNum(tag.attach(n.value)),
                };

                instrs.push(simple::FunStmt::Assignment(tag.attach(instr)));

                return sid;
            }
            plain::Expr::Ident(id) => return self.widen_plain_ident(id),
            plain::Expr::FunCall(fun) => {
                let tag = fun.location().to_tag(&mut self.tag_map);

                let mut ps = vec![];
                for arg in fun.args.value.iter() {
                    ps.push(self.compile_expr(instrs, &arg));
                }

                let target = self.get_single_use_identifier();
                let starget = simple::Ident::SingleUse(tag.attach(target));

                instrs.push(simple::FunStmt::Assignment(tag.attach(
                    simple::Assignment {
                        target: starget,
                        value: simple::AssignmentValue::Call(tag.attach(simple::Call {
                            fun_name: fun.name.to_tagged(&mut self.tag_map),
                            arguments: tag.attach(ps),
                        })),
                    },
                )));
                return starget;
            }
            _ => todo!(),
        }
    }

    fn get_next_label(&mut self) -> simple::Label {
        let id = self.next_loop_label;
        self.next_loop_label += 1;
        return simple::Label { id };
    }

    fn widen_plain_ident(&mut self, id: &plain::Ident) -> simple::Ident {
        match id {
            plain::Ident::Local(id) => simple::Ident::Local(id.to_tagged(&mut self.tag_map)),
            plain::Ident::Global(_id) => todo!("not supported yet"), // simple::Ident::Global(id.to_tagged(&mut self.tag_map)),
        }
    }
}
