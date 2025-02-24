use wasmparser::{self, WasmFeatures};

pub use wasm_exec::{execute_wasm, ExecuteWasmResult};

#[allow(dead_code)]
pub fn assert_valid_wasm(wasm: &[u8]) {
    wasmparser::Validator::new_with_features(
        WasmFeatures::GC
            | WasmFeatures::REFERENCE_TYPES
            | WasmFeatures::EXCEPTIONS
    )
        .validate_all(wasm)
        .expect("invalid wasm");
}
