use std::{collections::HashMap, io::Write};

use pinentry::PassphraseInput;
use secrecy::{ExposeSecret, Secret};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

use diva_livomo::{foliate, hypothesis, options, save, set_diff_flag};

fn get_hypothesis_token() -> anyhow::Result<Secret<String>> {
    let mut schema_attr = HashMap::new();
    schema_attr.insert("Type", libsecret::SchemaAttributeType::String);
    schema_attr.insert("App", libsecret::SchemaAttributeType::String);
    let schema = libsecret::Schema::new("moe.gyara.diva_livomo", libsecret::SchemaFlags::NONE, schema_attr);

    let mut attr = HashMap::new();
    attr.insert("Type", "Token");
    attr.insert("App", "Diva Līvõmō");

    let none = Option::<&gio::Cancellable>::None;
    let token = libsecret::password_lookup_sync(Some(&schema), attr.clone(), none)?;
    if let Some(token) = token {
        return Ok(Secret::new(token.to_string()));
    }

    let token = if let Some(mut input) = PassphraseInput::with_binary("pinentry-qt") {
        // pinentry binary is available!
        input
            .with_description("Enter new Token for Hypotheis")
            .with_prompt("Token:")
            .interact()
            .map_err(|e| anyhow::anyhow!("pinentry error: {}", e))?
    } else {
        return Err(anyhow::anyhow!("No pinentry"));
    };

    // store token in keyring
    libsecret::password_store_sync(Some(&schema), attr, None, "diva livomo", token.expose_secret(), none)?;
    Ok(token)
}

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
            .map_err(|e| log::error!("foliate error: {}", e))
            .map(|mut md| output.append(&mut md))
            .ok();
    }
    if hypothesis {
        let token = get_hypothesis_token()?;
        hypothesis::print(token)
            .map_err(|e| log::error!("hypothesis error: {}", e))
            .map(|mut md| output.append(&mut md))
            .ok();
    }
    let markdown = output.join("\n");
    let mut o: std::io::Stdout = std::io::stdout();
    let _ = o.write(markdown.as_bytes())?;
    save()?;
    Ok(())
}
