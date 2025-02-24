mod args;
mod compile;
mod run;

fn main() -> Result<(), miette::Report> {
    let args = args::run();
    match args.command {
        args::Command::Compile(opts) => compile::compile(opts),
        args::Command::Run(opts) => run::run(opts),
    }
}
