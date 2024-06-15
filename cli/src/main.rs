mod args;
mod compile;

fn main() {
    let args = args::run();
    match args.command {
        args::Command::Compile(opts) => compile::compile(opts),
    }
}
