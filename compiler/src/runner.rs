use crate::{codegen, error, linker, parser, renamer, simplifier, util::wasm::WasmBytes};

pub struct CompileResult {
    pub wasm: WasmBytes,
    pub syntax: parser::syntax::Module,
    pub plain: renamer::plain::Module,
    pub simple: simplifier::simple::Module,
}

pub fn runner(source: &str) -> Result<CompileResult, error::Error> {
    let syntax = parser::parse(source)?;
    let plain = renamer::rename(&syntax)?;
    let simple = simplifier::run(&plain);
    let fragment = codegen::run(&simple);
    let wasm = linker::run(fragment);

    Ok(CompileResult {
        wasm,
        syntax: syntax.clone(),
        plain: plain.clone(),
        simple: simple.clone(),
    })
}
