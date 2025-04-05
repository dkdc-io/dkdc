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
                            let formatted = format!(" • {:<width$} | {}", alias, target, width = max_key_len);
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
                            let formatted = format!(" • {:<width$} | {}", thing, target, width = max_key_len);
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
