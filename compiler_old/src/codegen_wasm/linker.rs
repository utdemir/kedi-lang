use std::collections::HashMap;

use crate::{codegen_wasm::fragment, parser::syntax};

use super::{linked, prims};

pub fn run(input: &fragment::Module) -> linked::Module {
    let mut env = LinkerEnv::new();

    for stmt in prims::prims().statements.iter() {
        match stmt {
            fragment::TopLevelStmt::FunDecl(fun) => {
                env.register_available_fragment(&fun.value);
            }
        }
    }

    for stmt in input.statements.iter() {
        match stmt {
            fragment::TopLevelStmt::FunDecl(fun) => {
                env.register_available_fragment(&fun.value);
            }
        }
    }

    for stmt in input.statements.iter() {
        match stmt {
            fragment::TopLevelStmt::FunDecl(fun) => {
                if fun.value.export {
                    env.resolve(&fun.value.name.value);
                }
            }
        }
    }

    let mut output_statements: Vec<linked::TopLevelStmt> = vec![];
    let mut funs: Vec<_> = env.funs.into_iter().collect();
    funs.sort_by_key(|(ix, _)| *ix);
    for (_, fun) in funs {
        output_statements.push(linked::TopLevelStmt::FunDecl(fun));
    }

    linked::Module {
        statements: output_statements,
    }
}

fn link_function(env: &mut LinkerEnv, fun: &fragment::FunDecl) -> linked::FunDecl {
    let mut out_instrs = vec![];

    for instr in fun.implementation.value.body.iter() {
        match instr {
            fragment::Instr::Call(fragment::Call { fun: glo, arity: _ }) => {
                let syn = fun
                    .refs
                    .get_by_left(glo)
                    .unwrap_or_else(|| panic!("No reference found for {:?}", glo));
                let id = env.resolve(syn);
                out_instrs.push(linked::Instr {
                    instr: wasm_encoder::Instruction::Call(id),
                });
            }
            fragment::Instr::Raw(r) => {
                out_instrs.push(linked::Instr { instr: r.clone() });
            }
        }
    }

    linked::FunDecl {
        name: fun.name.clone(),
        export: fun.export,
        implementation: fun.implementation.map(|old_impl| linked::FunImpl {
            params: old_impl.params.clone(),
            body: out_instrs,
        }),
    }
}

struct LinkerEnv {
    available_fragments: HashMap<syntax::Ident, fragment::FunDecl>,
    funs: HashMap<u32, linked::FunDecl>,
    ixs: HashMap<syntax::Ident, u32>,
}

impl LinkerEnv {
    fn new() -> Self {
        LinkerEnv {
            available_fragments: HashMap::new(),
            funs: HashMap::new(),
            ixs: HashMap::new(),
        }
    }

    fn register_available_fragment(&mut self, fun: &fragment::FunDecl) {
        self.available_fragments
            .insert(fun.name.value.clone(), fun.clone());
    }

    fn resolve(&mut self, name: &syntax::Ident) -> u32 {
        if let Some(ix) = self.ixs.get(name) {
            *ix
        } else {
            let fun = self
                .available_fragments
                .get(name)
                .unwrap_or_else(|| panic!("No available fragment found for {:?}", name))
                .clone();
            let linked = link_function(self, &fun);

            let ix = self.funs.len() as u32;
            self.ixs.insert(name.clone(), ix);
            self.funs.insert(ix, linked);
            ix
        }
    }
}
