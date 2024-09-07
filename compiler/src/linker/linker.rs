use std::collections::HashMap;

use crate::{codegen::fragment, parser::syntax};

use super::linked;

pub fn run(input: &fragment::Module) -> linked::Module {
    let mut env = LinkerEnv::new();

    let with_stdlib = super::prims::prims().add(&input);

    for stmt in with_stdlib.statements.iter() {
        match stmt {
            fragment::TopLevelStmt::FunDecl(fun) => env.add(fun.value.name.value.clone()),
        }
    }

    link(&mut env, &with_stdlib)
}

fn link(env: &mut LinkerEnv, input: &fragment::Module) -> linked::Module {
    let mut output_statements: Vec<linked::TopLevelStmt> = vec![];

    for stmt in input.statements.iter() {
        match stmt {
            fragment::TopLevelStmt::FunDecl(fun) => {
                output_statements.push(linked::TopLevelStmt::FunDecl(
                    fun.map(|fun| link_function(env, fun)),
                ));
            }
        }
    }

    return linked::Module {
        statements: output_statements,
    };
}

fn link_function(env: &mut LinkerEnv, fun: &fragment::FunDecl) -> linked::FunDecl {
    let mut out_instrs = vec![];

    for instr in fun.implementation.value.body.iter() {
        match instr {
            fragment::Instr::Call(fragment::Call { fun: glo, arity: _ }) => {
                let syn = fun.refs.get(glo).unwrap();
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

    return linked::FunDecl {
        name: fun.name.clone(),
        implementation: fun.implementation.map(|old_impl| linked::FunImpl {
            params: old_impl.params.clone(),
            body: out_instrs,
        }),
    };
}

struct LinkerEnv {
    ids: HashMap<syntax::Ident, u32>,
    next_id: u32,
}

impl LinkerEnv {
    fn new() -> Self {
        LinkerEnv {
            ids: HashMap::new(),
            next_id: 0,
        }
    }

    fn add(&mut self, id: syntax::Ident) {
        if let Some(_) = self.ids.get(&id) {
            panic!("duplicate id: {:?}", id);
        }

        let x = self.next_id;
        self.next_id += 1;
        self.ids.insert(id, x);
    }

    fn resolve(&self, id: &syntax::Ident) -> u32 {
        self.ids
            .get(&id)
            .unwrap_or_else(|| panic!("unknown id: {:?}, available: {:?}", id, self.ids))
            .clone()
    }
}
