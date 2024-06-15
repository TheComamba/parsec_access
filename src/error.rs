use std::fmt;

#[derive(Debug)]
pub enum ParsecAccessError {
    Connection(reqwest::Error),
    DataNotAvailable(String),
    Io(std::io::Error),
    RmpSerialization(rmp_serde::encode::Error),
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
