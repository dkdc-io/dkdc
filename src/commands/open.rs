use crate::config::{load_config, resolve_alias_to_thing};
use crate::error::Result;
use crate::terminal::{bottom_border, format_line, get_terminal_width, print_message, top_border};
use std::process::Command;

pub fn open_it(input: &str) -> Result<()> {
    let config = load_config()?;
    let thing = resolve_alias_to_thing(&config, input)?;

    print_message("dkdc", &format!(" opening: {}...", thing));

    Command::new("open")
        .arg(&thing)
        .spawn()
        .map_err(|e| crate::error::Error::Command(format!("Failed to open: {}", e)))?;

    Ok(())
}

pub fn print_config() {
    let width = get_terminal_width();
    let title = "dkdc";
    let mut lines = Vec::new();
    lines.push(top_border(title, width));

    match load_config() {
        Ok(config) => {
            lines.push(format_line(" aliases:", width));
            lines.push(format_line("", width));
            if let Some(ref open) = config.open {
                if let Some(ref aliases) = open.aliases {
                    if aliases.is_empty() {
                        lines.push(format_line(" no aliases defined", width));
                    } else {
                        let max_key_len = aliases.keys().map(|k| k.len()).max().unwrap_or(0);
                        let mut aliases_vec: Vec<(&String, &String)> = aliases.iter().collect();
                        aliases_vec.sort_by(|(a, _), (b, _)| {
                            let cmp = a.len().cmp(&b.len());
                            if cmp == std::cmp::Ordering::Equal {
                                a.cmp(b)
                            } else {
                                cmp
                            }
                        });
                        for (alias, target) in aliases_vec {
                            let formatted =
                                format!(" • {:<width$} | {}", alias, target, width = max_key_len);
                            lines.push(format_line(&formatted, width));
                        }
                    }
                } else {
                    lines.push(format_line(" no aliases defined", width));
                }

                lines.push(format_line("", width));
                lines.push(format_line(" things:", width));
                lines.push(format_line("", width));

                if let Some(ref things) = open.things {
                    if things.is_empty() {
                        lines.push(format_line(" no things defined", width));
                    } else {
                        let max_key_len = things.keys().map(|k| k.len()).max().unwrap_or(0);
                        let mut things_vec: Vec<(&String, &String)> = things.iter().collect();
                        things_vec.sort_by(|(a, _), (b, _)| {
                            let cmp = a.len().cmp(&b.len());
                            if cmp == std::cmp::Ordering::Equal {
                                a.cmp(b)
                            } else {
                                cmp
                            }
                        });
                        for (thing, target) in things_vec {
                            let formatted =
                                format!(" • {:<width$} | {}", thing, target, width = max_key_len);
                            lines.push(format_line(&formatted, width));
                        }
                    }
                } else {
                    lines.push(format_line(" no things defined", width));
                }
            } else {
                lines.push(format_line(" no config available", width));
            }
        }
        Err(e) => {
            lines.push(format_line(&format!(" error loading config: {}", e), width));
        }
    }

    lines.push(bottom_border(width));

    for line in lines {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, Open};
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    // Mock the load_config function for testing
    #[test]
    fn test_open_it_nonexistent_thing() {
        // This test purposely doesn't mock config to test the error path
        // when a thing doesn't exist in config or the config file doesn't exist

        // Use a non-existent thing
        let result = open_it("nonexistent_thing");

        // Should result in an error
        assert!(result.is_err());
    }

    #[test]
    fn test_print_config_format() {
        // We can't easily test the output of print_config() directly since it prints to stdout
        // Instead, we'll test the key components that would be used in formatting

        // Create a sample config
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

        // We're not asserting exact output here since print_config depends on terminal width
        // Instead, we just verify that the struct can be created without panicking
        assert!(config.open.is_some());

        if let Some(open) = &config.open {
            assert_eq!(open.aliases.as_ref().unwrap().len(), 2);
            assert_eq!(open.things.as_ref().unwrap().len(), 2);
        }
    }

    #[test]
    #[ignore] // Ignored by default as it would actually open the URL
    fn test_open_it_with_mock_config() {
        // Create a temporary directory to hold our mock config file
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join(".dkdc");
        fs::create_dir_all(&config_dir).unwrap();

        let config_file = config_dir.join("config.toml");
        fs::write(
            &config_file,
            r#"
        [open]
        [open.aliases]
        "test" = "testsite"
        
        [open.things]
        "testsite" = "https://example.com"
        "#,
        )
        .unwrap();

        // Set the DKDC_HOME environment variable to point to our temp directory
        unsafe {
            env::set_var("DKDC_HOME", config_dir.to_str().unwrap());
        }

        // Now call open_it - we're just checking it doesn't panic
        let _result = open_it("test");

        // We don't assert success as it depends on whether `open` command works
        // on the current platform, but the code should run without panicking

        // Clean up
        unsafe {
            env::remove_var("DKDC_HOME");
        }
    }
}
