use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
pub struct Opts {
    /// import from foliate
    #[clap(short, long)]
    pub foliate: bool,
    /// import all content
    #[clap(long = "nodiff")]
    pub no_diff: bool,
    /// print all info
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(short, long)]
    pub kindle: Option<PathBuf>,
}

pub fn parse() -> Opts {
    Opts::parse()
}
