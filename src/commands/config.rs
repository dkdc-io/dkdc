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
            config_file.to_str().ok_or_else(|| Error::Config("Failed to convert path to string".to_string()))?,
            editor
        ),
    );

    let status = Command::new(editor)
        .arg(&config_file)
        .status()
        .map_err(|e| Error::Command(format!("Failed to execute editor: {}", e)))?;

    if !status.success() {
        return Err(Error::Command("Editor exited with non-zero status".to_string()));
    }

    Ok(())
}
