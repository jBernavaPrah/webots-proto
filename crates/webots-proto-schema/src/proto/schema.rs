//! Webots node schema definitions.

use crate::proto::ast::FieldType;

/// Trait implemented by Webots nodes to provide schema information.
pub trait WebotsNode {
    /// Returns the name of the node.
    fn node_name() -> &'static str;

    /// Returns all fields of the node.
    fn all_fields() -> &'static [(&'static str, FieldType)];
}

/// Trait implemented by field types that map to a Webots `FieldType`.
///
/// Missing implementations surface as compile-time errors when defining node schemas.
pub trait WebotsFieldType {
    const FIELD_TYPE: FieldType;
}
