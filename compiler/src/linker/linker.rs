use std::collections::HashMap;
use ustr::Ustr;

use crate::{codegen::fragment, renamer::plain::GlobalIdentifier, util::wasm::WasmBytes};

pub fn run(mut input: fragment::Module) -> WasmBytes {
    link(&mut input);

    let mut fields = vec![];

    // Build the statements
    for stmt in input.statements {
        match stmt.value {
            fragment::TopLevelStmt::FunDecl(fun) => {
                let body = fun.implementation.value.body;
                fields.push(wast::core::ModuleField::Func(body));
            }
        }
    }

    // Build the module
    let mut module = wast::core::Module {
        span: wast::token::Span::from_offset(0),
        id: None,
        name: None,
        kind: wast::core::ModuleKind::Text(fields),
    };

    let wat = module.encode().unwrap();

    return WasmBytes { bytes: wat };
}

pub fn link(input: &mut fragment::Module) {
    for stmt in input.statements.iter_mut() {
        match &mut stmt.value {
            fragment::TopLevelStmt::FunDecl(fun) => {
                link_function(fun);
            }
        }
    }
}

pub fn link_function(fun: &mut fragment::FunDecl) {
    let mut idx = 0;
    let mut instrs = match &mut fun.implementation.value.body.kind {
        wast::core::FuncKind::Inline { expression, .. } => &mut expression.instrs,
        _ => panic!("unexpected function kind"),
    };

    loop {
        if idx > instrs.len() - 2 {
            break;
        }

        let [ref curr, ref next] = instrs[idx..idx + 2] else {
            break;
        };

        match (curr, next) {
            (
                wast::core::Instruction::I32Const(val),
                wast::core::Instruction::CallIndirect { .. },
            ) => {
                let name = Ustr::from(
                    fun.refs
                        .get(&GlobalIdentifier { id: *val as u32 })
                        .unwrap_or_else(|| panic!("unknown function index: {}", val))
                        .name
                        .as_str(),
                )
                .as_str();
                instrs[idx] = wast::core::Instruction::Nop;
                instrs[idx + 1] = wast::core::Instruction::Call(wast::token::Index::Id(
                    wast::token::Id::new(name, wast::token::Span::from_offset(0)),
                ));
                idx += 2;
            }
            _ => {
                idx += 1;
            }
        }
    }
}

// pub fn link_other(input: &mut fragment::Module) {
//     // First, get all top level names
//     let mut names = HashMap::new();
//     for (idx, stmt) in input.statements.iter().enumerate() {
//         match &stmt.value {
//             fragment::TopLevelStmt::FunDecl(fun) => {
//                 if names.contains_key(&fun.name.value) {
//                     panic!("duplicate function name: {}", fun.name.value.name);
//                 }

//                 names.insert(fun.name.value.clone(), idx);
//             }
//         }
//     }

//     // And then update the globals for each function
//     for stmt in input.statements.iter_mut() {
//         match &mut stmt.value {
//             fragment::TopLevelStmt::FunDecl(fun) => {
//                 let mut map = HashMap::new();

//                 for (key, value) in &fun.refs {
//                     let idx = names.get(value).unwrap();
//                     map.insert(*key, *idx);
//                 }

//                 update_call_indirects(&mut fun.implementation.value.body, map);
//             }
//         }
//     }
// }

fn update_call_indirects(func: &mut wast::core::Func, names: HashMap<GlobalIdentifier, usize>) {
    match &mut func.kind {
        wast::core::FuncKind::Inline { expression, .. } => {
            let instrs = &mut expression.instrs;

            for i in 0..instrs.len() - 1 {
                match &mut instrs[i] {
                    wast::core::Instruction::CallIndirect { .. } => {
                        let prev = instrs.get_mut(i - 1);
                        match prev {
                            Some(wast::core::Instruction::I32Const(ref mut val)) => {
                                let name = names.get(&GlobalIdentifier { id: *val as u32 });

                                match name {
                                    Some(new_val) => *val = (*new_val) as i32,
                                    None => {
                                        panic!("unknown function index: {}", val);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
