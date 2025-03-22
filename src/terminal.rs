use std::env;

pub fn get_terminal_width() -> usize {
    if let Ok(cols) = env::var("COLUMNS") {
        if let Ok(width) = cols.parse::<usize>() {
            return width;
        }
    }
    120
}

pub fn format_line(text: &str, width: usize) -> String {
    let inner_width = width - 2;
    let text_len = text.chars().count();
    let padding = inner_width.saturating_sub(text_len);
    format!("│{}{}│", text, " ".repeat(padding))
}

pub fn top_border(title: &str, width: usize) -> String {
    let title = format!(" {} ", title);
    let title_len = title.chars().count();
    let inner_width = width - 2;
    let available = inner_width.saturating_sub(title_len);
    let pad_left = available / 2;
    let pad_right = available - pad_left;
    format!(
        "╭{}{}{}╮",
        "─".repeat(pad_left),
        title,
        "─".repeat(pad_right)
    )
}

pub fn bottom_border(width: usize) -> String {
    format!("╰{}╯", "─".repeat(width - 2))
}

pub fn print_message(title: &str, message: &str) {
    let width = get_terminal_width();
    let top = top_border(title, width);
    let bottom = bottom_border(width);
    println!("{}", top);
    for line in message.lines() {
        println!("{}", format_line(line, width));
    }
    println!("{}", bottom);
}
