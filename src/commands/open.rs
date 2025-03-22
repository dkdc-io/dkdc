use crate::config::load_config;
use crate::terminal::{bottom_border, format_line, get_terminal_width, print_message, top_border};
use std::process::Command;

pub fn open_it(input: &str) {
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("error loading config: {}", e);
            return;
        }
    };

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

    let thing = if let Some(ref open) = config.open {
        if let Some(things) = &open.things {
            if let Some(mapped) = things.get(&target) {
                mapped.to_owned()
            } else {
                eprintln!("{} not found in config", target);
                return;
            }
        } else {
            eprintln!("no things defined in config");
            return;
        }
    } else {
        eprintln!("no config available");
        return;
    };

    print_message("dkdc", format!("opening: {}...", thing).as_str());

    if let Err(e) = Command::new("open").arg(&thing).spawn() {
        eprintln!("failed to open: {}", e);
    }
}

pub fn print_config() {
    let width = get_terminal_width();
    let title = "dkdc.io";
    let mut lines = Vec::new();
    lines.push(top_border(title, width));

    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };

    lines.push(format_line("aliases:", width));
    lines.push(format_line("", width));
    if let Some(ref open) = config.open {
        if let Some(ref aliases) = open.aliases {
            let max_key_len = aliases.keys().map(|k| k.len()).max().unwrap();
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
        } else {
            lines.push(format_line("No aliases defined", width));
        }

        lines.push(format_line("", width));
        lines.push(format_line("things:", width));
        lines.push(format_line("", width));

        if let Some(ref things) = open.things {
            let max_key_len = things.keys().map(|k| k.len()).max().unwrap();
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
        } else {
            lines.push(format_line("no things defined", width));
        }
    } else {
        lines.push(format_line("no config available", width));
    }

    lines.push(bottom_border(width));

    for line in lines {
        println!("{}", line);
    }
}
