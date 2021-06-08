use clap::Clap;

#[derive(Clap, Debug)]
pub struct Opts {
    /// import from foliate
    #[clap(short, long)]
    pub foliate: bool,
    /// import all content
    #[clap(long = "nodiff")]
    pub no_diff: bool,
}

pub fn parse() -> Opts {
    Opts::parse()
}
