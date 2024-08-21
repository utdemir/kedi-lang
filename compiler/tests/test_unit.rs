#[cfg(test)]
mod util;

mk_tests! {
    id: assert_example("id", "id", &[42], 42),
    assignment: assert_example("assignment", "assignment", &[], 2),
}

//

fn assert_example(example_name: &str, entrypoint: &str, params: &[i32], expected: i32) {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/data/examples");
    d.push(format!("{}.kedi", example_name));

    let src = std::fs::read_to_string(d).unwrap();
    let out = kedi_lang::runner::runner(&src).unwrap();
    let result = util::execute_wasm(&out.wasm.bytes, entrypoint, params);

    assert_eq!(result, expected, "expected {} but got {}", expected, result);
}
