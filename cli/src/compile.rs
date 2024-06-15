use crate::args::CompileArgs;
use kedi_lang::pp;

pub fn compile(opts: CompileArgs) {
    // Read input file.
    let contents = opts.entry.read_to_string().expect("Could not read file");

    let syntax = kedi_lang::parser::parse(&contents).expect("Could not parse file");
    if let Some(out_syntax) = opts.out_syntax {
        write_sexpr(&syntax, &out_syntax)
    }

    let plain = kedi_lang::renamer::rename(&syntax);
    if let Some(out_plain) = opts.out_plain {
        write_sexpr(&plain, &out_plain)
    }

    let simple = kedi_lang::simplifier::run(&plain);
    if let Some(out_simple) = opts.out_simple {
        write_sexpr(&simple, &out_simple)
    }

    let wasm = kedi_lang::codegen::run(&simple);

    if let Some(out_wat) = opts.out_wat {
        let txt = match wasm.to_wat() {
            Some(wat) => wat.text,
            None => "Could not convert to wat".to_string(),
        };
        out_wat.write(txt).expect("Could not write file");
    }

    opts.out.write(wasm.bytes).expect("Could not write file");
}

fn write_sexpr<T: pp::SExpr>(sexpr: &T, path: &patharg::OutputArg) {
    path.write(pp::print(sexpr, &pp::Options::default()) + "\n")
        .expect("Could not write file");
}
