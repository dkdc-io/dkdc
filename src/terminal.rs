use std::env;

/// Get the width of the terminal window
pub fn get_terminal_width() -> usize {
    // Try to get the width from environment variables
    if let Ok(cols) = env::var("COLUMNS") {
        if let Ok(width) = cols.parse::<usize>() {
            return width;
        }
    }

    // Fall back to a reasonable default
    120
}

/// Format a line of text with borders for display
pub fn format_line(text: &str, width: usize) -> String {
    let inner_width = width - 2;
    let text_len = text.chars().count();
    let padding = inner_width.saturating_sub(text_len);
    format!("│{}{}│", text, " ".repeat(padding))
}

/// Create a top border with a title
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

/// Create a bottom border
pub fn bottom_border(width: usize) -> String {
    format!("╰{}╯", "─".repeat(width - 2))
}

/// Print a formatted message with a title and borders
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_line() {
        // Test with text shorter than width
        let result = format_line("Hello", 10);
        assert_eq!(result, "│Hello   │");

        // Test with text equal to inner width
        let result = format_line("12345678", 10);
        assert_eq!(result, "│12345678│");

        // Test with text longer than inner width (should still format correctly)
        let result = format_line("1234567890", 10);
        assert_eq!(result, "│1234567890│");
    }

    #[test]
    fn test_top_border() {
        // Test with small width
        let result = top_border("test", 12);
        assert_eq!(result, "╭── test ──╮");

        // Test with large width
        let result = top_border("title", 20);
        assert_eq!(result, "╭───── title ──────╮");

        // Test with empty title
        let result = top_border("", 10);
        assert_eq!(result, "╭───  ───╮");
    }

    #[test]
    fn test_bottom_border() {
        // Test with various widths
        let result = bottom_border(10);
        assert_eq!(result, "╰────────╯");

        let result = bottom_border(5);
        assert_eq!(result, "╰───╯");
    }

    #[test]
    fn test_get_terminal_width() {
        // Test default case (should return default or env var if set)
        let width = get_terminal_width();
        // We can't easily assert the exact value since it depends on env or default
        assert!(width > 0);

        // Test with specific env var set
        unsafe {
            env::set_var("COLUMNS", "80");
        }
        assert_eq!(get_terminal_width(), 80);

        // Test with invalid env var value
        unsafe {
            env::set_var("COLUMNS", "invalid");
        }
        // Should fall back to default
        assert_eq!(get_terminal_width(), 120);

        // Clean up
        unsafe {
            env::remove_var("COLUMNS");
        }
    }
}
