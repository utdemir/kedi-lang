use crate::{codegen, error, parser, renamer, simplifier};

pub struct CompileResult {
    pub syntax: parser::syntax::Module,
    pub plain: renamer::plain::Module,
    pub simple: simplifier::simple::Module,
}

pub struct RunnerOptions {
    // pub include_stdlib: bool,
}

impl Default for RunnerOptions {
    fn default() -> Self {
        Self {
            // include_stdlib: true,
        }
    }
}

pub fn runner(source: &str, _options: RunnerOptions) -> Result<CompileResult, error::Error> {
    let syntax = parser::parse(source)?;
    let plain = renamer::rename(&syntax)?;
    let simple = simplifier::run(&plain);

    Ok(CompileResult {
        syntax,
        plain,
        simple,
    })
}

// pub fn stdlib() -> codegen_wasm::fragment::Module {
//     let str = include_str!("../lib/prelude.kedi");
//     let syntax = parser::parse(str).unwrap();
//     let plain = renamer::rename(&syntax).unwrap();
//     let simple = simplifier::run(&plain);
//     let mut fragment = codegen_wasm::run(&simple);
//     for stmt in fragment.statements.iter_mut() {
//         match stmt {
//             codegen_wasm::fragment::TopLevelStmt::FunDecl(fun) => {
//                 fun.value.export = false;
//             }
//         }
//     }
//     fragment
// }
