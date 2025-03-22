use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

const DKDC_DIR: &str = ".dkdc";
const DKDC_CONFIG: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub open: Option<Open>,
}

#[derive(Debug, Deserialize)]
pub struct Open {
    pub things: Option<HashMap<String, String>>,
    pub aliases: Option<HashMap<String, String>>,
}

pub fn get_dkdc_dir() -> PathBuf {
    env::var("DKDC_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("HOME not set");
            let mut path = PathBuf::from(home);
            path.push(DKDC_DIR);
            if !path.exists() {
                fs::create_dir_all(&path).expect("failed to create .dkdc directory");
            }
            path
        })
}

pub fn get_config_path() -> PathBuf {
    let mut path = get_dkdc_dir();
    path.push(DKDC_CONFIG);
    path
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    if !config_path.exists() {
        return Ok(Config { open: None });
    }
    let contents = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
