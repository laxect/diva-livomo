use std::io::Write;

use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

use diva_livomo::{foliate, hypothesis, options, save, set_diff_flag};

fn main() -> anyhow::Result<()> {
    let options::Opts {
        foliate,
        hypothesis,
        no_diff,
        verbose,
    } = options::parse();
    let level = if verbose { LevelFilter::Info } else { LevelFilter::Error };
    TermLogger::init(level, Config::default(), TerminalMode::Stderr, ColorChoice::Auto)?;
    set_diff_flag(!no_diff);
    let mut output = Vec::new();
    if foliate {
        foliate::print()
            .map_err(|e| log::error!("foliate error: {e}"))
            .map(|mut md| output.append(&mut md))
            .ok();
    }
    if hypothesis {
        hypothesis::print()
            .map_err(|e| log::error!("hypothesis error: {e}"))
            .map(|mut md| output.append(&mut md))
            .ok();
    }
    let markdown = output.join("\n");
    let mut o: std::io::Stdout = std::io::stdout();
    let _ = o.write(markdown.as_bytes())?;
    save()?;
    Ok(())
}
