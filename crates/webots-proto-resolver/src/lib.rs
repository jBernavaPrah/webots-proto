mod resolve;

use thiserror::Error;

pub use resolve::{ProtoResolver, ResolveOptions};

pub type ProtoResult<T> = Result<T, ProtoError>;

#[derive(Error, Debug)]
pub enum ProtoError {
    #[error("Parsing error: {0}")]
    ParseError(String),
    #[error("Template error: {0}")]
    TemplateError(String),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<webots_proto_template::TemplateError> for ProtoError {
    fn from(error: webots_proto_template::TemplateError) -> Self {
        Self::TemplateError(error.to_string())
    }
}
