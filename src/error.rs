//! Error handling for the Parsec access library.

use std::fmt;

/// Represents an error that can occur when accessing the Parsec data.
#[derive(Debug)]
pub enum ParsecAccessError {
    /// An error occurred while trying to establish a connection to the Parsec server.
    Connection(reqwest::Error),
    /// The requested data is not available.
    DataNotAvailable(String),
    /// An error occurred while trying to parse a glob pattern.
    Glob(glob::PatternError),
    /// An I/O error occurred.
    Io(std::io::Error),
}

impl fmt::Display for ParsecAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsecAccessError::Connection(err) => write!(f, "Connection error: {}", err),
            ParsecAccessError::DataNotAvailable(data) => write!(f, "Data {} not available", data),
            ParsecAccessError::Glob(err) => write!(f, "Glob error: {}", err),
            ParsecAccessError::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}
