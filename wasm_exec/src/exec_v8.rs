use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Eq)]
pub enum ExecuteWasmResult {
    Ok(i32),
    Timeout(),
}

pub fn execute_wasm(wasm: &[u8], export: &str, inputs: &[i32]) -> ExecuteWasmResult {
    // v8::V8::set_flags_from_string("--experimental-wasm-gc");

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

    let handle = isolate.thread_safe_handle();

    let handle_scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope, v8::ContextOptions::default());
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let c_source = format!(
        r#"
        let bytes = new Uint8Array({:?});
        let module = new WebAssembly.Module(bytes);
        let instance = new WebAssembly.Instance(module);
        const ret = instance.exports.{}({:?});
        12;
        "#,
        wasm, export, inputs
    );

    // write to file
    // let path = Path::new("/Users/utdemir/tmp/test.tmp.js");
    // let mut file = std::fs::File::create(path).unwrap();
    // file.write(c_source.as_bytes()).unwrap();
    // file.flush().unwrap();

    let source = v8::String::new(scope, &c_source).unwrap();
    let script = v8::Script::compile(scope, source, None).unwrap();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        // No need to be careful here as 'terminate_execution' is safe to call
        // even if the isolate is already disposed.
        handle.terminate_execution();
    });

    let result = match script.run(scope) {
        Some(result) => {
            println!("GOT result");
            println!("result: {:?}", result.to_object(scope));
            let result = result.to_integer(scope).unwrap();
            ExecuteWasmResult::Ok(result.value() as i32)
        }
        None => ExecuteWasmResult::Timeout(),
    };

    // TODO: Do we need to dispose anything here? Some examples
    // call `dispose` in an unsafe block here but apparently `Drop`
    // also does that, so might be unnecessary. Well, YOLO.

    result
}
