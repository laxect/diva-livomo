use diva_livomo::{foliate, hypothesis, kindle, options, save, set_diff_flag};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

fn main() -> anyhow::Result<()> {
    let options::Opts {
        foliate,
        hypothesis,
        no_diff,
        verbose,
        kindle,
    } = options::parse();
    let level = if verbose { LevelFilter::Info } else { LevelFilter::Error };
    TermLogger::init(level, Config::default(), TerminalMode::Stderr, ColorChoice::Auto)?;
    set_diff_flag(!no_diff);
    if foliate {
        let md = foliate::print()?;
        print!("{}", md);
    }
    if hypothesis {
        let md = hypothesis::print()?;
        print!("{}", md);
    }
    if let Some(kindle_clippings) = kindle {
        let md = kindle::parse(kindle_clippings)?;
        print!("{}", md);
    }
    save()?;
    Ok(())
}
