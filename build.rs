use clap::IntoApp;
use clap_generate::{generate_to, generators::Zsh};
include!("src/options.rs");

fn main() {
    let mut app = Opts::into_app();
    app.set_bin_name("dival");
    generate_to(Zsh, &mut app, "dival", ".").unwrap();
}
