use super::span::Span;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a complete PROTO document.
#[derive(Debug, Clone, PartialEq, Default, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct Proto {
    /// The VRML/PROTO header.
    pub header: Option<Header>,
    /// EXTERNPROTO declarations.
    pub externprotos: Vec<ExternProto>,
    /// The PROTO definition itself.
    pub proto: Option<ProtoDefinition>,
    /// Root nodes (if this is a .wbt or a PROTO without a wrapper).
    pub root_nodes: Vec<AstNode>,
    /// Absolute source file path when loaded from disk.
    #[serde(skip)]
    pub source_path: Option<PathBuf>,
    /// Original source content when loaded from disk.
    #[serde(skip)]
    pub source_content: Option<String>,
}

impl Proto {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_header(header: Header) -> Self {
        Self {
            header: Some(header),
            ..Self::default()
        }
    }
}

/// The file header, e.g., `#VRML_SIM R2025a utf8`.
#[derive(Debug, Clone, PartialEq, Default, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct Header {
    pub version: String,
    pub encoding: String,
    pub raw: Option<String>,
    pub span: Span,
}

/// An EXTERNPROTO declaration.
#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct ExternProto {
    pub url: String,
    pub alias: Option<String>,
    pub span: Span,
}

/// The main PROTO definition block.
#[derive(Debug, Clone, PartialEq, Default, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct ProtoDefinition {
    pub name: String,
    #[new(default)]
    pub fields: Vec<ProtoField>,
    #[new(default)]
    pub body: Vec<ProtoBodyItem>,
    pub span: Span,
}

/// A field declaration in the PROTO interface.
#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct ProtoField {
    pub name: String,
    pub field_type: FieldType,
    #[new(default)]
    pub default_value: Option<FieldValue>,
    /// e.g. "field", "vrmlField", "hiddenField", "deprecatedField"
    pub keyword: FieldKeyword,
    /// Optional list of allowed values for this field (e.g. `field SFString { "a", "b" } name "a"`)
    #[new(default)]
    pub restrictions: Option<Vec<FieldValue>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldKeyword {
    Field,
    VrmlField,
    HiddenField,
    DeprecatedField,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    SFBool,
    SFInt32,
    SFFloat,
    SFString,
    SFVec2f,
    SFVec3f,
    SFRotation,
    SFColor,
    SFNode,
    MFBool,
    MFInt32,
    MFFloat,
    MFString,
    MFVec2f,
    MFVec3f,
    MFRotation,
    MFColor,
    MFNode,
    Unknown(String),
}

/// An item inside the PROTO body: either a Node or a Template Block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProtoBodyItem {
    Node(AstNode),
    Template(TemplateBlock),
}

/// A template block `%< ... >%` or expression `%<= ... >%`.
#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct TemplateBlock {
    pub content: String,
    /// If true, it is an expression `%<= ... >%`.
    pub is_expression: bool,
    pub span: Span,
}

/// A node in the AST (e.g., `Robot { ... }` or `USE name`).
#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct AstNode {
    pub kind: AstNodeKind,
    pub span: Span,
}

impl Default for AstNode {
    fn default() -> Self {
        Self {
            kind: AstNodeKind::Node {
                type_name: "Group".to_string(),
                def_name: None,
                fields: vec![],
            },
            span: Span::default(),
        }
    }
}

impl From<AstNode> for FieldValue {
    fn from(node: AstNode) -> Self {
        FieldValue::Node(Box::new(node))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNodeKind {
    /// A standard node definition: `[DEF name] Type { fields }`
    Node {
        type_name: String,
        def_name: Option<String>,
        fields: Vec<NodeBodyElement>,
    },
    /// A USE node: `USE name`
    Use { use_name: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeBodyElement {
    Field(NodeField),
    Template(TemplateBlock),
    /// Represents an unexpected token inside the body that we keep to be lossless.
    /// This happens when template blocks mess up the structure (e.g., unbalanced braces).
    Raw(RawSyntax),
}

#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct RawSyntax {
    pub text: String,
    pub span: Span,
}

/// A field assignment inside a node (e.g., `translation 0 1 0` or `children IS bodySlot`).
#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct NodeField {
    pub name: String,
    pub value: FieldValue,
    pub span: Span,
}

/// The value of a field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldValue {
    Bool(bool),
    Int(i64, Option<String>),
    Float(f64, Option<String>),
    String(String),
    Vec2f([f64; 2]),
    Vec3f([f64; 3]),
    Rotation([f64; 4]),
    Color([f64; 3]),
    Node(Box<AstNode>),
    Array(ArrayValue),
    /// A sequence of numbers without brackets (e.g. `0 0 1`).
    NumberSequence(NumberSequence),
    /// `IS fieldName`
    Is(String),
    /// `NULL` literal
    Null,
    /// Template expression as a value `%<= ... >%`
    Template(TemplateBlock),
    /// For unknown or complex types not fully parsed yet
    Raw(String),
}

/// A bracketed array value.
#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct ArrayValue {
    #[new(default)]
    pub elements: Vec<ArrayElement>,
}

/// An array element.
#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct ArrayElement {
    pub value: FieldValue,
}

/// A sequence of numeric values.
#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct NumberSequence {
    #[new(default)]
    pub elements: Vec<NumberSequenceElement>,
}

/// A number sequence element.
#[derive(Debug, Clone, PartialEq, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct NumberSequenceElement {
    pub value: FieldValue,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::span::Span;

    #[test]
    fn test_manual_ast_construction() {
        let span = Span::default();

        let header = Header::new("R2025a".to_string(), "utf8".to_string(), None, span.clone());

        let extern_proto =
            ExternProto::new("PedestrianTorso.proto".to_string(), None, span.clone());

        let translation_field = ProtoField::new(
            "translation".to_string(),
            FieldType::SFVec3f,
            FieldKeyword::Field,
            span.clone(),
        )
        .with_default_value(FieldValue::Vec3f([0.0, 0.0, 1.27]));

        let rotation_field = ProtoField::new(
            "rotation".to_string(),
            FieldType::SFRotation,
            FieldKeyword::Field,
            span.clone(),
        )
        .with_default_value(FieldValue::Rotation([0.0, 0.0, 1.0, 0.0]));

        let template_statement = ProtoBodyItem::Template(TemplateBlock::new(
            " const rigid = fields.controllerArgs.value.length == 0; ".to_string(),
            false,
            span.clone(),
        ));

        let robot_node = AstNode::new(
            AstNodeKind::Node {
                type_name: "Robot".to_string(),
                def_name: None,
                fields: vec![],
            },
            span.clone(),
        );

        let proto_def = ProtoDefinition::new("Pedestrian".to_string(), span.clone())
            .with_fields(vec![translation_field, rotation_field])
            .with_body(vec![template_statement, ProtoBodyItem::Node(robot_node)]);

        let document = Proto::new()
            .with_header(header)
            .with_externprotos(vec![extern_proto])
            .with_proto(proto_def);

        assert_eq!(document.header.as_ref().unwrap().version, "R2025a");
        assert_eq!(document.externprotos.len(), 1);
        assert_eq!(document.externprotos[0].url, "PedestrianTorso.proto");

        let proto = document.proto.as_ref().unwrap();
        assert_eq!(proto.name, "Pedestrian");
        assert_eq!(proto.fields.len(), 2);
        assert_eq!(proto.fields[0].name, "translation");
        if let Some(FieldValue::Vec3f(val)) = &proto.fields[0].default_value {
            assert_eq!(*val, [0.0, 0.0, 1.27]);
        } else {
            panic!("Expected Vec3f default value");
        }

        assert_eq!(proto.body.len(), 2);
        if let ProtoBodyItem::Template(block) = &proto.body[0] {
            assert_eq!(
                block.content,
                " const rigid = fields.controllerArgs.value.length == 0; "
            );
            assert!(!block.is_expression);
        } else {
            panic!("Expected TemplateBlock");
        }
    }
}
