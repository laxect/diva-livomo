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
        foliate::print()
            .map_err(|e| log::error!("hypothesis error :{}", e))
            .map(|md| print!("{}", md))
            .ok();
    }
    if hypothesis {
        hypothesis::print()
            .map_err(|e| log::error!("hypothesis error :{}", e))
            .map(|md| print!("{}", md))
            .ok();
    }
    if let Some(kindle_clippings) = kindle {
        kindle::parse(kindle_clippings)
            .map_err(|e| log::error!("kindle error :{}", e))
            .map(|md| print!("{}", md))
            .ok();
    }
    save()?;
    Ok(())
}
