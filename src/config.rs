use crate::error::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

const DKDC_DIR: &str = ".dkdc";
const DKDC_CONFIG: &str = "config.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub open: Option<Open>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Open {
    pub things: Option<HashMap<String, String>>,
    pub aliases: Option<HashMap<String, String>>,
}

pub fn get_dkdc_dir() -> Result<PathBuf> {
    env::var("DKDC_HOME")
        .map(PathBuf::from)
        .or_else(|_| {
            env::var("HOME")
                .map(|home| {
                    let mut path = PathBuf::from(home);
                    path.push(DKDC_DIR);
                    path
                })
                .map_err(|_| Error::Config("HOME environment variable not set".to_string()))
        })
        .map(|path| {
            if !path.exists() {
                fs::create_dir_all(&path).map_err(|e| Error::Io(e))?;
            }
            Ok(path)
        })?
}

pub fn get_config_path() -> Result<PathBuf> {
    let mut path = get_dkdc_dir()?;
    path.push(DKDC_CONFIG);
    Ok(path)
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Ok(Config { open: None });
    }
    let contents = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn resolve_alias_to_thing(config: &Config, input: &str) -> Result<String> {
    // First check if the input is an alias
    let target = if let Some(ref open) = config.open {
        if let Some(aliases) = &open.aliases {
            if let Some(mapped) = aliases.get(input) {
                mapped.to_owned()
            } else {
                input.to_owned()
            }
        } else {
            input.to_owned()
        }
    } else {
        input.to_owned()
    };

    // Now get the actual thing URL/path
    if let Some(ref open) = config.open {
        if let Some(things) = &open.things {
            if let Some(mapped) = things.get(&target) {
                return Ok(mapped.to_owned());
            }
            return Err(Error::Missing(format!("'{}' not found in config", target)));
        }
        return Err(Error::Missing("No things defined in config".to_string()));
    }
    Err(Error::Missing(
        "Open section not defined in config".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_alias_to_thing() {
        // Create a test config with some aliases and things
        let mut aliases = HashMap::new();
        aliases.insert("gh".to_string(), "github".to_string());
        aliases.insert("yt".to_string(), "youtube".to_string());

        let mut things = HashMap::new();
        things.insert("github".to_string(), "https://github.com".to_string());
        things.insert("youtube".to_string(), "https://youtube.com".to_string());

        let open = Open {
            aliases: Some(aliases),
            things: Some(things),
        };

        let config = Config { open: Some(open) };

        // Test direct thing lookup
        let result = resolve_alias_to_thing(&config, "github");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com");

        // Test alias resolution
        let result = resolve_alias_to_thing(&config, "gh");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com");

        // Test missing thing
        let result = resolve_alias_to_thing(&config, "nonexistent");
        assert!(result.is_err());
        match result {
            Err(Error::Missing(msg)) => {
                assert!(msg.contains("'nonexistent' not found in config"));
            }
            _ => panic!("Expected Missing error"),
        }
    }

    #[test]
    fn test_empty_config() {
        // Test with empty config
        let config = Config { open: None };

        let result = resolve_alias_to_thing(&config, "anything");
        assert!(result.is_err());
        match result {
            Err(Error::Missing(msg)) => {
                assert_eq!(msg, "Open section not defined in config");
            }
            _ => panic!("Expected Missing error"),
        }
    }

    #[test]
    fn test_config_without_things() {
        // Config with aliases but no things
        let mut aliases = HashMap::new();
        aliases.insert("gh".to_string(), "github".to_string());

        let open = Open {
            aliases: Some(aliases),
            things: None,
        };

        let config = Config { open: Some(open) };

        let result = resolve_alias_to_thing(&config, "gh");
        assert!(result.is_err());
        match result {
            Err(Error::Missing(msg)) => {
                assert_eq!(msg, "No things defined in config");
            }
            _ => panic!("Expected Missing error"),
        }
    }
}
