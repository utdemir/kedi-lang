use std::collections::HashMap;

use wast::token::Id;

use crate::{
    codegen::fragment::{self, FunImpl, TopLevelStmt},
    parser::syntax,
    renamer::plain::GlobalIdent,
    util::{loc::WithLoc, wasm::WasmBytes},
};

pub fn run(input: &fragment::Module) -> WasmBytes {
    let mut env = LinkerEnv::new();

    for stmt in input.statements.iter() {
        match stmt {
            fragment::TopLevelStmt::FunDecl(fun) => env.add(fun.value.name.value),
        }
    }

    let linked = link(&mut env, &input);

    let mut fields = vec![];

    // Build the statements
    for stmt in linked.statements {
        match stmt {
            fragment::TopLevelStmt::FunDecl(fun) => {
                let body = fun.value.implementation.value.body;
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

pub fn link(env: &mut LinkerEnv, input: &fragment::Module) -> fragment::Module {
    let mut output_statements: Vec<TopLevelStmt> = vec![];

    for stmt in input.statements.iter() {
        match stmt {
            fragment::TopLevelStmt::FunDecl(fun) => {
                output_statements.push(fragment::TopLevelStmt::FunDecl(
                    fun.map(|fun| link_function(env, fun)),
                ));
            }
        }
    }

    return fragment::Module {
        statements: output_statements,
    };
}

pub fn link_function(env: &mut LinkerEnv, fun: &fragment::FunDecl) -> fragment::FunDecl {
    let mut out_instrs = vec![];

    for instr in fun.implementation.value.body.iter() {
        match instr {
            fragment::Instr::Call(fragment::Call { fun: glo, arity: _ }) => {
                let syn = fun.refs.get(glo).unwrap();
                let id = env.resolve(syn);
                out_instrs.push(fragment::Instr::Raw(wasm_encoder::Instruction::Call(id)));
            }
            other => {
                out_instrs.push(other.clone());
            }
        }
    }

    return fragment::FunDecl {
        name: fun.name.clone(),
        implementation: fun.implementation.map(|old_impl| FunImpl {
            params: old_impl.params.clone(),
            body: out_instrs,
        }),
        refs: HashMap::new(),
    };

    // let mut idx = 0;
    // let instrs = match &mut fun.implementation.value.body.kind {
    //     wast::core::FuncKind::Inline { expression, .. } => &mut expression.instrs,
    //     _ => panic!("unexpected function kind"),
    // };
    // loop {
    //     if idx > instrs.len() - 2 {
    //         break;
    //     }

    //     let [ref curr, ref next] = instrs[idx..idx + 2] else {
    //         break;
    //     };

    //     match (curr, next) {
    //         (
    //             wast::core::Instruction::I32Const(val),
    //             wast::core::Instruction::CallIndirect { .. },
    //         ) => {
    //             let name = ustr::ustr(
    //                 fun.refs
    //                     .get(&GlobalIdent { id: *val as u32 })
    //                     .unwrap_or_else(|| panic!("unknown function index: {}", val))
    //                     .0
    //                     .as_str(),
    //             )
    //             .as_str();
    //             instrs[idx] = wast::core::Instruction::Nop;
    //             instrs[idx + 1] = wast::core::Instruction::Call(wast::token::Index::Id(
    //                 wast::token::Id::new(name, wast::token::Span::from_offset(0)),
    //             ));
    //             idx += 2;
    //         }
    //         _ => {
    //             idx += 1;
    //         }
    //     }
    // }
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
            .unwrap_or_else(|| panic!("unknown id: {:?}", id))
            .clone()
    }
}
