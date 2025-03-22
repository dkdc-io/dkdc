use crate::config::get_config_path;
use crate::terminal::print_message;
use std::env;
use std::process::Command;

pub fn config_it() {
    let config_file = get_config_path();

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    //print_message(format!("opening {} with {}...", config_file, editor));
    // fix the "expected &str, found String" error
    print_message(
        "dkdc",
        &format!(
            "opening {} with {}...",
            config_file.to_str().expect("failed to convert to str"),
            editor
        ),
    );

    let status = Command::new(editor)
        .arg(&config_file)
        .status()
        .expect("failed to execute process");

    if !status.success() {
        eprintln!("Failed to edit config file");
    }
}
