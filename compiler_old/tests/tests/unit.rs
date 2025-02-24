use std::default;

use kedi_lang::interpreter::{
    InterpretOptions, InterpretOutOfFuel, InterpretResult, InterpretSuccess, KediValue,
};
use kedi_lang::runner::RunnerOptions;

use crate::mk_tests;
use crate::util::ExecuteWasmResult;

mk_tests! {
    id: assert_example("id", "id", &[42.into()], 42.into()),
    assignment: assert_example("assignment", "assignment", &[], 2.into()),
    // fibonacci: assert_example("fibonacci", "fibonacci", &[10], 55),
    is_greater_than_five_true: assert_example("if", "is_greater_than_five", &[6.into()], 1.into()),
    is_greater_than_five_false: assert_example("if", "is_greater_than_five", &[5.into()], 0.into()),

    fibonacci_0: assert_example("fibonacci", "fibonacci", &[0.into()], 0.into()),
    fibonacci_1: assert_example("fibonacci", "fibonacci", &[1.into()], 1.into()),
    fibonacci_2: assert_example("fibonacci", "fibonacci", &[2.into()], 1.into()),
    fibonacci_3: assert_example("fibonacci", "fibonacci", &[3.into()], 2.into()),
    fibonacci_4: assert_example("fibonacci", "fibonacci", &[4.into()], 3.into()),
    fibonacci_5: assert_example("fibonacci", "fibonacci", &[5.into()], 5.into()),
    fibonacci_6: assert_example("fibonacci", "fibonacci", &[6.into()], 8.into()),
    fibonacci_7: assert_example("fibonacci", "fibonacci", &[7.into()], 13.into()),

    infinite_loop: assert_example_result("infinite_loop", "infinite_loop", &[],
        InterpretResult::OutOfFuel(
            InterpretOutOfFuel {}
        )
    ),
}

//

fn assert_example(example_name: &str, entrypoint: &str, params: &[KediValue], expected: KediValue) {
    assert_example_result(
        example_name,
        entrypoint,
        params,
        InterpretResult::Success(InterpretSuccess {
            value: expected,
            fuel_used: 0,
        }),
    );
}

fn assert_example_result(
    example_name: &str,
    entrypoint: &str,
    params: &[KediValue],
    expected: InterpretResult,
) {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/data/examples");
    d.push(format!("{}.kedi", example_name));

    let src = std::fs::read_to_string(d).unwrap();
    let out = kedi_lang::runner::runner(&src, RunnerOptions {}).unwrap();
    let actual = kedi_lang::interpreter::run(
        InterpretOptions {
            fuel_limit: Some(10000),
        },
        &out.simple,
        entrypoint,
        params.to_vec(),
    );

    let actual = match actual {
        InterpretResult::Success(InterpretSuccess { value, .. }) => {
            InterpretResult::Success(InterpretSuccess {
                value,
                fuel_used: 0,
            })
        }
        other => other,
    };

    assert_eq!(
        actual, expected,
        "Expected {:?}, got {:?}",
        expected, actual
    );
}
