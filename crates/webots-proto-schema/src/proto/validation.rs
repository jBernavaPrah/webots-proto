//! PROTO document validation and diagnostics.
//!
//! This module provides validation for Webots PROTO documents, including:
//! - Header validation (version and encoding)
//! - PROTO interface validation (field types and default value conformance)
//! - Node validation against Webots schema
//! - IS bindings validation
//! - DEF/USE validation
//! - Array arity validation
//! - Nullability validation

use crate::proto::ast::*;
use crate::proto::builtin_nodes::get_builtin_schema;
use crate::proto::span::Span;
use std::collections::HashMap;

/// A Webots node schema field definition.
#[derive(Debug, Clone)]
pub struct SchemaField {
    pub name: String,
    pub field_type: FieldType,
}

/// A Webots node schema.
#[derive(Debug, Clone)]
pub struct NodeSchema {
    pub name: &'static str,
    pub fields: Vec<SchemaField>,
}

impl NodeSchema {
    pub fn get_field_type(&self, name: &str) -> Option<FieldType> {
        self.fields
            .iter()
            .find(|field| field.name == name)
            .map(|field| field.field_type.clone())
    }
}

/// A diagnostic message with location information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    /// The span where the diagnostic applies.
    pub span: Span,
    /// The severity of the diagnostic.
    pub severity: Severity,
    /// The diagnostic message.
    pub message: String,
    /// An optional suggestion for fixing the issue.
    pub suggestion: Option<String>,
}

/// The severity of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    /// An error that prevents correct processing.
    Error,
    /// A warning that doesn't prevent processing but may indicate a problem.
    Warning,
}

/// A collection of diagnostics.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticSet {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticSet {
    /// Creates a new empty diagnostic set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a diagnostic to the set.
    ///
    /// # Arguments
    ///
    /// * `diagnostic` - The diagnostic to add.
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Adds multiple diagnostics to the set.
    pub fn extend(&mut self, diagnostics: impl IntoIterator<Item = Diagnostic>) {
        self.diagnostics.extend(diagnostics);
    }

    /// Returns an iterator over the diagnostics.
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }

    /// Returns true if there are any errors in the set.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error))
    }

    /// Returns the number of diagnostics.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns true if there are no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

pub(crate) fn validate_document(document: &Proto) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();
    let mut context = ValidationContext::new();

    diagnostics.extend(validate_header(document));

    if let Some(proto_def) = &document.proto {
        context.interface_fields = proto_def.fields.clone();
        diagnostics.extend(validate_interface(proto_def));
        diagnostics.extend(validate_node_body(&proto_def.body, &mut context));
    }

    for node in &document.root_nodes {
        diagnostics.extend(validate_ast_node(node, &mut context));
    }

    diagnostics
}

/// Validates runtime-only semantic rules on an AST node tree.
///
/// This intentionally skips schema typing and IS/DEF/USE checks.
pub(crate) fn validate_runtime_semantics(node: &AstNode) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();
    validate_runtime_semantics_recursive(node, &mut diagnostics);
    diagnostics
}

/// Validation context to track DEF/USE and interface fields.
#[derive(Debug, Clone, Default)]
pub struct ValidationContext {
    pub defs: HashMap<String, String>, // name -> type_name
    pub interface_fields: Vec<ProtoField>,
}

impl ValidationContext {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Validates a PROTO document header.
fn validate_header(document: &Proto) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();

    if let Some(header) = &document.header {
        // Validate version
        if header.version != "R2025a" {
            diagnostics.add(Diagnostic {
                span: header.span.clone(),
                severity: Severity::Error,
                message: format!("Unsupported version: {}", header.version),
                suggestion: Some("Use R2025a".to_string()),
            });
        }

        // Validate encoding
        if header.encoding != "utf8" {
            diagnostics.add(Diagnostic {
                span: header.span.clone(),
                severity: Severity::Error,
                message: format!("Unsupported encoding: {}", header.encoding),
                suggestion: Some("Use utf8".to_string()),
            });
        }
    } else if document.proto.is_some() || !document.root_nodes.is_empty() {
        diagnostics.add(Diagnostic {
            span: Span::default(),
            severity: Severity::Error,
            message: "Missing PROTO header".to_string(),
            suggestion: Some("Add #VRML_SIM R2025a utf8 at the beginning of the file".to_string()),
        });
    }

    diagnostics
}

/// Validates a PROTO interface.
fn validate_interface(proto_def: &ProtoDefinition) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();

    for field in &proto_def.fields {
        if let Some(default_value) = &field.default_value {
            diagnostics.extend(validate_field_value(
                default_value,
                &field.field_type,
                &field.span,
            ));

            if let Some(restrictions) = &field.restrictions {
                // Check if default value satisfies restrictions
                if !restrictions
                    .iter()
                    .any(|value| values_are_equal(value, default_value))
                {
                    diagnostics.add(Diagnostic {
                        span: field.span.clone(),
                        severity: Severity::Error,
                        message: format!(
                            "Default value {:?} is not in the allowed restrictions",
                            default_value
                        ),
                        suggestion: None,
                    });
                }
            }
        }

        if let Some(restrictions) = &field.restrictions {
            for res in restrictions {
                diagnostics.extend(validate_field_value(res, &field.field_type, &field.span));
            }
        }
    }

    diagnostics
}

fn values_are_equal(a: &FieldValue, b: &FieldValue) -> bool {
    if let (FieldValue::Int(ia, _), FieldValue::Float(fb, _)) = (a, b) {
        *ia as f64 == *fb
    } else if let (FieldValue::Float(fa, _), FieldValue::Int(ib, _)) = (a, b) {
        *fa == *ib as f64
    } else {
        a == b
    }
}

/// Validates a PROTO node body.
fn validate_node_body(items: &[ProtoBodyItem], context: &mut ValidationContext) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();

    for item in items {
        match item {
            ProtoBodyItem::Node(node) => {
                diagnostics.extend(validate_ast_node(node, context));
            }
            ProtoBodyItem::Template(_) => {
                // Template blocks are skipped in static validation for now
            }
        }
    }

    diagnostics
}

/// Validates an AST node.
fn validate_ast_node(node: &AstNode, context: &mut ValidationContext) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();

    match &node.kind {
        AstNodeKind::Node {
            type_name,
            def_name,
            fields,
        } => {
            if let Some(name) = def_name {
                context.defs.insert(name.clone(), type_name.clone());
            }

            diagnostics.extend(validate_runtime_semantics_for_node(node));

            if let Some(schema) = get_builtin_schema(type_name) {
                for element in fields {
                    if let NodeBodyElement::Field(field) = element {
                        // 1. Schema Validation (Type Check)
                        if let Some(expected_type) = schema.get_field_type(&field.name) {
                            diagnostics.extend(validate_field_value(
                                &field.value,
                                &expected_type,
                                &field.span,
                            ));
                            diagnostics.extend(validate_node_field_semantics(
                                type_name,
                                &field.name,
                                &field.value,
                                &field.span,
                            ));

                            // Check IS binding
                            if let FieldValue::Is(ref_name) = &field.value {
                                if let Some(interface_field) = context
                                    .interface_fields
                                    .iter()
                                    .find(|f| f.name == *ref_name)
                                {
                                    if !types_are_compatible(
                                        &interface_field.field_type,
                                        &expected_type,
                                    ) {
                                        diagnostics.add(Diagnostic {
                                            span: field.span.clone(),
                                            severity: Severity::Error,
                                            message: format!(
                                                "Type mismatch in IS binding: interface field '{}' is {}, but node field '{}' expects {}",
                                                ref_name,
                                                field_type_name(&interface_field.field_type),
                                                field.name,
                                                field_type_name(&expected_type)
                                            ),
                                            suggestion: None,
                                        });
                                    }
                                } else {
                                    diagnostics.add(Diagnostic {
                                        span: field.span.clone(),
                                        severity: Severity::Error,
                                        message: format!("Undefined IS reference: {}", ref_name),
                                        suggestion: None,
                                    });
                                }
                            }
                        } else {
                            let suggestion = find_closest_match(
                                &field.name,
                                schema.fields.iter().map(|field| field.name.as_str()),
                            );
                            diagnostics.add(Diagnostic {
                                span: field.span.clone(),
                                severity: Severity::Warning,
                                message: format!(
                                    "Unknown field '{}' for node '{}'",
                                    field.name, type_name
                                ),
                                suggestion: suggestion.map(|s| format!("Did you mean '{}'?", s)),
                            });
                        }

                        // 2. Content Recursion (DEF/USE Registration & Child Validation)
                        // Verify content regardless of whether the field is known in the schema
                        if let FieldValue::Node(child_node) = &field.value {
                            diagnostics.extend(validate_ast_node(child_node, context));
                        } else if let FieldValue::Array(arr) = &field.value {
                            for el in &arr.elements {
                                if let FieldValue::Node(child_node) = &el.value {
                                    diagnostics.extend(validate_ast_node(child_node, context));
                                }
                            }
                        }
                    }
                }
            } else {
                diagnostics.add(Diagnostic {
                    span: node.span.clone(),
                    severity: Severity::Warning,
                    message: format!("Unknown node type '{}'", type_name),
                    suggestion: None,
                });

                // Still validate fields even if node type is unknown (to find DEF/USE inside)
                for element in fields {
                    if let NodeBodyElement::Field(field) = element {
                        if let FieldValue::Node(child_node) = &field.value {
                            diagnostics.extend(validate_ast_node(child_node, context));
                        } else if let FieldValue::Array(arr) = &field.value {
                            for el in &arr.elements {
                                if let FieldValue::Node(child_node) = &el.value {
                                    diagnostics.extend(validate_ast_node(child_node, context));
                                }
                            }
                        }
                    }
                }
            }
        }
        AstNodeKind::Use { use_name } => {
            if !context.defs.contains_key(use_name) {
                diagnostics.add(Diagnostic {
                    span: node.span.clone(),
                    severity: Severity::Error,
                    message: format!("Undefined USE reference: {}", use_name),
                    suggestion: None,
                });
            }
        }
    }

    diagnostics
}

fn validate_runtime_semantics_for_node(node: &AstNode) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();

    let AstNodeKind::Node {
        type_name, fields, ..
    } = &node.kind
    else {
        return diagnostics;
    };

    if matches!(type_name.as_str(), "Solid" | "Robot") {
        if solid_requires_inertia_warning(fields) {
            diagnostics.add(Diagnostic {
                span: node.span.clone(),
                severity: Severity::Warning,
                message:
                    "Undefined inertia matrix: using the identity matrix. Please specify 'boundingObject' or 'inertiaMatrix' values."
                        .to_string(),
                suggestion: Some(
                    "Add a non-NULL `boundingObject` on this Solid or set `Physics.inertiaMatrix`."
                        .to_string(),
                ),
            });
        }

        diagnostics.extend(validate_sibling_solid_name_uniqueness(fields));
    }

    diagnostics
}

fn validate_runtime_semantics_recursive(node: &AstNode, diagnostics: &mut DiagnosticSet) {
    diagnostics.extend(validate_runtime_semantics_for_node(node));

    let AstNodeKind::Node { fields, .. } = &node.kind else {
        return;
    };

    for element in fields {
        let NodeBodyElement::Field(field) = element else {
            continue;
        };
        collect_runtime_semantics_from_value(&field.value, diagnostics);
    }
}

fn collect_runtime_semantics_from_value(value: &FieldValue, diagnostics: &mut DiagnosticSet) {
    match value {
        FieldValue::Node(node) => validate_runtime_semantics_recursive(node, diagnostics),
        FieldValue::Array(array) => {
            for element in &array.elements {
                collect_runtime_semantics_from_value(&element.value, diagnostics);
            }
        }
        _ => {}
    }
}

fn solid_requires_inertia_warning(fields: &[NodeBodyElement]) -> bool {
    let Some(physics_field) = get_node_field(fields, "physics") else {
        return false;
    };

    // Webots warns when a physics-enabled Solid lacks both boundingObject and inertiaMatrix.
    let physics_node = match &physics_field.value {
        FieldValue::Node(node) => node,
        FieldValue::Null => return false,
        FieldValue::Template(_) | FieldValue::Is(_) => return false,
        _ => return false,
    };

    let has_bounding_object = matches!(
        get_node_field(fields, "boundingObject").map(|f| &f.value),
        Some(FieldValue::Node(_)) | Some(FieldValue::Template(_)) | Some(FieldValue::Is(_))
    );

    if has_bounding_object {
        return false;
    }

    !physics_has_explicit_inertia_matrix(physics_node)
}

fn physics_has_explicit_inertia_matrix(physics_node: &AstNode) -> bool {
    let AstNodeKind::Node { fields, .. } = &physics_node.kind else {
        return false;
    };

    let Some(inertia_matrix) = get_node_field(fields, "inertiaMatrix") else {
        return false;
    };

    match &inertia_matrix.value {
        FieldValue::Null => false,
        FieldValue::Array(items) => !items.elements.is_empty(),
        FieldValue::Template(_) | FieldValue::Is(_) => true,
        _ => true,
    }
}

fn validate_sibling_solid_name_uniqueness(fields: &[NodeBodyElement]) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();
    let mut seen: HashMap<String, Span> = HashMap::new();

    let child_solids = collect_immediate_child_solids(fields);
    for child_solid in child_solids {
        if let Some(first_span) = seen.get(&child_solid.name) {
            diagnostics.add(Diagnostic {
                span: child_solid.span.clone(),
                severity: Severity::Warning,
                message: format!(
                    "'name' field value should be unique among sibling Solid nodes: '{}'",
                    child_solid.name
                ),
                suggestion: Some(format!(
                    "Rename one of the sibling Solid nodes; first occurrence is at line {}.",
                    first_span.start_line
                )),
            });
        } else {
            seen.insert(child_solid.name, child_solid.span);
        }
    }

    diagnostics
}

#[derive(Debug)]
struct ChildSolidInfo {
    name: String,
    span: Span,
}

fn collect_immediate_child_solids(fields: &[NodeBodyElement]) -> Vec<ChildSolidInfo> {
    let Some(children_field) = get_node_field(fields, "children") else {
        return Vec::new();
    };

    collect_child_solids_from_value(&children_field.value)
}

fn collect_child_solids_from_value(value: &FieldValue) -> Vec<ChildSolidInfo> {
    match value {
        FieldValue::Node(node) => collect_child_solids_from_node(node),
        FieldValue::Array(array) => {
            let mut solids = Vec::new();
            for element in &array.elements {
                solids.extend(collect_child_solids_from_value(&element.value));
            }
            solids
        }
        _ => Vec::new(),
    }
}

fn collect_child_solids_from_node(node: &AstNode) -> Vec<ChildSolidInfo> {
    let AstNodeKind::Node {
        type_name, fields, ..
    } = &node.kind
    else {
        return Vec::new();
    };

    if type_name == "Solid" {
        return vec![ChildSolidInfo {
            name: get_effective_solid_name(node),
            span: node.span.clone(),
        }];
    }

    // Joint endPoint solids are siblings of regular children in Webots' Solid tree.
    if let Some(end_point) = get_node_field(fields, "endPoint")
        && let FieldValue::Node(end_point_node) = &end_point.value
        && is_solid_node(end_point_node)
    {
        return vec![ChildSolidInfo {
            name: get_effective_solid_name(end_point_node),
            span: end_point_node.span.clone(),
        }];
    }

    Vec::new()
}

fn is_solid_node(node: &AstNode) -> bool {
    matches!(
        &node.kind,
        AstNodeKind::Node {
            type_name,
            fields: _,
            def_name: _,
        } if type_name == "Solid"
    )
}

fn get_effective_solid_name(node: &AstNode) -> String {
    let AstNodeKind::Node { fields, .. } = &node.kind else {
        return "solid".to_string();
    };

    if let Some(name_field) = get_node_field(fields, "name")
        && let FieldValue::String(name) = &name_field.value
    {
        return name.clone();
    }

    "solid".to_string()
}

fn get_node_field<'a>(fields: &'a [NodeBodyElement], name: &str) -> Option<&'a NodeField> {
    fields.iter().find_map(|element| match element {
        NodeBodyElement::Field(field) if field.name == name => Some(field),
        _ => None,
    })
}

fn validate_node_field_semantics(
    parent_type_name: &str,
    field_name: &str,
    value: &FieldValue,
    span: &Span,
) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();

    if parent_type_name == "Shape"
        && field_name == "geometry"
        && let FieldValue::Node(node) = value
        && let AstNodeKind::Node { type_name, .. } = &node.kind
        && type_name == "CadShape"
    {
        diagnostics.add(Diagnostic {
            span: span.clone(),
            severity: Severity::Warning,
            message: "Skipped node: Cannot insert CadShape node in 'geometry' field of Shape node."
                .to_string(),
            suggestion: Some(
                "Use a geometry node supported by Shape.geometry, such as Mesh, Box, Sphere, or IndexedFaceSet."
                    .to_string(),
            ),
        });
    }

    diagnostics
}

/// Validates a field value against its expected type.
fn validate_field_value(
    value: &FieldValue,
    expected_type: &FieldType,
    span: &Span,
) -> DiagnosticSet {
    let mut diagnostics = DiagnosticSet::new();

    let scalar_type_match = matches!(
        (value, expected_type),
        (FieldValue::Bool(_), FieldType::SFBool)
            | (FieldValue::Int(_, _), FieldType::SFInt32)
            | (FieldValue::Float(_, _), FieldType::SFFloat)
            | (FieldValue::Int(_, _), FieldType::SFFloat)
            | (FieldValue::String(_), FieldType::SFString)
            | (FieldValue::Vec2f(_), FieldType::SFVec2f)
            | (FieldValue::Vec3f(_), FieldType::SFVec3f)
            | (FieldValue::Rotation(_), FieldType::SFRotation)
            | (FieldValue::Color(_), FieldType::SFColor)
            | (FieldValue::Node(_), FieldType::SFNode)
            | (FieldValue::Null, FieldType::SFNode)
    );

    if scalar_type_match {
        // Int can be promoted to Float by design.
    } else if let FieldValue::Array(arr) = value {
        if expected_type_is_multiple(expected_type) {
            if !validate_flat_numeric_array(arr, expected_type, span, &mut diagnostics) {
                let element_type = get_element_type(expected_type);
                for el in &arr.elements {
                    diagnostics.extend(validate_field_value(&el.value, &element_type, span));
                }
            }
        } else {
            diagnostics.add(Diagnostic {
                span: span.clone(),
                severity: Severity::Error,
                message: format!(
                    "Type mismatch: expected {}, found {:?}",
                    field_type_name(expected_type),
                    value
                ),
                suggestion: None,
            });
        }
    } else if matches!(value, FieldValue::Is(_)) {
        // Handled in validate_ast_node for IS bindings
    } else if let FieldValue::NumberSequence(seq) = value {
        if seq.elements.len() == 1 {
            // Treat single number sequence as a single number/float
            diagnostics.extend(validate_field_value(
                &seq.elements[0].value,
                expected_type,
                span,
            ));
        } else {
            // Check if sequence matches expected fixed-size type
            let length = seq.elements.len();
            if (expected_type == &FieldType::SFVec2f && length == 2)
                || (expected_type == &FieldType::SFVec3f && length == 3)
                || (expected_type == &FieldType::SFColor && length == 3)
                || (expected_type == &FieldType::SFRotation && length == 4)
            {
            } else {
                diagnostics.add(Diagnostic {
                    span: span.clone(),
                    severity: Severity::Error,
                    message: format!(
                        "Invalid number sequence length for {}",
                        field_type_name(expected_type)
                    ),
                    suggestion: None,
                });
            }
        }
    } else if matches!(value, FieldValue::Null) && expected_type == &FieldType::MFNode {
        // NULL is not allowed for MFNode (should be [])
        diagnostics.add(Diagnostic {
            span: span.clone(),
            severity: Severity::Error,
            message: "NULL not allowed for MFNode; use [] instead".to_string(),
            suggestion: Some("[]".to_string()),
        });
    } else if matches!(value, FieldValue::Template(_)) {
        // Templates are dynamic, assume they are correct type-wise for now
    } else {
        diagnostics.add(Diagnostic {
            span: span.clone(),
            severity: Severity::Error,
            message: format!(
                "Type mismatch: expected {}, found {:?}",
                field_type_name(expected_type),
                value
            ),
            suggestion: None,
        });
    }

    diagnostics
}

fn validate_flat_numeric_array(
    array: &ArrayValue,
    expected_type: &FieldType,
    span: &Span,
    diagnostics: &mut DiagnosticSet,
) -> bool {
    let Some(group_size) = flat_numeric_group_size(expected_type) else {
        return false;
    };

    if array.elements.is_empty() {
        return true;
    }

    if !array
        .elements
        .iter()
        .all(|element| is_numeric_scalar(&element.value))
    {
        return false;
    }

    if array.elements.len() % group_size != 0 {
        diagnostics.add(Diagnostic {
            span: span.clone(),
            severity: Severity::Error,
            message: format!(
                "Invalid flat array length for {}: expected a multiple of {} values",
                field_type_name(expected_type),
                group_size
            ),
            suggestion: None,
        });
    }

    true
}

fn flat_numeric_group_size(expected_type: &FieldType) -> Option<usize> {
    match expected_type {
        FieldType::MFVec2f => Some(2),
        FieldType::MFVec3f => Some(3),
        FieldType::MFRotation => Some(4),
        FieldType::MFColor => Some(3),
        _ => None,
    }
}

fn is_numeric_scalar(value: &FieldValue) -> bool {
    matches!(value, FieldValue::Int(_, _) | FieldValue::Float(_, _))
}

fn types_are_compatible(actual: &FieldType, expected: &FieldType) -> bool {
    // For IS bindings, Webots generally requires exact matches
    actual == expected
}

fn expected_type_is_multiple(t: &FieldType) -> bool {
    matches!(
        t,
        FieldType::MFBool
            | FieldType::MFInt32
            | FieldType::MFFloat
            | FieldType::MFString
            | FieldType::MFVec2f
            | FieldType::MFVec3f
            | FieldType::MFColor
            | FieldType::MFRotation
            | FieldType::MFNode
    )
}

fn get_element_type(t: &FieldType) -> FieldType {
    if *t == FieldType::MFBool {
        FieldType::SFBool
    } else if *t == FieldType::MFInt32 {
        FieldType::SFInt32
    } else if *t == FieldType::MFFloat {
        FieldType::SFFloat
    } else if *t == FieldType::MFString {
        FieldType::SFString
    } else if *t == FieldType::MFVec2f {
        FieldType::SFVec2f
    } else if *t == FieldType::MFVec3f {
        FieldType::SFVec3f
    } else if *t == FieldType::MFColor {
        FieldType::SFColor
    } else if *t == FieldType::MFRotation {
        FieldType::SFRotation
    } else if *t == FieldType::MFNode {
        FieldType::SFNode
    } else {
        t.clone()
    }
}

fn field_type_name(t: &FieldType) -> String {
    match t {
        FieldType::SFBool => "SFBool".to_string(),
        FieldType::SFInt32 => "SFInt32".to_string(),
        FieldType::SFFloat => "SFFloat".to_string(),
        FieldType::SFString => "SFString".to_string(),
        FieldType::SFVec2f => "SFVec2f".to_string(),
        FieldType::SFVec3f => "SFVec3f".to_string(),
        FieldType::SFRotation => "SFRotation".to_string(),
        FieldType::SFColor => "SFColor".to_string(),
        FieldType::SFNode => "SFNode".to_string(),
        FieldType::MFBool => "MFBool".to_string(),
        FieldType::MFInt32 => "MFInt32".to_string(),
        FieldType::MFFloat => "MFFloat".to_string(),
        FieldType::MFString => "MFString".to_string(),
        FieldType::MFVec2f => "MFVec2f".to_string(),
        FieldType::MFVec3f => "MFVec3f".to_string(),
        FieldType::MFRotation => "MFRotation".to_string(),
        FieldType::MFColor => "MFColor".to_string(),
        FieldType::MFNode => "MFNode".to_string(),
        FieldType::Unknown(s) => format!("Unknown({})", s),
    }
}

fn find_closest_match<'a>(
    name: &str,
    candidates: impl Iterator<Item = &'a str>,
) -> Option<&'a str> {
    let mut best_match = None;
    let mut best_distance = usize::MAX;

    for candidate in candidates {
        let distance = levenshtein_distance(name, candidate);
        if distance < best_distance && distance <= 3 {
            best_distance = distance;
            best_match = Some(candidate);
        }
    }

    best_match
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();
    let n = v1.len();
    let m = v2.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for (i, row) in dp.iter_mut().enumerate().take(n + 1) {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate().take(m + 1) {
        *cell = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            let cost = if v1[i - 1] == v2[j - 1] { 0 } else { 1 };
            dp[i][j] = std::cmp::min(
                dp[i - 1][j] + 1,
                std::cmp::min(dp[i][j - 1] + 1, dp[i - 1][j - 1] + cost),
            );
        }
    }

    dp[n][m]
}

impl IntoIterator for DiagnosticSet {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.into_iter()
    }
}
