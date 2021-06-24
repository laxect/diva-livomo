use crate::hypothesis::HypothesisConfig;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs, io::Read};

#[derive(Deserialize)]
pub(crate) struct Config {
    pub hypothesis: Option<HypothesisConfig>,
}

pub(crate) static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_path = dirs::config_dir()
        .expect("XDG config not setting")
        .join("diva-līvõmō")
        .join("config.toml");
    let mut config = fs::File::open(&config_path)
        .map_err(|_e| println!("read failed: {}", config_path.to_string_lossy()))
        .unwrap();
    let mut buffer = Vec::new();
    config
        .read_to_end(&mut buffer)
        .map_err(|_e| println!("read failed: {}", config_path.to_string_lossy()))
        .unwrap();
    toml::from_slice(&buffer).expect("config error")
});
