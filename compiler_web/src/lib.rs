use wasm_bindgen::prelude::*;

use base64::Engine;
use kedi_lang::{self, util::pp::SExpr as _};
use serde::Serialize;

#[derive(Serialize)]
pub struct CompileSuccessWeb {
    pub syntax: String,
    pub plain: String,
    pub simple: String,
    pub wat: String,
    pub wasm: String,
}

#[wasm_bindgen(typescript_custom_section)]
const CompileSuccessWeb: &'static str = r#"
interface CompileSuccessWeb {
    syntax: string;
    plain: string;
    simple: string;
    wat: string;
    wasm: string;
}
"#;

#[derive(Serialize)]
pub struct CompileErrorWeb {
    pub message: String,
}

#[wasm_bindgen(typescript_custom_section)]
const CompileErrorWeb: &'static str = r#"
interface CompileErrorWeb {
    message: string;
}
"#;

#[derive(Serialize)]
pub enum CompileResultWeb {
    Success(CompileSuccessWeb),
    Error(CompileErrorWeb),
}

#[wasm_bindgen(typescript_custom_section)]
const CompileResultWeb: &'static str = r#"
type CompileResultWeb = 
    { Success: CompileSuccessWeb } |
    { Error: CompileErrorWeb };
"#;

#[wasm_bindgen]
pub fn runner(source: &str) -> JsValue {
    let runner = kedi_lang::runner::runner(source);

    match runner {
        Ok(result) => {
            let success = CompileSuccessWeb {
                syntax: result.syntax.to_sexpr().to_pretty_string(),
                plain: result.plain.to_sexpr().to_pretty_string(),
                simple: result.simple.to_sexpr().to_pretty_string(),
                wat: result.wasm.to_wat().unwrap().text,
                wasm: base64::engine::general_purpose::STANDARD.encode(result.wasm.bytes),
            };

            let ret = CompileResultWeb::Success(success);

            serde_wasm_bindgen::to_value(&ret).unwrap()
        }
        Err(err) => {
            let error = CompileErrorWeb {
                message: format!("{:?}", err),
            };

            let ret = CompileResultWeb::Error(error);

            serde_wasm_bindgen::to_value(&ret).unwrap()
        }
    }
}
