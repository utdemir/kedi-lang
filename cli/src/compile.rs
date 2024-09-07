use crate::args::CompileArgs;
use kedi_lang::{error::annotate_error, util};

pub fn compile(opts: CompileArgs) -> Result<(), miette::Report> {
    // Read input file.
    let contents = opts.entry.read_to_string().expect("Could not read file");

    let stdlib_ = kedi_lang::runner::stdlib();

    let syntax =
        kedi_lang::parser::parse(&contents).map_err(|e| annotate_error(e, contents.clone()))?;
    if let Some(out_syntax) = opts.out_syntax {
        write_sexpr(&syntax, &out_syntax)
    }

    let plain =
        kedi_lang::renamer::rename(&syntax).map_err(|e| annotate_error(e, contents.clone()))?;

    if let Some(out_plain) = opts.out_plain {
        write_sexpr(&plain, &out_plain)
    }

    let simple = kedi_lang::simplifier::run(&plain);
    if let Some(out_simple) = opts.out_simple {
        write_sexpr(&simple, &out_simple)
    }

    let fragment_ = kedi_lang::codegen::run(&simple);
    let fragment = stdlib_.add(&fragment_);

    if let Some(out_fragment) = opts.out_fragment {
        write_sexpr(&fragment, &out_fragment)
    }

    let linked = kedi_lang::linker::run(&fragment);
    if let Some(out_linked) = opts.out_linked {
        write_sexpr(&linked, &out_linked)
    }

    let wasm = kedi_lang::linker::mk_wasm(&linked);
    if let Some(out_wat) = opts.out_wat {
        let txt = match wasm.to_wat() {
            Some(wat) => wat.text,
            None => "Could not convert to wat".to_string(),
        };
        out_wat.write(txt).expect("Could not write file");
    }

    opts.out.write(wasm.bytes).expect("Could not write file");

    Ok(())
}

fn write_sexpr<T: util::pp::SExpr>(sexpr: &T, path: &patharg::OutputArg) {
    path.write(util::pp::print(sexpr, &util::pp::Options::default()) + "\n")
        .expect("Could not write file");
}
