#[cfg(feature = "validation")]
mod imports;

#[cfg(feature = "validation")]
use std::collections::HashSet;

use thiserror::Error;

pub use webots_proto_ast as ast;
#[cfg(feature = "resolver")]
pub use webots_proto_resolver as resolver;
#[cfg(feature = "schema")]
pub use webots_proto_schema as schema;
#[cfg(feature = "schema")]
pub use webots_proto_schema::types;

pub use webots_proto_ast::{AstNode, FieldValue, Proto};
#[cfg(feature = "resolver")]
pub use webots_proto_resolver::{ProtoResolver, ResolveOptions};
#[cfg(feature = "r2025a")]
pub use webots_proto_schema::r2025a;
#[cfg(feature = "schema")]
pub use webots_proto_schema::{
    Diagnostic, DiagnosticSet, Error as SchemaError, NodeSchema, Result as SchemaResult,
    SchemaField, Severity, ValidationContext, ast_to_r2025a_node, r2025a_node_to_ast,
};
#[cfg(feature = "template")]
pub use webots_proto_template::{
    RenderContext, RenderOptions, RenderWebotsVersion, TemplateContext, TemplateError,
    TemplateEvaluator, TemplateField, TemplateFieldBinding, TemplateWebotsVersion,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Ast(#[from] webots_proto_ast::ProtoError),
    #[cfg(feature = "template")]
    #[error(transparent)]
    Template(#[from] webots_proto_template::TemplateError),
    #[cfg(feature = "resolver")]
    #[error(transparent)]
    Resolver(#[from] webots_proto_resolver::ProtoError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "validation")]
#[derive(Debug, Clone, Default)]
pub struct ValidationOptions {
    pub local_externproto_naming: bool,
    pub runtime_semantics: bool,
}

#[cfg(feature = "validation")]
impl ValidationOptions {
    pub fn new() -> Self {
        Self {
            local_externproto_naming: true,
            runtime_semantics: true,
        }
    }

    pub fn with_local_externproto_naming(mut self, enabled: bool) -> Self {
        self.local_externproto_naming = enabled;
        self
    }

    pub fn with_runtime_semantics(mut self, enabled: bool) -> Self {
        self.runtime_semantics = enabled;
        self
    }
}

#[cfg(any(feature = "template", feature = "validation"))]
pub trait ProtoExt {
    #[cfg(feature = "template")]
    fn render(&self, options: &RenderOptions) -> Result<String>;
    #[cfg(feature = "validation")]
    fn validate(&self) -> Result<DiagnosticSet>;
    #[cfg(feature = "validation")]
    fn validate_with_options(&self, options: &ValidationOptions) -> Result<DiagnosticSet>;
}

#[cfg(any(feature = "template", feature = "validation"))]
impl ProtoExt for Proto {
    #[cfg(feature = "template")]
    fn render(&self, options: &RenderOptions) -> Result<String> {
        Ok(webots_proto_template::render(self, options)?)
    }

    #[cfg(feature = "validation")]
    fn validate(&self) -> Result<DiagnosticSet> {
        self.validate_with_options(&ValidationOptions::new())
    }

    #[cfg(feature = "validation")]
    fn validate_with_options(&self, options: &ValidationOptions) -> Result<DiagnosticSet> {
        validate_with_options(self, options)
    }
}

#[cfg(feature = "validation")]
pub fn validate(proto: &Proto) -> Result<DiagnosticSet> {
    validate_with_options(proto, &ValidationOptions::new())
}

#[cfg(feature = "validation")]
pub fn validate_with_options(proto: &Proto, options: &ValidationOptions) -> Result<DiagnosticSet> {
    let mut diagnostics = schema::validate(proto);

    if options.local_externproto_naming
        && let Some(source_path) = &proto.source_path
    {
        diagnostics.extend(imports::validate_local_externproto_naming(
            source_path,
            proto,
        )?);
    }

    if options.runtime_semantics
        && let Some(source_path) = &proto.source_path
    {
        let content = proto
            .source_content
            .clone()
            .map(Ok)
            .unwrap_or_else(|| std::fs::read_to_string(source_path).map_err(Error::from))?;

        if let Some(base_path) = source_path.parent() {
            let expanded_root = ProtoResolver::new(ResolveOptions::new())
                .to_root_node(&content, Some(base_path))?;
            diagnostics.extend(schema::validate_runtime_semantics(&expanded_root));
        }
    }

    Ok(dedupe_diagnostics(diagnostics))
}

#[cfg(feature = "validation")]
fn dedupe_diagnostics(diagnostics: DiagnosticSet) -> DiagnosticSet {
    let mut deduped = DiagnosticSet::new();
    let mut seen = HashSet::new();
    for diagnostic in diagnostics.iter() {
        if seen.insert(diagnostic.clone()) {
            deduped.add(diagnostic.clone());
        }
    }
    deduped
}
