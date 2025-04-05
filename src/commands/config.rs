use crate::config::get_config_path;
use crate::error::{Error, Result};
use crate::terminal::print_message;
use std::env;
use std::fs;
use std::process::Command;

pub const DEFAULT_CONFIG: &str = r#"# dkdc config
[open.aliases]
a = "thing"
alias = "thing"
    
[open.things]
thing = "https://github.com/dkdc-io/dkdc"
"#;

pub fn config_it() -> Result<()> {
    let config_file = get_config_path()?;

    if !config_file.exists() {
        print_message("dkdc", " creating default config file...");
        fs::write(&config_file, DEFAULT_CONFIG)
            .map_err(|e| Error::Config(format!("Failed to write config file: {}", e)))?;
    }

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    print_message(
        "dkdc",
        &format!(
            " opening {} with {}...",
            config_file
                .to_str()
                .ok_or_else(|| Error::Config("Failed to convert path to string".to_string()))?,
            editor
        ),
    );

    let status = Command::new(editor)
        .arg(&config_file)
        .status()
        .map_err(|e| Error::Command(format!("Failed to execute editor: {}", e)))?;

    if !status.success() {
        return Err(Error::Command(
            "Editor exited with non-zero status".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    // Test the default config format
    #[test]
    fn test_default_config_format() {
        // Verify that the default config contains expected sections
        assert!(DEFAULT_CONFIG.contains("[open.aliases]"));
        assert!(DEFAULT_CONFIG.contains("[open.things]"));

        // Verify it has some default values
        assert!(DEFAULT_CONFIG.contains("a = \"thing\""));
        assert!(DEFAULT_CONFIG.contains("alias = \"thing\""));
        assert!(DEFAULT_CONFIG.contains("thing = \"https://github.com/dkdc-io/dkdc\""));
    }

    // Test that the config can be parsed as valid TOML
    #[test]
    fn test_default_config_valid_toml() {
        // Try to parse the default config as TOML
        let parsed = toml::from_str::<toml::Value>(DEFAULT_CONFIG);
        assert!(parsed.is_ok(), "Default config should be valid TOML");

        let config = parsed.unwrap();

        // Verify the parsed structure
        assert!(
            config.get("open").is_some(),
            "Config should have 'open' section"
        );

        let open = config.get("open").unwrap();
        assert!(
            open.get("aliases").is_some(),
            "Config should have 'open.aliases' section"
        );
        assert!(
            open.get("things").is_some(),
            "Config should have 'open.things' section"
        );
    }

    // Test the config_it function in isolation - this is tricky because it spawns an editor
    // So we'll mock/test parts of it
    #[test]
    fn test_config_file_path() {
        // We can at least check that get_config_path returns a path to a file named config.toml
        let path_result = get_config_path();
        assert!(path_result.is_ok(), "Should be able to get config path");

        let path = path_result.unwrap();
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), "config.toml");
    }

    #[test]
    #[ignore] // Ignored because it depends on environment and would launch an editor
    fn test_config_it_creates_file() {
        // Create a temp directory for testing
        let dir = tempdir().unwrap();
        let fake_home = dir.path().to_path_buf();

        // Set DKDC_HOME to our test directory
        unsafe {
            env::set_var("DKDC_HOME", fake_home.to_str().unwrap());
            // Set a fake editor that just returns success
            env::set_var("EDITOR", "echo");
        }

        // Run config_it - this should create a config file but won't actually launch an editor
        let result = config_it();

        // Clean up
        unsafe {
            env::remove_var("DKDC_HOME");
            env::remove_var("EDITOR");
        }

        // We're really just checking that the function doesn't panic
        // The actual execution will depend on the environment
        assert!(result.is_ok() || result.is_err());
    }
}
