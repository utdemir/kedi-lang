#[derive(Debug, PartialEq, Eq)]
pub enum ExecuteWasmResult {
    Ok(i32),
    Timeout(),
}

pub fn execute_wasm(wasm: &[u8], export: &str, inputs: &[i32]) -> ExecuteWasmResult {
    let engine = wasmtime::Engine::new(
        wasmtime::Config::new()
            .cranelift_opt_level(wasmtime::OptLevel::Speed)
            .debug_info(true)
            .consume_fuel(true)
            // WASM proposals
            .wasm_function_references(true)
            .wasm_gc(true)
            .wasm_reference_types(true),
    )
    .unwrap();

    let module = wasmtime::Module::from_binary(&engine, wasm).unwrap();

    let mut store = wasmtime::Store::new(&engine, ());
    store.set_fuel(10_000).unwrap();

    let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();

    let answer = instance
        .get_func(&mut store, export)
        .expect("this was not an exported function");

    let inp = inputs
        .iter()
        .map(|x| wasmtime::Val::I32(*x))
        .collect::<Vec<_>>();
    let mut out = [wasmtime::Val::I32(0)];
    match answer.call(&mut store, &inp, &mut out) {
        Ok(_) => {}
        Err(e) => {
            if e.downcast_ref::<wasmtime::Trap>().is_some() {
                return ExecuteWasmResult::Timeout();
            }
            panic!("failed to execute wasm: {:?}", e);
        }
    }

    ExecuteWasmResult::Ok(out[0].unwrap_i32())
}
