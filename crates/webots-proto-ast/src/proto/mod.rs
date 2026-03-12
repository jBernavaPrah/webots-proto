pub mod ast;
pub(crate) mod lexer;
pub mod parser;
pub mod span;
pub mod writer;

use self::parser::Parser;
use self::writer::ProtoWriter as Writer;
use std::path::Path;
use thiserror::Error;

pub type ProtoResult<T> = Result<T, ProtoError>;

#[derive(Error, Debug)]
pub enum ProtoError {
    #[error("Parsing error: {0}")]
    ParseError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Format error: {0}")]
    FormatError(#[from] std::fmt::Error),
}

impl std::str::FromStr for ast::Proto {
    type Err = ProtoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        parser
            .parse_document()
            .map_err(|error| ProtoError::ParseError(format!("{error:?}")))
    }
}

impl std::fmt::Display for ast::Proto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Writer::new().write_document_canonical(f, self)
    }
}

impl ast::Proto {
    pub fn from_file(path: impl AsRef<Path>) -> ProtoResult<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let mut document: ast::Proto = content.parse()?;
        document.source_path = Some(path.to_path_buf());
        Ok(document)
    }

    pub fn to_lossless_string(&self) -> ProtoResult<String> {
        Writer::new().write_lossless(self)
    }

    pub fn to_canonical_string(&self) -> ProtoResult<String> {
        Ok(Writer::new().write_canonical(self))
    }
}
