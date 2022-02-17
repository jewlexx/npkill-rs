use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(short, long, default_value = "node_modules")]
    pub search: String,

    #[clap(short, long, default_value = ".")]
    pub dir: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
