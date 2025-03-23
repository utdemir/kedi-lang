mod args;
mod compile;
mod run;

// fn main() -> Result<(), miette::Report> {
//     let args = args::run();
//     match args.command {
//         args::Command::Compile(opts) => compile::compile(opts),
//         args::Command::Run(opts) => run::run(opts),
//     }
// }

fn main() {
    print!("{}", f(0));
}

fn f(i: i32) -> i32 {
    f(i)
}
