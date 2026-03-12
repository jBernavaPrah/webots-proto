pub mod error;
#[macro_use]
pub mod macros;
pub mod types;
mod versions;

mod proto {
    pub mod ast {
        pub use webots_proto_ast::proto::ast::*;
    }

    pub mod span {
        pub use webots_proto_ast::proto::span::*;
    }

    pub type ProtoResult<T> = Result<T, ProtoError>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ProtoError {
        ParseError(String),
        SerializationError(String),
    }

    pub mod builtin_nodes;
    pub(crate) mod conversion;
    pub mod schema;
    pub mod validation;
}

pub use error::{Error, Result};
pub use proto::validation::{
    Diagnostic, DiagnosticSet, NodeSchema, SchemaField, Severity, ValidationContext,
};
pub use versions::r2025a;

pub fn validate(proto: &proto::ast::Proto) -> DiagnosticSet {
    proto::validation::validate_document(proto)
}

pub fn validate_runtime_semantics(node: &proto::ast::AstNode) -> DiagnosticSet {
    proto::validation::validate_runtime_semantics(node)
}

pub fn ast_to_r2025a_node(node: &proto::ast::AstNode) -> proto::ProtoResult<r2025a::Node> {
    proto::conversion::ast_to_r2025a_node(node)
}

pub fn r2025a_node_to_ast(node: &r2025a::Node) -> proto::ProtoResult<proto::ast::AstNode> {
    proto::conversion::r2025a_node_to_ast(node)
}
