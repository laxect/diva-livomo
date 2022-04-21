use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opts {
    /// import from foliate
    #[clap(short, long)]
    pub foliate: bool,
    /// import from hypothes.is
    #[clap(short, long)]
    pub hypothesis: bool,
    /// import all content
    #[clap(long = "nodiff")]
    pub no_diff: bool,
    /// print all info
    #[clap(short, long)]
    pub verbose: bool,
}

pub fn parse() -> Opts {
    Opts::parse()
}
