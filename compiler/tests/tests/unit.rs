use crate::mk_tests;
use crate::util;

mk_tests! {
    id: assert_example("id", "id", &[42], 42),
    assignment: assert_example("assignment", "assignment", &[], 2),
    // fibonacci: assert_example("fibonacci", "fibonacci", &[10], 55),
    is_greater_than_five_true: assert_example("if", "is_greater_than_five", &[6], 1),
    is_greater_than_five_false: assert_example("if", "is_greater_than_five", &[5], 0),

    fibonacci_0: assert_example("fibonacci", "fibonacci", &[0], 0),
    fibonacci_1: assert_example("fibonacci", "fibonacci", &[1], 1),
    fibonacci_2: assert_example("fibonacci", "fibonacci", &[2], 1),
    fibonacci_3: assert_example("fibonacci", "fibonacci", &[3], 2),
    fibonacci_4: assert_example("fibonacci", "fibonacci", &[4], 3),
    fibonacci_5: assert_example("fibonacci", "fibonacci", &[5], 5),
    fibonacci_6: assert_example("fibonacci", "fibonacci", &[6], 8),
    fibonacci_7: assert_example("fibonacci", "fibonacci", &[7], 13),
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
