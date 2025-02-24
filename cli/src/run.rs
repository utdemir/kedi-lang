use crate::args::RunArgs;
use kedi_lang::error::annotate_error;
use wasm_exec::execute_wasm;

pub fn run(opts: RunArgs) -> Result<(), miette::Report> {
    let parameters = opts.parameters.unwrap_or_default();
    let export = opts.export.as_deref().unwrap_or("main");

    // Read input file.
    let contents = opts.entry.read_to_string().expect("Could not read file");

    // let syntax =
    //     kedi_lang::parser::parse(&contents).map_err(|e| annotate_error(e, contents.clone()))?;
    // let plain =
    //     kedi_lang::renamer::rename(&syntax).map_err(|e| annotate_error(e, contents.clone()))?;
    // let simple = kedi_lang::simplifier::run(&plain);

    // let result = execute_wasm(&wasm.bytes, export, parameters.as_slice());
    // println!("{:?}", result);
    Ok(())
}
