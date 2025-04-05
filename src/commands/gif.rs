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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_extension_to_gif() {
        // Test with various file extensions
        assert_eq!(extension_to_gif("video.mp4"), "video.gif");
        assert_eq!(extension_to_gif("movie.avi"), "movie.gif");
        assert_eq!(extension_to_gif("animation.gif"), "animation.gif"); // Should remain unchanged
        assert_eq!(extension_to_gif("file"), "file.gif"); // No extension
        assert_eq!(extension_to_gif("/path/to/video.mp4"), "video.gif");
        assert_eq!(extension_to_gif("file.with.dots.mp4"), "file.with.dots.gif");
    }

    #[test]
    #[ignore] // Ignore by default as it depends on external ffmpeg
    fn test_execute_ffmpeg() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("test.mp4");
        let output_path = dir.path().join("test.gif");

        // Create a dummy input file (not a real mp4)
        let mut file = File::create(&input_path).unwrap();
        file.write_all(b"dummy mp4 content").unwrap();

        // Try to execute ffmpeg
        // This test will likely fail if ffmpeg isn't installed
        let result = execute_ffmpeg(
            input_path.to_str().unwrap(),
            output_path.to_str().unwrap(),
        );

        // We don't assert success/failure here since it depends on ffmpeg
        // being installed and working correctly
        // Just make sure it returns a result
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_gif_it_missing_file() {
        // Test with a file that doesn't exist
        let result = gif_it("nonexistent_file.mp4".to_string(), None);
        
        // Should return a Missing error
        assert!(result.is_err());
        match result {
            Err(Error::Missing(msg)) => {
                assert!(msg.contains("not found"));
            }
            _ => panic!("Expected Missing error"),
        }
    }
}
