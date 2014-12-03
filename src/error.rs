use std::{io, error};
use std::error::FromError;

use self::ErrorKind::{
    DecodingError,
    IoError,
};

/// Application generic result type.
pub type FerrumResult<T> = Result<T, FerrumError>;

/// An enum of all error kinds.
#[deriving(PartialEq, Eq, Clone, Show)]
pub enum ErrorKind {
    /// Failed to decode a file.
    DecodingError(String),
    /// An IO error was encountered.
    IoError(io::IoError),
    /// The configuration file is improperly formatted.
    InvalidConfigError,
}

/// Represents a Ferrum error. For the most part you should be using
/// the Error trait to interact with this rather than the actual
/// struct.
#[deriving(PartialEq, Eq, Clone, Show)]
pub struct FerrumError {
    pub kind: ErrorKind,
    pub desc: &'static str,
    pub detail: Option<String>,
}

impl error::Error for FerrumError {
    fn description(&self) -> &str {
        match self.kind {
            DecodingError(_) => "Error decoding file",
            IoError(_) => "Encountered an I/O error",
            ErrorKind::InvalidConfigError => "Invalid configuration file"
        }
    }

    fn detail(&self) -> Option<String> {
        match self.kind {
            DecodingError(ref filename) => Some(format!("Filename was {}", filename)),
            _ => None,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.kind {
            IoError(ref err) => Some(err as &error::Error),
            _ => None,
        }
    }
}

impl FromError<(ErrorKind, &'static str)> for FerrumError {
    fn from_error((kind, desc): (ErrorKind, &'static str)) -> FerrumError {
        FerrumError { kind: kind, desc: desc, detail: None }
    }
}

impl FromError<io::IoError> for FerrumError {
    fn from_error(err: io::IoError) -> FerrumError {
        FerrumError {
            kind: IoError(err),
            desc: "An internal IO error occured.",
            detail: None
        }
    }
}
