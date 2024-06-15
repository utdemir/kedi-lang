mod utils;

use kedi_lang::{codegen, parser, renamer, simplifier};
use test_each_file::test_each_file;

test_each_file! { in "./compiler/tests/data/examples" => test }

fn test(src: &str) {
    let syntax = parser::parse(src).expect("Could not parse file");
    let renamed = renamer::rename(&syntax);
    let simplified = simplifier::run(&renamed);
    let wasm = codegen::run(&simplified);

    utils::validate_wasm(&wasm.bytes);
}
