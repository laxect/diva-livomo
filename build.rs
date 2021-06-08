use clap::IntoApp;
use clap_generate::{generate_to, generators::Zsh};
include!("src/options.rs");

fn main() {
    let mut app = Opts::into_app();
    app.set_bin_name("dival");
    generate_to::<Zsh, _, _>(
        &mut app, // We need to specify what generator to use
        "dival",  // We need to specify the bin name manually
        ".",      // We need to specify where to write to
    );
}
