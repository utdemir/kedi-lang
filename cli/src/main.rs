mod args;
mod compile;

fn main() -> Result<(), miette::Report> {
    let args = args::run();
    match args.command {
        args::Command::Compile(opts) => compile::compile(opts),
    }
}
