use std::collections::HashMap;

use wasm_encoder::ValType;

use crate::parser::syntax;
use crate::{codegen::fragment, util::loc::WithLoc};

pub fn prims() -> fragment::Module {
    let stmts = vec![
        mk_prim_binop("gt?", wasm_encoder::Instruction::I32GtU),
        mk_prim_binop("gte?", wasm_encoder::Instruction::I32GeU),
        mk_prim_binop("lt?", wasm_encoder::Instruction::I32LtU),
        mk_prim_binop("lte?", wasm_encoder::Instruction::I32LeU),
        mk_prim_binop("eq?", wasm_encoder::Instruction::I32Eq),
        mk_prim_binop("add", wasm_encoder::Instruction::I32Add),
    ];

    fragment::Module { statements: stmts }
}

fn mk_prim(name: &str, params: Vec<ValType>, body: Vec<fragment::Instr>) -> fragment::TopLevelStmt {
    fragment::TopLevelStmt::FunDecl(WithLoc::unknown(fragment::FunDecl {
        name: WithLoc::unknown(syntax::Ident(format!("__prim_{}", name))),
        implementation: WithLoc::unknown(fragment::FunImpl { params, body }),
        refs: HashMap::new(),
    }))
}

fn mk_prim_binop(name: &str, instr: wasm_encoder::Instruction<'static>) -> fragment::TopLevelStmt {
    mk_prim(
        name,
        vec![ValType::I32, ValType::I32],
        vec![
            fragment::Instr::Raw(wasm_encoder::Instruction::LocalGet(0)),
            fragment::Instr::Raw(wasm_encoder::Instruction::LocalGet(1)),
            fragment::Instr::Raw(instr),
        ],
    )
}
