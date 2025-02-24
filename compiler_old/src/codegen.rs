use crate::error;
use crate::simplifier::simple;

trait CodegenTransform<In, Out, Err> {
    fn transform(&self, input: In) -> Result<Out, Err>;
}

trait CodegenLinker<In, Out, Err> {
    fn link(&self, input: In) -> Result<Out, Err>;
}

pub enum CodegenTarget {
    Wasm,
    Js,
}

pub struct CodegenOptions {
    pub target: CodegenTarget,
}

pub fn run(options: CodegenOptions, simple: simple::Module) -> Result<Vec<u8>, error::Error> {
    match options.target {
        // WASM
        ////////
        #[cfg(feature = "codegen-wasm")]
        CodegenTarget::Wasm => {
            let fragment = fragment::run(&simple);
            let linked = linker::run(&fragment);
            let wasm = linker::mk_wasm(&linked);
            Ok(wasm)
        }
        #[cfg(not(feature = "codegen-wasm"))]
        CodegenTarget::Wasm => panic!("Wasm codegen not enabled"),

        // JS
        ////////
        #[cfg(feature = "codegen-js")]
        CodegenTarget::Js => {
            let fragment = fragment::run(&simple);
            let linked = linker::run(&fragment);
            let js = linker::mk_js(&linked);
            Ok(js.into_bytes())
        }
        #[cfg(not(feature = "codegen-js"))]
        CodegenTarget::Js => panic!("JS codegen not enabled"),
    }
}
