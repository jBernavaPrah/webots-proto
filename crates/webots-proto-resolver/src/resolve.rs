//! PROTO resolver for expanding EXTERNPROTO dependencies.
//!
//! This module provides functionality to resolve and expand EXTERNPROTO declarations
//! in PROTO files, producing a fully expanded robot definition with no external references.

use super::{ProtoError, ProtoResult};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use webots_proto_ast::proto::ast::{
    AstNode, AstNodeKind, FieldValue, NodeBodyElement, Proto, ProtoBodyItem,
};
use webots_proto_ast::proto::parser::Parser;
use webots_proto_template::RenderOptions;

#[derive(Clone)]
struct ProtoExpansion {
    root_node: AstNode,
    interface_defaults: HashMap<String, FieldValue>,
}

/// Configuration options for PROTO resolution.
#[derive(Debug, Clone, Default)]
pub struct ResolveOptions {
    /// Optional path to Webots projects directory for resolving webots:// URLs.
    pub webots_projects_dir: Option<PathBuf>,
    /// Maximum recursion depth to prevent infinite loops.
    pub max_depth: usize,
}

impl ResolveOptions {
    /// Creates a new ResolveOptions with default values.
    pub fn new() -> Self {
        Self {
            webots_projects_dir: None,
            max_depth: 10,
        }
    }

    /// Sets the Webots projects directory for resolving webots:// URLs.
    pub fn with_webots_projects_dir(mut self, path: PathBuf) -> Self {
        self.webots_projects_dir = Some(path);
        self
    }

    /// Sets the maximum recursion depth.
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
}

/// PROTO resolver for expanding EXTERNPROTO dependencies.
pub struct ProtoResolver {
    options: ResolveOptions,
    /// Track visited files to detect circular dependencies.
    visited: HashSet<PathBuf>,
    /// Current recursion depth.
    depth: usize,
}

impl ProtoResolver {
    /// Creates a new ProtoResolver with the given options.
    pub fn new(options: ResolveOptions) -> Self {
        Self {
            options,
            visited: HashSet::new(),
            depth: 0,
        }
    }

    pub fn to_root_node(
        &mut self,
        input: &str,
        base_path: Option<impl AsRef<Path>>,
    ) -> ProtoResult<AstNode> {
        let base_path = base_path.map(|p| p.as_ref().to_path_buf());
        let mut parser = Parser::new(input);
        let doc = parser
            .parse_document()
            .map_err(|e| ProtoError::ParseError(format!("{:?}", e)))?;

        // Determine base path: use provided, or fall back to current directory
        let base_path = base_path
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        self.expand_document(&doc, &base_path)
    }

    /// Resolves an EXTERNPROTO URL to a local file path.
    fn resolve_url(&self, url: &str, base_path: impl AsRef<Path>) -> ProtoResult<PathBuf> {
        let base_path = base_path.as_ref();
        // Handle webots:// URLs
        if url.starts_with("webots://") {
            if let Some(ref webots_dir) = self.options.webots_projects_dir {
                let relative_path = url.strip_prefix("webots://").unwrap();
                return Ok(webots_dir.join(relative_path));
            } else {
                return Err(ProtoError::ParseError(format!(
                    "webots:// URL '{}' requires webots_projects_dir to be configured",
                    url
                )));
            }
        }

        // Handle https:// URLs
        if url.starts_with("https://") || url.starts_with("http://") {
            return Err(ProtoError::ParseError(format!(
                "Network URL '{}' is not supported by the resolver",
                url
            )));
        }

        // Handle local file paths (relative or absolute)
        let path = Path::new(url);
        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            // Resolve relative to the base path
            Ok(base_path.join(path))
        }
    }

    /// Loads and parses a PROTO file from the given path.
    fn load_proto(&mut self, path: impl AsRef<Path>) -> ProtoResult<Proto> {
        let path = path.as_ref();
        // Check for circular dependencies
        let canonical_path = path
            .canonicalize()
            .map_err(|e| ProtoError::ParseError(format!("Failed to canonicalize path: {}", e)))?;

        if self.visited.contains(&canonical_path) {
            return Err(ProtoError::ParseError(format!(
                "Circular dependency detected: {:?}",
                canonical_path
            )));
        }

        // Check recursion depth
        if self.depth >= self.options.max_depth {
            return Err(ProtoError::ParseError(format!(
                "Maximum recursion depth ({}) exceeded",
                self.options.max_depth
            )));
        }

        self.visited.insert(canonical_path.clone());
        self.depth += 1;

        // Load and parse the file
        let content = std::fs::read_to_string(&canonical_path).map_err(|e| {
            ProtoError::ParseError(format!("Failed to read file {:?}: {}", canonical_path, e))
        })?;

        let mut parser = Parser::new(&content);
        let doc = parser
            .parse_document()
            .map_err(|e| ProtoError::ParseError(format!("Parse error in {:?}: {:?}", path, e)))?;

        self.depth -= 1;

        Ok(doc)
    }

    /// Expands a PROTO document by resolving all EXTERNPROTO dependencies.
    fn expand_document(
        &mut self,
        doc: &Proto,
        base_path: impl AsRef<Path>,
    ) -> ProtoResult<AstNode> {
        let base_path = base_path.as_ref();
        // First, render templates if this is a PROTO definition
        let rendered_doc = if doc.proto.is_some() {
            let rendered_content = webots_proto_template::render(doc, &RenderOptions::default())?;
            let mut parser = Parser::new(&rendered_content);
            parser.parse_document().map_err(|e| {
                ProtoError::ParseError(format!("Failed to parse rendered template: {:?}", e))
            })?
        } else {
            doc.clone()
        };

        // Build a map of EXTERNPROTO declarations: PROTO name -> expanded node + defaults.
        let mut proto_definitions: HashMap<String, ProtoExpansion> = HashMap::new();

        // Preserve EXTERNPROTO declarations from the original parsed document.
        // Template rendering may output only PROTO body content and omit header directives.
        for ext in &doc.externprotos {
            let resolved_path = self.resolve_url(&ext.url, base_path)?;
            let nested_doc = self.load_proto(&resolved_path)?;
            let nested_base = resolved_path.parent().unwrap_or(Path::new("."));

            // Recursively expand the nested document
            let expanded_nested = self.expand_document(&nested_doc, nested_base)?;

            // Get the PROTO name from the nested document
            if let Some(proto) = &nested_doc.proto {
                let mut interface_defaults = HashMap::new();
                for field in &proto.fields {
                    if let Some(default_value) = field.default_value.clone() {
                        interface_defaults.insert(field.name.clone(), default_value);
                    }
                }
                proto_definitions.insert(
                    proto.name.clone(),
                    ProtoExpansion {
                        root_node: expanded_nested,
                        interface_defaults,
                    },
                );
            }
        }

        // Extract the root node from the PROTO body
        let mut root_node = if let Some(proto) = &rendered_doc.proto {
            // Find the first node in the PROTO body
            let mut found_node = None;
            for item in &proto.body {
                if let ProtoBodyItem::Node(node) = item {
                    found_node = Some(node.clone());
                    break;
                }
            }

            found_node.ok_or_else(|| {
                ProtoError::ParseError("PROTO definition has no root node".to_string())
            })?
        } else {
            // If not a PROTO, return the first root node
            rendered_doc
                .root_nodes
                .first()
                .ok_or_else(|| ProtoError::ParseError("Document has no root nodes".to_string()))?
                .clone()
        };

        // Inline EXTERNPROTO references and resolve IS bindings
        self.inline_proto_nodes(&mut root_node, &proto_definitions)?;
        self.normalize_mesh_urls(&mut root_node, base_path);

        Ok(root_node)
    }

    fn normalize_mesh_urls(&self, node: &mut AstNode, base_path: &Path) {
        let AstNodeKind::Node {
            type_name, fields, ..
        } = &mut node.kind
        else {
            return;
        };

        if type_name == "CadShape" {
            for element in fields.iter_mut() {
                let NodeBodyElement::Field(field) = element else {
                    continue;
                };
                if field.name != "url" {
                    continue;
                }
                self.normalize_mesh_field_value(&mut field.value, base_path);
            }
        }

        for element in fields.iter_mut() {
            let NodeBodyElement::Field(field) = element else {
                continue;
            };
            match &mut field.value {
                FieldValue::Node(child_node) => self.normalize_mesh_urls(child_node, base_path),
                FieldValue::Array(array) => {
                    for item in &mut array.elements {
                        if let FieldValue::Node(child_node) = &mut item.value {
                            self.normalize_mesh_urls(child_node, base_path);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn normalize_mesh_field_value(&self, field_value: &mut FieldValue, base_path: &Path) {
        match field_value {
            FieldValue::String(url) | FieldValue::Raw(url) => {
                *url = normalize_local_url(url, base_path);
            }
            FieldValue::Array(array) => {
                for element in &mut array.elements {
                    if let FieldValue::String(url) | FieldValue::Raw(url) = &mut element.value {
                        *url = normalize_local_url(url, base_path);
                    }
                }
            }
            _ => {}
        }
    }

    /// Recursively inline EXTERNPROTO nodes and resolve IS bindings.
    fn inline_proto_nodes(
        &self,
        node: &mut AstNode,
        proto_definitions: &HashMap<String, ProtoExpansion>,
    ) -> ProtoResult<()> {
        if let AstNodeKind::Node {
            type_name,
            def_name,
            fields,
        } = &mut node.kind
        {
            // Check if this node type is a PROTO that needs inlining
            if let Some(proto_expansion) = proto_definitions.get(type_name) {
                // Clone the PROTO definition
                let mut inlined_node = proto_expansion.root_node.clone();

                // Resolve IS bindings in the inlined node with values from the current node
                self.resolve_is_bindings(
                    &mut inlined_node,
                    fields,
                    &proto_expansion.interface_defaults,
                )?;

                // Preserve caller DEF naming on the inlined root node so downstream
                // frame/joint identifiers remain stable and human-readable.
                if let Some(caller_def_name) = def_name.clone()
                    && let AstNodeKind::Node {
                        def_name: inlined_def_name,
                        ..
                    } = &mut inlined_node.kind
                {
                    *inlined_def_name = Some(caller_def_name);
                }

                // Replace the current node with the inlined version
                *node = inlined_node;

                // Continue processing the inlined node
                if let AstNodeKind::Node { fields, .. } = &mut node.kind {
                    // Recursively process child nodes in the inlined content
                    for element in fields.iter_mut() {
                        if let NodeBodyElement::Field(field) = element {
                            if let FieldValue::Node(child_node) = &mut field.value {
                                self.inline_proto_nodes(child_node.as_mut(), proto_definitions)?;
                            } else if let FieldValue::Array(array) = &mut field.value {
                                for item in &mut array.elements {
                                    if let FieldValue::Node(child_node) = &mut item.value {
                                        self.inline_proto_nodes(
                                            child_node.as_mut(),
                                            proto_definitions,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // Not a PROTO node, just recursively process children
                for element in fields.iter_mut() {
                    if let NodeBodyElement::Field(field) = element {
                        if let FieldValue::Node(child_node) = &mut field.value {
                            self.inline_proto_nodes(child_node.as_mut(), proto_definitions)?;
                        } else if let FieldValue::Array(array) = &mut field.value {
                            for item in &mut array.elements {
                                if let FieldValue::Node(child_node) = &mut item.value {
                                    self.inline_proto_nodes(
                                        child_node.as_mut(),
                                        proto_definitions,
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Resolve IS bindings in a node by replacing them with actual values.
    fn resolve_is_bindings(
        &self,
        node: &mut AstNode,
        parent_fields: &[NodeBodyElement],
        interface_defaults: &HashMap<String, FieldValue>,
    ) -> ProtoResult<()> {
        // Build a map of field name -> value from parent
        let mut field_values: HashMap<String, FieldValue> = HashMap::new();
        for element in parent_fields {
            if let NodeBodyElement::Field(field) = element {
                field_values.insert(field.name.clone(), field.value.clone());
            }
        }

        // Recursively resolve IS bindings in this node
        if let AstNodeKind::Node { fields, .. } = &mut node.kind {
            for element in fields.iter_mut() {
                if let NodeBodyElement::Field(field) = element {
                    // Check if this field has an IS binding
                    if let FieldValue::Is(ref field_name) = field.value {
                        // Look up the value from the parent
                        if let Some(value) = field_values.get(field_name) {
                            field.value = value.clone();
                        } else if let Some(default_value) = interface_defaults.get(field_name) {
                            // If the caller omitted the field, use the PROTO interface default.
                            field.value = default_value.clone();
                        }
                    }

                    // Recursively process child nodes
                    match &mut field.value {
                        FieldValue::Node(child_node) => {
                            self.resolve_is_bindings(
                                child_node,
                                parent_fields,
                                interface_defaults,
                            )?;
                        }
                        FieldValue::Array(array) => {
                            for item in &mut array.elements {
                                if let FieldValue::Node(child_node) = &mut item.value {
                                    self.resolve_is_bindings(
                                        child_node,
                                        parent_fields,
                                        interface_defaults,
                                    )?;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}

fn normalize_local_url(url: &str, base_path: &Path) -> String {
    if url.contains("://") {
        return url.to_string();
    }

    let as_path = Path::new(url);
    if as_path.is_absolute() {
        return as_path.to_string_lossy().to_string();
    }

    let absolute_base = if base_path.is_absolute() {
        base_path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(base_path)
    };

    let candidate = absolute_base.join(as_path);
    candidate
        .canonicalize()
        .unwrap_or(candidate)
        .to_string_lossy()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_options_builder() {
        let options = ResolveOptions::new().with_max_depth(5);

        assert_eq!(options.max_depth, 5);
        assert!(options.webots_projects_dir.is_none());
    }

    #[test]
    fn test_resolve_local_path() {
        let resolver = ProtoResolver::new(ResolveOptions::new());
        let base = Path::new("/base/path");

        let result = resolver.resolve_url("Child.proto", base).unwrap();
        assert_eq!(result, PathBuf::from("/base/path/Child.proto"));
    }

    #[test]
    fn test_resolve_webots_url_without_config() {
        let resolver = ProtoResolver::new(ResolveOptions::new());
        let base = Path::new("/base/path");

        let result = resolver.resolve_url("webots://projects/robots/Robot.proto", base);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_webots_url_with_config() {
        let options =
            ResolveOptions::new().with_webots_projects_dir(PathBuf::from("/webots/assets"));
        let resolver = ProtoResolver::new(options);
        let base = Path::new("/base/path");

        let result = resolver
            .resolve_url("webots://projects/robots/Robot.proto", base)
            .unwrap();
        assert_eq!(
            result,
            PathBuf::from("/webots/assets/projects/robots/Robot.proto")
        );
    }

    #[test]
    fn test_reject_network_url_by_default() {
        let resolver = ProtoResolver::new(ResolveOptions::new());
        let base = Path::new("/base/path");

        let result = resolver.resolve_url("https://example.com/Robot.proto", base);
        assert!(result.is_err());
    }

    #[test]
    fn test_normalize_local_url_keeps_remote_urls() {
        let base = Path::new("/tmp");
        assert_eq!(
            normalize_local_url("https://example.com/mesh.obj", base),
            "https://example.com/mesh.obj"
        );
        assert_eq!(
            normalize_local_url("webots://projects/mesh.obj", base),
            "webots://projects/mesh.obj"
        );
    }
}
