use wasmparser;
use wasmtime;

#[allow(dead_code)]
pub fn assert_valid_wasm(wasm: &[u8]) {
    wasmparser::validate(wasm).expect("invalid wasm");
}

#[allow(dead_code)]
pub fn execute_wasm(wasm: &[u8], export: &str, inputs: &[i32]) -> i32 {
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::new(&engine, wasm).unwrap();
    let linker = wasmtime::Linker::new(&engine);
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module).unwrap();
    let fun = instance.get_func(&mut store, export).unwrap();

    let mut results = [wasmtime::Val::I32(0)];
    fun.call(
        store,
        &inputs
            .iter()
            .map(|x| wasmtime::Val::I32(*x))
            .collect::<Vec<_>>(),
        &mut results,
    )
    .unwrap();

    return match results[0] {
        wasmtime::Val::I32(x) => x,
        _ => panic!("unexpected return type"),
    };
}
