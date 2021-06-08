use diva_livomo::{foliate, options, save, set_diff_flag};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;
    let options::Opts { foliate, no_diff } = options::parse();
    set_diff_flag(!no_diff);
    if foliate {
        let md = foliate::print()?;
        println!("{}", md);
    }
    save()?;
    Ok(())
}
