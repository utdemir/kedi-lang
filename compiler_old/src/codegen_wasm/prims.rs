use bimap::BiHashMap;
use wasm_encoder::ValType;

use crate::codegen_wasm::rts::{object_val_type, OBJECT_TAG_I32, OBJECT_TYPE_ID};
use crate::parser::syntax;
use crate::renamer::plain;
use crate::{codegen_wasm::fragment, util::loc::WithLoc};

pub fn prims() -> fragment::Module {
    let stmts = vec![
        mk_prim_i32_binop("gt?", wasm_encoder::Instruction::I32GtU),
        mk_prim_i32_binop("gte?", wasm_encoder::Instruction::I32GeU),
        mk_prim_i32_binop("lt?", wasm_encoder::Instruction::I32LtU),
        mk_prim_i32_binop("lte?", wasm_encoder::Instruction::I32LeU),
        mk_prim_i32_binop("eq?", wasm_encoder::Instruction::I32Eq),
        mk_prim_i32_binop("add", wasm_encoder::Instruction::I32Add),
        mk_pack_i32(),
        mk_unpack_i32(),
    ];

    fragment::Module { statements: stmts }
}

fn mk_prim(
    name: &str,
    params: Vec<ValType>,
    body: Vec<fragment::Instr>,
    refs: BiHashMap<plain::GlobalIdent, syntax::Ident>,
) -> fragment::TopLevelStmt {
    fragment::TopLevelStmt::FunDecl(WithLoc::unknown(fragment::FunDecl {
        name: WithLoc::unknown(syntax::Ident(format!("__prim_{}", name))),
        export: false,
        implementation: WithLoc::unknown(fragment::FunImpl { params, body }),
        refs,
    }))
}

fn mk_prim_i32_binop(
    name: &str,
    instr: wasm_encoder::Instruction<'static>,
) -> fragment::TopLevelStmt {
    mk_prim(
        name,
        vec![ValType::I32, ValType::I32],
        vec![
            fragment::Instr::Raw(wasm_encoder::Instruction::LocalGet(0)),
            fragment::Instr::Call(fragment::Call {
                fun: plain::GlobalIdent { id: 0 },
                arity: 1,
            }),
            fragment::Instr::Raw(wasm_encoder::Instruction::LocalGet(1)),
            fragment::Instr::Raw(instr),
        ],
        BiHashMap::from_iter(vec![(
            plain::GlobalIdent { id: 0 },
            syntax::Ident("__prim_unpack_i32".to_string()),
        )]),
    )
}

fn mk_pack_i32() -> fragment::TopLevelStmt {
    mk_prim(
        "pack_i32",
        vec![ValType::I32],
        vec![
            // Tag
            fragment::Instr::Raw(wasm_encoder::Instruction::I32Const(OBJECT_TAG_I32)),
            // Value
            fragment::Instr::Raw(wasm_encoder::Instruction::LocalGet(0)),
            // Pack
            fragment::Instr::Raw(wasm_encoder::Instruction::StructNew(OBJECT_TYPE_ID)),
        ],
        BiHashMap::new(),
    )
}

fn mk_unpack_i32() -> fragment::TopLevelStmt {
    mk_prim(
        "unpack_i32",
        vec![object_val_type()],
        vec![
            // Start with the param
            fragment::Instr::Raw(wasm_encoder::Instruction::LocalGet(0)),
            // Get the tag
            fragment::Instr::Raw(wasm_encoder::Instruction::StructGet {
                struct_type_index: OBJECT_TYPE_ID,
                field_index: 0,
            }),
            // Check the tag
            fragment::Instr::Raw(wasm_encoder::Instruction::I32Const(OBJECT_TAG_I32)),
            fragment::Instr::Raw(wasm_encoder::Instruction::I32Eq),
            // If the tag is correct, unwrap the value, or throw
            fragment::Instr::Raw(wasm_encoder::Instruction::If(
                wasm_encoder::BlockType::Empty,
            )),
            fragment::Instr::Raw(wasm_encoder::Instruction::LocalGet(0)),
            fragment::Instr::Raw(wasm_encoder::Instruction::StructGet {
                struct_type_index: OBJECT_TYPE_ID,
                field_index: 1,
            }),
            fragment::Instr::Raw(wasm_encoder::Instruction::Else),
            fragment::Instr::Raw(wasm_encoder::Instruction::I32Const(0)),
            fragment::Instr::Raw(wasm_encoder::Instruction::Unreachable),
            fragment::Instr::Raw(wasm_encoder::Instruction::End),
        ],
        BiHashMap::new(),
    )
}
