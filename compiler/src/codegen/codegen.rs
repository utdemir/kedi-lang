#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};

use wasm_encoder;

use super::fragment::{self, FunDecl, FunImpl, Instr};
use crate::{renamer::plain, simplifier::simple};

pub fn run(input: &simple::Module) -> fragment::Module {
    let mut statements = vec![];

    for stmt in input.statements.iter() {
        match &stmt {
            simple::TopLevelStmt::FunDecl(fun) => {
                let implementation = fun.value.implementation.map(codegen_function);
                let f = fun.map(|fun| {
                    return FunDecl {
                        name: fun.name.clone(),
                        implementation,
                        refs: fun.refs.clone(),
                    };
                });

                statements.push(fragment::TopLevelStmt::FunDecl(f));
            }
        }
    }

    return fragment::Module { statements };
}

fn codegen_function(input: &simple::FunImpl) -> fragment::FunImpl {
    let mut instructions: Vec<Instr> = vec![];
    let mut state = CodegenState::new();

    for param in input.parameters.value.iter() {
        state.register_param(&param.value);
    }

    for stmt in input.body.value.iter() {
        codegen_statement(&mut state, &mut instructions, &stmt);
    }

    let locals: Vec<(u32, wasm_encoder::ValType)> = state
        .locals
        .values()
        .map(|x| (*x, wasm_encoder::ValType::I32))
        .collect();

    return fragment::FunImpl {
        params: input
            .parameters
            .value
            .iter()
            .map(|x| wasm_encoder::ValType::I32)
            .collect(),
        body: instructions,
    };
}

fn codegen_statement(state: &mut CodegenState, instrs: &mut Vec<Instr>, stmt: &simple::FunStmt) {
    match stmt {
        simple::FunStmt::Assignment(l_assignment) => {
            let simple::Assignment { value, target } = &l_assignment.value;

            match value {
                simple::AssignmentValue::LitNum(i) => {
                    instrs.push(Instr::Raw(wasm_encoder::Instruction::I32Const(i.value.0)));
                }
                simple::AssignmentValue::Ident(ref id) => {
                    instrs.push(Instr::Raw(wasm_encoder::Instruction::LocalGet(
                        state.resolve_simple_ident(id),
                    )));
                }
                simple::AssignmentValue::Call(l_call) => {
                    let id = l_call.value.fun_name.value;
                    let args = &l_call.value.arguments.value;

                    for arg in args.iter() {
                        instrs.push(Instr::Raw(wasm_encoder::Instruction::LocalGet(
                            state.resolve_simple_ident(arg),
                        )));
                    }

                    instrs.push(Instr::Call(fragment::Call {
                        fun: id,
                        arity: args.len(),
                    }));
                }
            }

            instrs.push(Instr::Raw(wasm_encoder::Instruction::LocalSet(
                state.resolve_simple_ident(&target),
            )));
        }
        simple::FunStmt::Return(id) => {
            instrs.push(Instr::Raw(wasm_encoder::Instruction::LocalGet(
                state.resolve_simple_ident(id),
            )));
            instrs.push(Instr::Raw(wasm_encoder::Instruction::Return));
        }
        simple::FunStmt::Nop => {}
        simple::FunStmt::If(if_) => {
            // let simple::If {
            //     condition,
            //     then,
            //     else_,
            // } = &if_.value;

            // let ret = wast::core::Instruction::If(Box::new(wast::core::BlockType {
            //     label: None,
            //     label_name: None,
            //     ty: wast::core::TypeUse {
            //         index: None,
            //         inline: None,
            //     },
            // }));

            todo!("codegen_statement: If");
        }
        simple::FunStmt::Loop(loop_) => {
            // let simple::Loop { label, body } = &loop_.value;

            // let ret = wast::core::Instruction::Loop(Box::new(wast::core::BlockType {
            //     label: None,
            //     label_name: None,
            //     ty: wast::core::TypeUse {
            //         index: None,
            //         inline: None,
            //     },
            // }));

            todo!("codegen_statement: Loop");
        }
        simple::FunStmt::Break(_label) => {
            // instrs.push(wast::core::Instruction::Nop);
            todo!("codegen_statement: Break");
        }
        simple::FunStmt::Branch(_label) => {
            // instrs.push(wast::core::Instruction::Nop);
            todo!("codegen_statement: Branch");
        }
    }
}

// State

struct CodegenState {
    params: HashMap<plain::LocalIdent, u32>,
    locals: HashMap<plain::LocalIdent, u32>,
    single_uses: HashMap<simple::SingleUseIdent, u32>,
    next_local_id: u32,
}

impl CodegenState {
    fn new() -> Self {
        return CodegenState {
            params: HashMap::new(),
            locals: HashMap::new(),
            single_uses: HashMap::new(),
            next_local_id: 0,
        };
    }

    fn register_param(&mut self, param: &plain::LocalIdent) {
        self.params.insert(param.clone(), self.next_local_id);
        self.next_local_id += 1;
    }

    fn resolve_local(&mut self, local: &plain::LocalIdent) -> u32 {
        if let Some(x) = self.params.get(local) {
            return *x;
        }

        if let Some(x) = self.locals.get(local) {
            return *x;
        }

        let id = self.next_local_id;
        self.next_local_id += 1;

        self.locals.insert(local.clone(), id);
        return id;
    }

    fn resolve_single_use(&mut self, single_use: &simple::SingleUseIdent) -> u32 {
        if let Some(x) = self.single_uses.get(single_use) {
            return *x;
        }

        let id = self.next_local_id;
        self.next_local_id += 1;

        self.single_uses.insert(single_use.clone(), id);
        return id;
    }

    fn resolve_simple_ident(&mut self, id: &simple::Ident) -> u32 {
        match id {
            simple::Ident::Local(id) => self.resolve_local(&id.value),
            simple::Ident::SingleUse(id) => self.resolve_single_use(&id.value),
        }
    }

    // fn resolve_local(&mut self, local: &plain::LocalIdent) -> u32 {
    //     match self.locals.get(local) {
    //         Some(x) => return *x,
    //         None => {
    //             let id = self.next_local_id;
    //             self.next_local_id += 1;
    //             self.locals.insert(local.clone(), id);
    //             return id;
    //         }
    //     }
    // }
}

// Utils

fn empty_span() -> wast::token::Span {
    return wast::token::Span::from_offset(0);
}

// // Hack, obviously.
// fn leak_str(s: String) -> &'static str {
//     return ustr::ustr(&s).as_str();
// }

// fn local_identifier_to_id(id: &plain::LocalIdent) -> wast::token::Id<'static> {
//     let str: &'static str = leak_str(format!("local#{}", id.id));
//     return wast::token::Id::new(&str, empty_span());
// }

// fn _local_identifier_to_uid(id: &plain::LocalIdent) -> wast::token::Index<'static> {
//     let id = wast::token::Index::Id(local_identifier_to_id(id));
//     return id;
// }
// fn global_identifier_to_id(id: &plain::GlobalIdent) -> wast::token::Id<'static> {
//     let str: &'static str = leak_str(format!("global#{}", id.id));
//     return wast::token::Id::new(&str, empty_span());
// }

// fn _global_identifier_to_uid(id: &plain::GlobalIdent) -> wast::token::Index<'static> {
//     return wast::token::Index::Id(global_identifier_to_id(id));
// }

// fn single_use_identifier_to_id(id: &simple::SingleUseIdent) -> wast::token::Id<'static> {
//     let str: &'static str = leak_str(format!("single_use#{}", id.id));
//     return wast::token::Id::new(&str, empty_span());
// }

// fn _single_use_identifier_to_uid(id: &simple::SingleUseIdent) -> wast::token::Index<'static> {
//     return wast::token::Index::Id(single_use_identifier_to_id(id));
// }

// fn simple_identifier_to_id(id: &simple::Ident) -> wast::token::Id<'static> {
//     match id {
//         simple::Ident::Local(id) => local_identifier_to_id(&id.value),
//         // simple::Ident::Global(id) => global_identifier_to_id(&id.value),
//         simple::Ident::SingleUse(id) => single_use_identifier_to_id(&id.value),
//     }
// }

// fn simple_identifier_to_uid(id: &simple::Ident) -> wast::token::Index<'static> {
//     return wast::token::Index::Id(simple_identifier_to_id(id));
// }
