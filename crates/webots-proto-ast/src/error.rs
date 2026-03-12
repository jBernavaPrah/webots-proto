use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Syntax error at line {line}, column {col}: {message}")]
    Syntax {
        line: usize,
        col: usize,
        message: String,
    },

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Expected {expected}, found {found}")]
    UnexpectedToken { expected: String, found: String },

    #[error("Unknown version: {0}")]
    UnknownVersion(String),
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Serialization(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Deserialization(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
