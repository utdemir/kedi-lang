#[cfg(test)]
mod util;

mk_tests! {
    id: test("id"),
    assignment: test("assignment"),
    fibonacci: test("fibonacci"),
    id_with_unused_var: test("id_with_unused_var"),
    two_funs: test("two_funs"),
}

fn test(example_name: &str) {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/data/examples");
    d.push(format!("{}.kedi", example_name));
    let src = std::fs::read_to_string(d).unwrap();
    let out = kedi_lang::runner::runner(&src);
    util::assert_valid_wasm(&out.unwrap().wasm.bytes);
}
