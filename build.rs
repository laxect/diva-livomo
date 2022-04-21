use clap::IntoApp;
use clap_complete::{generate_to, Shell};
include!("src/options.rs");

fn main() {
    let mut app = Opts::command();
    app.set_bin_name("dival");
    generate_to(Shell::Zsh, &mut app, "dival", ".").unwrap();
}
