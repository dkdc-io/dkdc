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