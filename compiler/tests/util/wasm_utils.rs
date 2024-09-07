use rusty_v8 as v8;
use wasmparser;

pub fn execute_wasm(wasm: &[u8], export: &str, inputs: &[i32]) -> i32 {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
    let handle_scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope);
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let c_source = format!(
        r#"
        let bytes = new Uint8Array({:?});
        let module = new WebAssembly.Module(bytes);
        let instance = new WebAssembly.Instance(module);
        instance.exports.{}({:?});
        "#,
        wasm, export, inputs
    );

    let source = v8::String::new(scope, &c_source).unwrap();
    let script = v8::Script::compile(scope, source, None).unwrap();

    let result = script.run(scope).unwrap();

    // Convert the result to an integer and return it.
    let result = result.to_integer(scope).unwrap();
    let result_value = result.value() as i32;

    // TODO: Do we need to dispose anything here? Some examples
    // call `dispose` in an unsafe block here but apparently `Drop`
    // also does that, so might be unnecessary. Well, YOLO.

    result_value
}

#[allow(dead_code)]
pub fn assert_valid_wasm(wasm: &[u8]) {
    wasmparser::validate(wasm).expect("invalid wasm");
}
