use std::collections::HashSet;

use super::fragment::{self, FunDecl, FunImpl};
use crate::{renamer::plain, simplifier::simple};

pub fn run(input: &simple::Module) -> fragment::Module {
    let mut statements = vec![];

    for stmt in input.statements.iter() {
        match &stmt.value {
            simple::TopLevelStmt::FunDecl(fun) => {
                let gen = codegen_function(&fun.value);
                let f = fun.map(|fun| {
                    return fragment::TopLevelStmt::FunDecl(FunDecl {
                        name: fun.name.clone(),
                        implementation: fun.implementation.location.attach(FunImpl {
                            parameters: fun
                                .implementation
                                .value
                                .parameters
                                .value
                                .iter()
                                .map(|x| x.value)
                                .collect(),
                            body: gen,
                        }),
                        refs: fun.refs.clone(),
                    });
                });

                statements.push(f);
            }
        }
    }

    return fragment::Module { statements };
}

fn codegen_function(input: &simple::FunDecl) -> wast::core::Func {
    let mut instructions = vec![];
    let mut state = CodegenState::new();

    for stmt in input.implementation.value.body.value.iter() {
        codegen_statement(&mut state, &mut instructions, &stmt.value);
    }

    let locals = state
        .locals
        .iter()
        .map(|x| wast::core::Local {
            name: None,
            id: Some(simple_identifier_to_id(x)),
            ty: wast::core::ValType::I32,
        })
        .collect::<Vec<_>>();

    return wast::core::Func {
        span: empty_span(),
        id: Some(wast::token::Id::new(
            input.name.value.name.as_str(),
            empty_span(),
        )),
        name: Some(wast::token::NameAnnotation {
            name: input.name.value.name.as_str(),
        }),
        exports: wast::core::InlineExport {
            names: vec![&input.name.value.name],
        },
        ty: wast::core::TypeUse {
            index: None,
            inline: Some(wast::core::FunctionType {
                params: input
                    .implementation
                    .value
                    .parameters
                    .value
                    .iter()
                    .map(|x| {
                        (
                            Some(local_identifier_to_id(&x.value)),
                            None,
                            wast::core::ValType::I32,
                        )
                    })
                    .collect(),
                results: Box::new([wast::core::ValType::I32]),
            }),
        },
        kind: wast::core::FuncKind::Inline {
            locals: locals.into_boxed_slice(),
            expression: wast::core::Expression {
                instrs: instructions.into_boxed_slice(),
                branch_hints: vec![],
            },
        },
    };
}

fn codegen_statement(
    state: &mut CodegenState,
    instrs: &mut Vec<wast::core::Instruction>,
    stmt: &simple::Statement,
) {
    match stmt {
        simple::Statement::Assignment(l_assignment) => {
            let simple::Assignment {
                value: l_value,
                target: l_target,
            } = l_assignment;

            let value = &l_value.value;
            let target = &l_target.value;

            match target {
                simple::Identifier::SingleUse { .. }
                | simple::Identifier::Plain(plain::Identifier::Local(_)) => {
                    state.locals.insert(target.clone());
                }
                _ => {}
            }

            match value {
                simple::AssignmentValue::LiteralNumber(i) => {
                    instrs.push(wast::core::Instruction::I32Const(*i));
                }
                simple::AssignmentValue::Identifier(ref id) => {
                    instrs.push(wast::core::Instruction::LocalGet(simple_identifier_to_uid(
                        id,
                    )));
                }
                simple::AssignmentValue::Call(simple::Call {
                    fun_name: l_fun_name,
                    arguments: l_arguments,
                }) => {
                    let id = l_fun_name.value;
                    let args = &l_arguments.value;

                    for arg in args.iter() {
                        instrs.push(wast::core::Instruction::LocalGet(simple_identifier_to_uid(
                            &arg.value,
                        )));
                    }

                    instrs.push(wast::core::Instruction::I32Const(id.id as i32));

                    instrs.push(wast::core::Instruction::CallIndirect(Box::new(
                        wast::core::CallIndirect {
                            table: wast::token::Index::Num(0, empty_span()),
                            ty: wast::core::TypeUse {
                                index: None,
                                inline: Some(wast::core::FunctionType {
                                    params: args
                                        .iter()
                                        .map(|_| (None, None, wast::core::ValType::I32))
                                        .collect::<Vec<_>>()
                                        .into_boxed_slice(),
                                    results: Box::new([wast::core::ValType::I32]),
                                }),
                            },
                        },
                    )));
                }
            }

            instrs.push(wast::core::Instruction::LocalSet(simple_identifier_to_uid(
                target,
            )));
        }
        simple::Statement::Return(id) => {
            instrs.push(wast::core::Instruction::LocalGet(simple_identifier_to_uid(
                id,
            )));
            instrs.push(wast::core::Instruction::Return);
        }
        simple::Statement::Nop => {}
        simple::Statement::If(if_) => {
            let simple::If {
                condition: _l_condition,
                then: _l_then,
                else_: _l_else,
            } = if_;

            let ret = wast::core::Instruction::If(Box::new(wast::core::BlockType {
                label: None,
                label_name: None,
                ty: wast::core::TypeUse {
                    index: None,
                    inline: None,
                },
            }));

            instrs.push(ret);
        }
        simple::Statement::Loop(loop_) => {
            let simple::Loop {
                label: _l_label,
                body: _l_body,
            } = loop_;

            let ret = wast::core::Instruction::Loop(Box::new(wast::core::BlockType {
                label: None,
                label_name: None,
                ty: wast::core::TypeUse {
                    index: None,
                    inline: None,
                },
            }));

            instrs.push(ret);
        }
        simple::Statement::Break(_label) => {
            instrs.push(wast::core::Instruction::Nop);
        }
        simple::Statement::Branch(_label) => {
            instrs.push(wast::core::Instruction::Nop);
        }
        simple::Statement::InlineWasm(inline_wasm) => {
            for input in inline_wasm.input_stack.value.iter() {
                instrs.push(wast::core::Instruction::LocalGet(simple_identifier_to_uid(
                    &input.value,
                )));
            }

            instrs.push(inline_wasm.wasm.value.clone());

            for output in inline_wasm.output_stack.value.iter() {
                instrs.push(wast::core::Instruction::LocalSet(simple_identifier_to_uid(
                    &output.value,
                )));
            }
        }
    }
}

// State

struct CodegenState {
    locals: HashSet<simple::Identifier>,
}

impl CodegenState {
    fn new() -> Self {
        return CodegenState {
            locals: HashSet::new(),
        };
    }
}

// Utils

fn empty_span() -> wast::token::Span {
    return wast::token::Span::from_offset(0);
}

// Hack, obviously.
fn leak_str(s: String) -> &'static str {
    return ustr::ustr(&s).as_str();
}

fn local_identifier_to_id(id: &plain::LocalIdentifier) -> wast::token::Id<'static> {
    let str: &'static str = leak_str(format!("local#{}", id.id));
    return wast::token::Id::new(&str, empty_span());
}

fn _local_identifier_to_uid(id: &plain::LocalIdentifier) -> wast::token::Index<'static> {
    let id = wast::token::Index::Id(local_identifier_to_id(id));
    return id;
}
fn global_identifier_to_id(id: &plain::GlobalIdentifier) -> wast::token::Id<'static> {
    let str: &'static str = leak_str(format!("global#{}", id.id));
    return wast::token::Id::new(&str, empty_span());
}

fn _global_identifier_to_uid(id: &plain::GlobalIdentifier) -> wast::token::Index<'static> {
    return wast::token::Index::Id(global_identifier_to_id(id));
}

fn plain_identifier_to_id(id: &plain::Identifier) -> wast::token::Id<'static> {
    match id {
        plain::Identifier::Local(id) => local_identifier_to_id(id),
        plain::Identifier::Global(id) => global_identifier_to_id(id),
    }
}

fn _plain_identifier_to_uid(id: &plain::Identifier) -> wast::token::Index<'static> {
    return wast::token::Index::Id(plain_identifier_to_id(id));
}

fn single_use_identifier_to_id(id: &simple::SingleUseIdentifier) -> wast::token::Id<'static> {
    let str: &'static str = leak_str(format!("single_use#{}", id.id));
    return wast::token::Id::new(&str, empty_span());
}

fn _single_use_identifier_to_uid(id: &simple::SingleUseIdentifier) -> wast::token::Index<'static> {
    return wast::token::Index::Id(single_use_identifier_to_id(id));
}

fn simple_identifier_to_id(id: &simple::Identifier) -> wast::token::Id<'static> {
    match id {
        simple::Identifier::Plain(id) => plain_identifier_to_id(id),
        simple::Identifier::SingleUse(id) => single_use_identifier_to_id(id),
    }
}

fn simple_identifier_to_uid(id: &simple::Identifier) -> wast::token::Index<'static> {
    return wast::token::Index::Id(simple_identifier_to_id(id));
}
