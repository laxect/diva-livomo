use diva_livomo::{foliate, save};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;
    let fos = foliate::load().unwrap();
    for mut item in fos.into_iter() {
        item.remove_old();
        if item.has_annotation() {
            println!("{}", item.to_md());
        }
        item.mark_as_old();
    }
    save()?;
    Ok(())
}
