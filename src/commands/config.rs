use crate::config::get_config_path;
use crate::terminal::print_message;
use std::env;
use std::process::Command;

pub const DEFAULT_CONFIG: &str = r#"# dkdc config
[open.aliases]
a = "thing"
alias = "thing"
    
[open.things]
thing = "https:://github.com/dkdc-io/dkdc"
"#;

pub fn config_it() {
    let config_file = get_config_path();

    if !config_file.exists() {
        print_message("dkdc", " creating default config file...");
        std::fs::write(&config_file, DEFAULT_CONFIG).expect("failed to write config file");
    }

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    print_message(
        "dkdc",
        &format!(
            " opening {} with {}...",
            config_file.to_str().expect("failed to convert to str"),
            editor
        ),
    );

    let status = Command::new(editor)
        .arg(&config_file)
        .status()
        .expect("failed to execute process");

    if !status.success() {
        eprintln!("failed to edit config file");
    }
}
