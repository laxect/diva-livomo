use clap::Clap;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long)]
    pub foliate: bool,
    #[clap(long = "nodiff")]
    pub no_diff: bool,
}

pub fn parse() -> Opts {
    Opts::parse()
}
