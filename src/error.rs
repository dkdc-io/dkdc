use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Config(String),
    Io(io::Error),
    Toml(toml::de::Error),
    Command(String),
    Missing(String),
    Ffmpeg(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Config(msg) => write!(f, "Config error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Toml(err) => write!(f, "TOML parsing error: {}", err),
            Error::Command(msg) => write!(f, "Command error: {}", msg),
            Error::Missing(msg) => write!(f, "Missing: {}", msg),
            Error::Ffmpeg(msg) => write!(f, "FFmpeg error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Toml(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_error_display() {
        let config_err = Error::Config("test config error".to_string());
        assert_eq!(format!("{}", config_err), "Config error: test config error");

        let io_err = Error::Io(IoError::new(ErrorKind::NotFound, "test io error"));
        assert!(format!("{}", io_err).starts_with("IO error:"));

        let command_err = Error::Command("test command error".to_string());
        assert_eq!(format!("{}", command_err), "Command error: test command error");

        let missing_err = Error::Missing("test missing error".to_string());
        assert_eq!(format!("{}", missing_err), "Missing: test missing error");

        let ffmpeg_err = Error::Ffmpeg("test ffmpeg error".to_string());
        assert_eq!(format!("{}", ffmpeg_err), "FFmpeg error: test ffmpeg error");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = IoError::new(ErrorKind::NotFound, "test io error");
        let err: Error = io_err.into();
        
        match err {
            Error::Io(_) => (),
            _ => panic!("Expected Error::Io variant"),
        }
    }

    #[test]
    fn test_error_source() {
        let io_err = Error::Io(IoError::new(ErrorKind::NotFound, "test io error"));
        assert!(io_err.source().is_some());

        let config_err = Error::Config("test config error".to_string());
        assert!(config_err.source().is_none());
    }
}