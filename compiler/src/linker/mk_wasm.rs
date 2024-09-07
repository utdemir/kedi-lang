use std::collections::HashSet;

use crate::util::wasm::WasmBytes;

use super::linked;

pub fn mk_wasm(module: &linked::Module) -> WasmBytes {
    // Encode the type section.
    let mut types = wasm_encoder::TypeSection::new();
    for stmt in module.statements.iter() {
        match stmt {
            linked::TopLevelStmt::FunDecl(fun) => {
                let params = fun.value.implementation.value.params.clone();
                let results = vec![wasm_encoder::ValType::I32];
                types.function(params, results);
            }
        }
    }

    // Encode the function section.
    let mut exports = wasm_encoder::ExportSection::new();
    let mut functions = wasm_encoder::FunctionSection::new();
    for (ix, stmt) in module.statements.iter().enumerate() {
        match stmt {
            linked::TopLevelStmt::FunDecl(fun) => {
                functions.function(ix as u32);
                if fun.value.export {
                    exports.export(
                        &fun.value.name.value.0,
                        wasm_encoder::ExportKind::Func,
                        ix as u32,
                    );
                }
            }
        }
    }

    // Build the statements
    let mut codes = wasm_encoder::CodeSection::new();
    for stmt in module.statements.iter() {
        match stmt {
            linked::TopLevelStmt::FunDecl(fun) => {
                let ls = locals(&fun.value.implementation.value.body);
                let mut f = wasm_encoder::Function::new(ls.iter().map(|t| (1, t.clone())));
                for instr in fun.value.implementation.value.body.iter() {
                    f.instruction(&instr.instr);
                }
                f.instruction(&wasm_encoder::Instruction::End);
                codes.function(&f);
            }
        }
    }

    // Build the module
    let mut module = wasm_encoder::Module::new();
    module.section(&types);
    module.section(&functions);
    module.section(&exports);
    module.section(&codes);

    let wat = module.finish();

    return WasmBytes { bytes: wat };
}

fn locals(instrs: &[linked::Instr]) -> Vec<wasm_encoder::ValType> {
    let mut locals = HashSet::new();
    for instr in instrs.iter() {
        match instr {
            linked::Instr {
                instr: wasm_encoder::Instruction::LocalSet(l),
            } => {
                locals.insert(*l);
            }
            _ => {}
        }
    }

    let max = locals.iter().max().unwrap_or(&0);

    return (0..max + 1).map(|_| wasm_encoder::ValType::I32).collect();
}
