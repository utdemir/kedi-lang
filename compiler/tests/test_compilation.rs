#[cfg(test)]
mod util;

use test_each_file::test_each_file;

test_each_file! { in "./compiler/tests/data/examples" => test }

fn test(src: &str) {
    let out = kedi_lang::runner::runner(src);
    util::assert_valid_wasm(&out.unwrap().wasm.bytes);
}
