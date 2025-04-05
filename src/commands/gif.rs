use crate::error::{Error, Result};
use crate::terminal::print_message;
use std::path::Path;
use std::process::Command;

pub fn gif_it(input: String, output: Option<String>) -> Result<()> {
    print_message("dkdc", &format!(" converting {} to gif...", input));

    let input_path = Path::new(&input);
    if !input_path.exists() {
        return Err(Error::Missing(format!("Input file {} not found", input)));
    }

    let output_path = match output {
        Some(output) => output,
        None => extension_to_gif(&input),
    };

    print_message("dkdc", &format!(" output: {}", output_path));

    // Execute ffmpeg to convert the file
    execute_ffmpeg(&input, &output_path)
}

fn extension_to_gif(input: &str) -> String {
    let input_path = Path::new(input);
    
    match input_path.extension() {
        Some(ext) if ext != "gif" => {
            // If there's an extension and it's not gif, replace it
            let stem = input_path.file_stem().unwrap_or_default();
            format!("{}.gif", stem.to_string_lossy())
        }
        Some(_) => {
            // If it's already a gif, return as is
            input.to_string()
        }
        None => {
            // No extension, just add .gif
            format!("{}.gif", input)
        }
    }
}

fn execute_ffmpeg(input: &str, output: &str) -> Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-filter_complex")
        .arg("[0:v] fps=10,scale=640:-1:flags=lanczos,palettegen [p]; [0:v] fps=10,scale=640:-1:flags=lanczos [x]; [x][p] paletteuse")
        .arg("-y") // Overwrite output if it exists
        .arg(output)
        .status()
        .map_err(|e| Error::Command(format!("Failed to execute ffmpeg: {}", e)))?;

    if !status.success() {
        return Err(Error::Ffmpeg(format!("ffmpeg exited with status: {}", status)));
    }

    print_message("dkdc", " conversion successful!");
    Ok(())
}
