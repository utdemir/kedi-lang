use crate::codegen::types::*;
use crate::{renamer::plain, simplifier::simple};

pub fn run(input: &simple::Module) -> WasmBytes {
    let mut fields = vec![];

    for stmt in input.statements.iter() {
        match stmt {
            simple::TopLevelStmt::FunDecl(fun) => {
                let fun = codegen_function(&fun);
                let field = wast::core::ModuleField::Func(fun);
                fields.push(field);
            }
        }
    }

    let mut module = wast::core::Module {
        span: empty_span(),
        id: None,
        name: None,
        kind: wast::core::ModuleKind::Text(fields),
    };

    let wat = module.encode().unwrap();

    return WasmBytes { bytes: wat };
}

fn codegen_function(input: &simple::FunDecl) -> wast::core::Func {
    let mut instructions = vec![];

    let mut locals = vec![];

    for stmt in input.implementation.body.iter() {
        match stmt {
            simple::Statement::Assignment(simple::Assignment { target, value }) => {
                let t = simple_identifier_to_id(target);

                match target {
                    simple::Identifier::SingleUse { .. }
                    | &simple::Identifier::Plain(plain::Identifier::Local(_)) => {
                        locals.push(wast::core::Local {
                            id: Some(t),
                            name: None,
                            ty: wast::core::ValType::I32,
                        });
                    }
                    _ => {}
                }

                match value {
                    simple::AssignmentValue::LiteralNumber(i) => {
                        instructions.push(wast::core::Instruction::I32Const(*i));
                    }
                    &simple::AssignmentValue::Identifier(ref id) => {
                        instructions.push(wast::core::Instruction::LocalGet(
                            simple_identifier_to_uid(id),
                        ));
                    }
                    &simple::AssignmentValue::Call(ref id, ref args) => {
                        for arg in args.iter() {
                            instructions.push(wast::core::Instruction::LocalGet(
                                simple_identifier_to_uid(arg),
                            ));
                        }

                        instructions.push(wast::core::Instruction::I32Const(id.id as i32));

                        instructions.push(wast::core::Instruction::CallIndirect(Box::new(
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

                instructions.push(wast::core::Instruction::LocalSet(simple_identifier_to_uid(
                    target,
                )));
            }
            simple::Statement::Return(id) => {
                instructions.push(wast::core::Instruction::LocalGet(simple_identifier_to_uid(
                    id,
                )));
                instructions.push(wast::core::Instruction::Return);
            }
            simple::Statement::Loop(_) => todo!(),
            simple::Statement::Branch(_) => todo!(),
        }
    }

    return wast::core::Func {
        span: empty_span(),
        id: None,
        name: None,
        exports: wast::core::InlineExport {
            names: vec![&input.name.name],
        },
        ty: wast::core::TypeUse {
            index: None,
            inline: Some(wast::core::FunctionType {
                params: input
                    .implementation
                    .parameters
                    .iter()
                    .map(|x| {
                        (
                            Some(local_identifier_to_id(x)),
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

// State

struct CodegenState {
    next_single_use_identifier: u32,
}

// Utils

fn empty_span() -> wast::token::Span {
    return wast::token::Span::from_offset(0);
}

// Hack, obviously.
fn leak_str(s: String) -> &'static str {
    return Box::leak(s.into_boxed_str());
}

fn local_identifier_to_id(id: &plain::LocalIdentifier) -> wast::token::Id<'static> {
    let str: &'static str = leak_str(format!("local#{}", id.id));
    return wast::token::Id::new(&str, empty_span());
}

fn local_identifier_to_uid(id: &plain::LocalIdentifier) -> wast::token::Index<'static> {
    let id = wast::token::Index::Id(local_identifier_to_id(id));
    return id;
}
fn global_identifier_to_id(id: &plain::GlobalIdentifier) -> wast::token::Id<'static> {
    let str: &'static str = leak_str(format!("global#{}", id.id));
    return wast::token::Id::new(&str, empty_span());
}

fn global_identifier_to_uid(id: &plain::GlobalIdentifier) -> wast::token::Index<'static> {
    return wast::token::Index::Id(global_identifier_to_id(id));
}

fn plain_identifier_to_id(id: &plain::Identifier) -> wast::token::Id<'static> {
    match id {
        plain::Identifier::Local(id) => local_identifier_to_id(id),
        plain::Identifier::Global(id) => global_identifier_to_id(id),
    }
}

fn plain_identifier_to_uid(id: &plain::Identifier) -> wast::token::Index<'static> {
    return wast::token::Index::Id(plain_identifier_to_id(id));
}

fn single_use_identifier_to_id(id: &simple::SingleUseIdentifier) -> wast::token::Id<'static> {
    let str: &'static str = leak_str(format!("single_use#{}", id.id));
    return wast::token::Id::new(&str, empty_span());
}

fn single_use_identifier_to_uid(id: &simple::SingleUseIdentifier) -> wast::token::Index<'static> {
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
