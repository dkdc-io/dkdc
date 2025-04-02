//use crate::terminal::{bottom_border, format_line, get_terminal_width, print_message, top_border};
use std::io::{self};
use std::process::Command;

pub fn gif_it(input: String, output: Option<String>) {
    println!("Converting to gif...");
    println!("Input: {}", input);
    let output_path = match output {
        Some(output) => output,
        None => extension_to_gif(&input),
    };
    println!("Output: {}", output_path);

    // Execute ffmpeg to convert the file
    match execute_ffmpeg(&input, &output_path) {
        Ok(_) => println!("Conversion successful!"),
        Err(e) => eprintln!("Error converting to GIF: {}", e),
    }
}

fn extension_to_gif(input: &str) -> String {
    let mut output = String::from(input);
    if input.contains(".") {
        let mut parts: Vec<&str> = input.split(".").collect();
        let extension = parts.pop().unwrap();
        if extension != "gif" {
            output = format!("{}.gif", parts.join("."));
        }
    } else {
        output = format!("{}.gif", input);
    }
    output
}

fn execute_ffmpeg(input: &str, output: &str) -> Result<(), io::Error> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-filter_complex")
        .arg("[0:v] fps=10,scale=640:-1:flags=lanczos,palettegen [p]; [0:v] fps=10,scale=640:-1:flags=lanczos [x]; [x][p] paletteuse")
        .arg(output)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("ffmpeg exited with status: {}", status),
        ));
    }

    Ok(())
}
