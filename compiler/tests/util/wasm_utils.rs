use wasmparser;

pub use wasm_exec::{execute_wasm, ExecuteWasmResult};

#[allow(dead_code)]
pub fn assert_valid_wasm(wasm: &[u8]) {
    wasmparser::validate(wasm).expect("invalid wasm");
}
