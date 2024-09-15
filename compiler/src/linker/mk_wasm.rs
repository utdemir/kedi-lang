use std::collections::{HashMap, HashSet};

use wasm_encoder::TagType;

use crate::codegen::rts::{object_val_type, OBJECT_TYPE_ID};
use crate::util::wasm::WasmBytes;

use super::linked;

pub fn mk_wasm(module: &linked::Module) -> WasmBytes {
    let mut env = MkWasmEnv::new();

    let obj_type = env.get_type_type_ix(vec![
        // Tag
        wasm_encoder::FieldType {
            element_type: wasm_encoder::StorageType::Val(wasm_encoder::ValType::I32),
            mutable: false,
        },
        // Value
        wasm_encoder::FieldType {
            element_type: wasm_encoder::StorageType::Val(wasm_encoder::ValType::Ref(
                wasm_encoder::RefType::ANYREF,
            )),
            mutable: false,
        },
    ]);
    assert!(obj_type == OBJECT_TYPE_ID);

    let mut tags = wasm_encoder::TagSection::new();
    tags.tag(TagType {
        kind: wasm_encoder::TagKind::Exception,
        func_type_idx: 0,
    });

    let mut exports = wasm_encoder::ExportSection::new();
    let mut functions = wasm_encoder::FunctionSection::new();
    let mut codes = wasm_encoder::CodeSection::new();
    for (ix, stmt) in module.statements.iter().enumerate() {
        match stmt {
            linked::TopLevelStmt::FunDecl(fun) => {
                // Encode the function type
                functions.function(env.get_type_func_ix(
                    fun.value.implementation.value.params.clone(),
                    vec![object_val_type()],
                ));
                // Export if necessary
                if fun.value.export {
                    exports.export(
                        &fun.value.name.value.0,
                        wasm_encoder::ExportKind::Func,
                        ix as u32,
                    );
                }
                // Encode the function body
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
    module.section(&env.type_section);
    module.section(&functions);
    module.section(&tags);
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

    return (0..max + 1).map(|_| object_val_type()).collect();
}

struct MkWasmEnv {
    type_map: HashMap<TypeKind, u32>,
    type_section: wasm_encoder::TypeSection,
}

impl MkWasmEnv {
    fn new() -> MkWasmEnv {
        MkWasmEnv {
            type_map: HashMap::new(),
            type_section: wasm_encoder::TypeSection::new(),
        }
    }

    fn get_type_ix(&mut self, ty: TypeKind) -> u32 {
        if let Some(existing) = self.type_map.get(&ty) {
            return *existing;
        } else {
            match ty {
                TypeKind::Func(ref params, ref results) => {
                    self.type_section.function(params.clone(), results.clone());
                }
                TypeKind::Type_(ref params) => {
                    self.type_section.struct_(params.clone());
                }
            }
            let ix = self.type_section.len() - 1;
            self.type_map.insert(ty, ix);
            return ix;
        }
    }

    fn get_type_type_ix(&mut self, fields: Vec<wasm_encoder::FieldType>) -> u32 {
        let ty = TypeKind::Type_(fields);
        return self.get_type_ix(ty);
    }

    fn get_type_func_ix(
        &mut self,
        params: Vec<wasm_encoder::ValType>,
        results: Vec<wasm_encoder::ValType>,
    ) -> u32 {
        let ty = TypeKind::Func(params, results);
        return self.get_type_ix(ty);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TypeKind {
    Func(Vec<wasm_encoder::ValType>, Vec<wasm_encoder::ValType>),
    Type_(Vec<wasm_encoder::FieldType>),
}
