//! Error handling for the Parsec access library.

use std::fmt;

/// Represents an error that can occur when accessing the Parsec data.
#[derive(Debug)]
pub enum ParsecAccessError {
    /// An error occurred while trying to establish a connection to the Parsec server.
    Connection(reqwest::Error),
    /// The requested data is not available.
    DataNotAvailable(String),
    /// An I/O error occurred.
    Io(std::io::Error),
    /// An error occurred during MessagePack serialization.
    RmpSerialization(rmp_serde::encode::Error),
    /// An error occurred during MessagePack deserialization.
    RmpDeserialization(rmp_serde::decode::Error),
}

impl fmt::Display for ParsecAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsecAccessError::Connection(err) => write!(f, "Connection error: {}", err),
            ParsecAccessError::DataNotAvailable(data) => write!(f, "Data {} not available", data),
            ParsecAccessError::Io(err) => write!(f, "I/O error: {}", err),
            ParsecAccessError::RmpSerialization(err) => {
                write!(f, "MessagePack serialization error: {}", err)
            }
            ParsecAccessError::RmpDeserialization(err) => {
                write!(f, "MessagePack deserialization error: {}", err)
            }
        }
    }
}
