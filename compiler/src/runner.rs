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
    let std_ = stdlib();

    let syntax = parser::parse(source)?;
    let plain = renamer::rename(&syntax)?;
    let simple = simplifier::run(&plain);
    let fragment = codegen::run(&simple);

    let combined = std_.add(&fragment);

    let linked = linker::run(&combined);
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
    let str = include_str!("../lib/prelude.kedi");
    let syntax = parser::parse(str).unwrap();
    let plain = renamer::rename(&syntax).unwrap();
    let simple = simplifier::run(&plain);
    let mut fragment = codegen::run(&simple);
    for stmt in fragment.statements.iter_mut() {
        match stmt {
            codegen::fragment::TopLevelStmt::FunDecl(fun) => {
                fun.value.export = false;
            }
        }
    }
    fragment
}
