use clap::Parser as _;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Compile(CompileArgs),
    Run(RunArgs),
}

#[derive(clap::Args, Debug)]
pub struct CompileArgs {
    pub entry: patharg::InputArg,

    #[arg(long)]
    pub out: patharg::OutputArg,

    #[arg(long)]
    pub out_syntax: Option<patharg::OutputArg>,
    #[arg(long)]
    pub out_plain: Option<patharg::OutputArg>,
    #[arg(long)]
    pub out_simple: Option<patharg::OutputArg>,
    #[arg(long)]
    pub out_fragment: Option<patharg::OutputArg>,
    #[arg(long)]
    pub out_linked: Option<patharg::OutputArg>,
    #[arg(long)]
    pub out_wat: Option<patharg::OutputArg>,
}

#[derive(clap::Args, Debug)]
pub struct RunArgs {
    pub entry: patharg::InputArg,

    #[arg(long)]
    pub export: Option<String>,

    #[arg(long)]
    pub parameters: Option<Vec<i32>>,
}

pub fn run() -> Args {
    Args::parse()
}
