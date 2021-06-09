use diva_livomo::{foliate, options, save, set_diff_flag};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

fn main() -> anyhow::Result<()> {
    let options::Opts {
        foliate,
        no_diff,
        verbose,
    } = options::parse();
    let level = if verbose { LevelFilter::Info } else { LevelFilter::Error };
    TermLogger::init(level, Config::default(), TerminalMode::Stderr, ColorChoice::Auto)?;
    set_diff_flag(!no_diff);
    if foliate {
        let md = foliate::print()?;
        println!("{}", md);
    }
    save()?;
    Ok(())
}
