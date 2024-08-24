use crate::{codegen, error, linker, parser, renamer, simplifier, util::wasm::WasmBytes};

pub struct CompileResult {
    pub syntax: parser::syntax::Module,
    pub plain: renamer::plain::Module,
    pub simple: simplifier::simple::Module,
    pub fragment: codegen::fragment::Module,
    pub linked: linker::linked::Module,
    pub wasm: WasmBytes,
}

pub fn runner(source: &str) -> Result<CompileResult, error::Error> {
    // let _std = stdlib();

    let syntax = parser::parse(source)?;
    let plain = renamer::rename(&syntax)?;
    let simple = simplifier::run(&plain);
    let fragment = codegen::run(&simple);
    let linked = linker::run(&fragment);
    let wasm = linker::mk_wasm(&linked);

    Ok(CompileResult {
        syntax,
        plain,
        simple,
        fragment,
        linked,
        wasm,
    })
}

pub fn stdlib() -> codegen::fragment::Module {
    let str = include_str!("stdlib.kedi");
    let syntax = parser::parse(str).unwrap();
    let plain = renamer::rename(&syntax).unwrap();
    let simple = simplifier::run(&plain);
    let fragment = codegen::run(&simple);
    fragment
}
