use std::collections::{HashMap, HashSet};

use crate::codegen_wasm::rts::{object_val_type, OBJECT_TYPE_ID};
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
            // element_type: wasm_encoder::StorageType::Val(wasm_encoder::ValType::Ref(
            //     wasm_encoder::RefType::ANYREF
            // )),
            element_type: wasm_encoder::StorageType::Val(wasm_encoder::ValType::I32),
            mutable: false,
        },
    ]);
    assert!(obj_type == OBJECT_TYPE_ID);
    // let tag_fun = env.get_type_func_ix(vec![wasm_encoder::ValType::I32], vec![]);

    // let mut tags = wasm_encoder::TagSection::new();
    // tags.tag(TagType {
    //     kind: wasm_encoder::TagKind::Exception,
    //     func_type_idx: tag_fun,
    // });

    let mut exports = wasm_encoder::ExportSection::new();
    for stmt in module.statements.iter() {
        match stmt {
            linked::TopLevelStmt::FunDecl(fun) => {
                let type_ix = env.get_type_func_ix(
                    fun.implementation.value.params.clone(),
                    vec![object_val_type()],
                );
                let locals = locals(&fun.implementation.value.body);
                let body = fun
                    .implementation
                    .value
                    .body
                    .iter()
                    .map(|instr| instr.instr.clone())
                    .collect();

                let fun_ix = env.add_func(type_ix, locals, body);

                // Export if necessary
                if fun.export {
                    exports.export(
                        &fun.name.value.0,
                        wasm_encoder::ExportKind::Func,
                        fun_ix as u32,
                    );
                }
            }
        }
    }

    // Build the module
    let mut module = wasm_encoder::Module::new();
    module.section(&env.type_section);
    module.section(&env.function_section);
    // module.section(&tags);
    module.section(&exports);
    module.section(&env.code_section);
    let wat = module.finish();

    WasmBytes { bytes: wat }
}

fn locals(instrs: &[linked::Instr]) -> Vec<wasm_encoder::ValType> {
    let mut locals = HashSet::new();
    for instr in instrs.iter() {
        if let linked::Instr {
            instr: wasm_encoder::Instruction::LocalSet(l),
        } = instr
        {
            locals.insert(*l);
        }
    }

    let max = locals.iter().max().unwrap_or(&0);

    (0..max + 1).map(|_| object_val_type()).collect()
}

struct MkWasmEnv {
    type_map: HashMap<TypeKind, u32>,
    type_section: wasm_encoder::TypeSection,
    function_section: wasm_encoder::FunctionSection,
    code_section: wasm_encoder::CodeSection,
}

impl MkWasmEnv {
    fn new() -> MkWasmEnv {
        MkWasmEnv {
            type_map: HashMap::new(),
            type_section: wasm_encoder::TypeSection::new(),
            function_section: wasm_encoder::FunctionSection::new(),
            code_section: wasm_encoder::CodeSection::new(),
        }
    }

    fn get_type_ix(&mut self, ty: TypeKind) -> u32 {
        if let Some(existing) = self.type_map.get(&ty) {
            *existing
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
            ix
        }
    }

    fn get_type_type_ix(&mut self, fields: Vec<wasm_encoder::FieldType>) -> u32 {
        let ty = TypeKind::Type_(fields);
        self.get_type_ix(ty)
    }

    fn get_type_func_ix(
        &mut self,
        params: Vec<wasm_encoder::ValType>,
        results: Vec<wasm_encoder::ValType>,
    ) -> u32 {
        let ty = TypeKind::Func(params, results);
        self.get_type_ix(ty)
    }

    fn add_func(
        &mut self,
        type_ix: u32,
        locals: Vec<wasm_encoder::ValType>,
        body: Vec<wasm_encoder::Instruction>,
    ) -> u32 {
        let mut f = wasm_encoder::Function::new(locals.iter().map(|t| (1, *t)));
        for instr in body.iter() {
            f.instruction(instr);
        }
        f.instruction(&wasm_encoder::Instruction::End);
        self.function_section.function(type_ix);
        self.code_section.function(&f);

        self.function_section.len() - 1
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TypeKind {
    Func(Vec<wasm_encoder::ValType>, Vec<wasm_encoder::ValType>),
    Type_(Vec<wasm_encoder::FieldType>),
}
