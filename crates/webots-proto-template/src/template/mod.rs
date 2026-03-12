use boa_engine::object::ObjectInitializer;
use boa_engine::property::{Attribute, PropertyKey};
use boa_engine::{Context, JsString, Source};
use std::collections::HashMap;
use thiserror::Error;
use webots_proto_ast::proto::parser::Parser;

pub mod js_api;
pub mod parser;
pub mod types;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("JS execution error: {0}")]
    JsError(String), // We might lose span info if not careful, but we'll try to map it later
    #[error("Template parsing error: {0}")]
    ParseError(String),
    #[error("Generated VRML invalid: {0}")]
    ValidationError(String),
}

use crate::template::parser::{TemplateChunk, parse};
use crate::template::types::{
    TemplateContext, TemplateField, TemplateFieldBinding, TemplateWebotsVersion,
};

#[derive(Debug, Clone, Default)]
pub struct TemplateEvaluator {
    context_data: TemplateContext,
}

impl TemplateEvaluator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_context(context_data: TemplateContext) -> Self {
        Self { context_data }
    }

    /// Evaluates a Webots PROTO template with explicit field bindings and context.
    ///
    /// This mirrors Webots procedural PROTO behavior:
    /// - `fields.<name>.value`
    /// - `fields.<name>.defaultValue`
    /// - global `context` object with fixed metadata keys.
    pub fn evaluate_with_environment(
        &self,
        template_content: &str,
        fields: &HashMap<String, TemplateFieldBinding>,
    ) -> Result<String, TemplateError> {
        evaluate_template_with_environment(template_content, fields, &self.context_data)
    }

    /// Evaluates a template with value-only field inputs.
    ///
    /// For each entry, `defaultValue` mirrors `value`.
    pub fn evaluate_with_fields<I, K, V>(
        &self,
        template_content: &str,
        fields: I,
    ) -> Result<String, TemplateError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<TemplateField>,
    {
        let bindings = fields
            .into_iter()
            .map(|(field_name, field_value)| {
                let field_value = field_value.into();
                (
                    field_name.into(),
                    TemplateFieldBinding::new(field_value.clone(), field_value),
                )
            })
            .collect::<HashMap<_, _>>();
        self.evaluate_with_environment(template_content, &bindings)
    }

    /// Evaluates a template with fully specified field bindings.
    pub fn evaluate_with_bindings<I, K>(
        &self,
        template_content: &str,
        fields: I,
    ) -> Result<String, TemplateError>
    where
        I: IntoIterator<Item = (K, TemplateFieldBinding)>,
        K: Into<String>,
    {
        let bindings = fields
            .into_iter()
            .map(|(field_name, binding)| (field_name.into(), binding))
            .collect::<HashMap<_, _>>();
        self.evaluate_with_environment(template_content, &bindings)
    }
}

/// Evaluates a Webots PROTO template with explicit field bindings and context.
///
/// This mirrors Webots procedural PROTO behavior:
/// - `fields.<name>.value`
/// - `fields.<name>.defaultValue`
/// - global `context` object with fixed metadata keys.
fn evaluate_template_with_environment(
    template_content: &str,
    fields: &HashMap<String, TemplateFieldBinding>,
    context_data: &TemplateContext,
) -> Result<String, TemplateError> {
    // 1. Parse the template
    let chunks =
        parse(template_content).map_err(|e| TemplateError::ParseError(format!("{:?}", e)))?;

    // 2. Setup JS Context
    let mut context = Context::default();

    // Remove nondeterministic globals
    let global = context.global_object();
    global
        .delete_property_or_throw(PropertyKey::from(JsString::from("Date")), &mut context)
        .ok();

    if let Ok(math_obj) = global.get(PropertyKey::from(JsString::from("Math")), &mut context)
        && let Some(math_obj) = math_obj.as_object()
    {
        math_obj
            .delete_property_or_throw(PropertyKey::from(JsString::from("random")), &mut context)
            .ok();
    }

    // 3. Setup Output Buffer
    // We utilize a closure to hide the real accumulator from the user.
    // `__webots_output_buffer` is exposed as a dummy to prevent interference.
    // `__webots_write` is locked down.
    let init_script = r#"
        (function() {
            var real_buffer = [];
            
            // Expose a dummy buffer for users who try to access it
            var dummy_buffer = [];
            Object.defineProperty(this, "__webots_output_buffer", {
                value: dummy_buffer,
                writable: false,
                configurable: false,
                enumerable: false
            });

            // The write function pushes to the real buffer
            Object.defineProperty(this, "__webots_write", {
                value: function(s) {
                    real_buffer.push(String(s));
                },
                writable: false,
                configurable: false,
                enumerable: false
            });
            
            // Internal accessor for Rust to retrieve output
            Object.defineProperty(this, "__webots_get_output_internal", {
                value: function() {
                    return real_buffer.join("");
                },
                writable: false,
                configurable: false,
                enumerable: false
            });
        }).call(this);
    "#;
    context
        .eval(Source::from_bytes(init_script.as_bytes()))
        .map_err(|e| TemplateError::JsError(e.to_string()))?;

    // 4. Map fields
    let fields_obj = ObjectInitializer::new(&mut context).build();
    for (field_name, field_binding) in fields {
        let field_value = field_binding.value.to_js_value(&mut context);
        let field_default_value = field_binding.default_value.to_js_value(&mut context);
        let field_wrapper = ObjectInitializer::new(&mut context)
            .property(
                PropertyKey::from(JsString::from("value")),
                field_value,
                Attribute::READONLY,
            )
            .property(
                PropertyKey::from(JsString::from("defaultValue")),
                field_default_value,
                Attribute::READONLY,
            )
            .build();

        fields_obj
            .set(
                PropertyKey::from(JsString::from(field_name.as_str())),
                field_wrapper,
                false,
                &mut context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }

    context
        .register_global_property(
            PropertyKey::from(JsString::from("fields")),
            fields_obj,
            Attribute::READONLY,
        )
        .map_err(|e| TemplateError::JsError(e.to_string()))?;

    // 5. Register fixed `context` metadata object
    let context_object = build_context_object(context_data, &mut context)?;
    context
        .register_global_property(
            PropertyKey::from(JsString::from("context")),
            context_object,
            Attribute::READONLY,
        )
        .map_err(|e| TemplateError::JsError(e.to_string()))?;

    // 6. Transpile chunks to JS calls to `__webots_write`
    let mut js_code = String::new();
    for chunk in chunks {
        match chunk {
            TemplateChunk::Text { content, .. } => {
                // Use serde_json to produce a safe JS string literal
                match serde_json::to_string(&content) {
                    Ok(s) => js_code.push_str(&format!("__webots_write({});\n", s)),
                    Err(_) => {
                        return Err(TemplateError::ParseError(
                            "Failed to serialize text chunk".into(),
                        ));
                    }
                }
            }
            TemplateChunk::ExpressionBlock { content, .. } => {
                js_code.push_str(&format!("__webots_write({});\n", content));
            }
            TemplateChunk::ExecutionBlock { content, .. } => {
                js_code.push_str(&content);
                js_code.push('\n');
            }
        }
    }

    // 7. Execute
    match context.eval(Source::from_bytes(js_code.as_bytes())) {
        Ok(_) => {
            // Retrieve output
            let output_script = r#"__webots_get_output_internal()"#;
            match context.eval(Source::from_bytes(output_script.as_bytes())) {
                Ok(res) => {
                    match res.as_string() {
                        Some(s) => {
                            let output = s
                                .to_std_string()
                                .map_err(|e| TemplateError::JsError(e.to_string()))?;

                            // 8. Validate generated output (Basic parse check)
                            // We use strict parsing for the generated body
                            let mut parser = Parser::new(&output);
                            match parser.parse_body() {
                                Ok(_) => Ok(output),
                                Err(e) => Err(TemplateError::ValidationError(format!("{:?}", e))),
                            }
                        }
                        None => Ok(String::new()), // Empty output
                    }
                }
                Err(e) => Err(TemplateError::JsError(e.to_string())),
            }
        }
        Err(e) => {
            // TODO: Attempt to map line number from `e` back to `chunks` spans
            // For now just return the JS error.
            Err(TemplateError::JsError(e.to_string()))
        }
    }
}

fn build_context_object(
    context_data: &TemplateContext,
    context: &mut Context,
) -> Result<boa_engine::JsValue, TemplateError> {
    let context_object = ObjectInitializer::new(context).build();

    if let Some(world_path) = &context_data.world {
        context_object
            .set(
                PropertyKey::from(JsString::from("world")),
                JsString::from(world_path.as_str()),
                false,
                context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }
    if let Some(proto_path) = &context_data.proto {
        context_object
            .set(
                PropertyKey::from(JsString::from("proto")),
                JsString::from(proto_path.as_str()),
                false,
                context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }
    if let Some(project_path) = &context_data.project_path {
        context_object
            .set(
                PropertyKey::from(JsString::from("project_path")),
                JsString::from(project_path.as_str()),
                false,
                context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }
    if let Some(webots_home) = &context_data.webots_home {
        context_object
            .set(
                PropertyKey::from(JsString::from("webots_home")),
                JsString::from(webots_home.as_str()),
                false,
                context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }
    if let Some(temporary_files_path) = &context_data.temporary_files_path {
        context_object
            .set(
                PropertyKey::from(JsString::from("temporary_files_path")),
                JsString::from(temporary_files_path.as_str()),
                false,
                context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }
    if let Some(operating_system) = &context_data.os {
        context_object
            .set(
                PropertyKey::from(JsString::from("os")),
                JsString::from(operating_system.as_str()),
                false,
                context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }
    if let Some(node_id) = &context_data.id {
        context_object
            .set(
                PropertyKey::from(JsString::from("id")),
                JsString::from(node_id.as_str()),
                false,
                context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }
    if let Some(coordinate_system) = &context_data.coordinate_system {
        context_object
            .set(
                PropertyKey::from(JsString::from("coordinate_system")),
                JsString::from(coordinate_system.as_str()),
                false,
                context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }
    if let Some(version) = &context_data.webots_version {
        context_object
            .set(
                PropertyKey::from(JsString::from("webots_version")),
                build_webots_version_object(version, context)?,
                false,
                context,
            )
            .map_err(|e| TemplateError::JsError(e.to_string()))?;
    }

    Ok(context_object.into())
}

fn build_webots_version_object(
    version: &TemplateWebotsVersion,
    context: &mut Context,
) -> Result<boa_engine::JsValue, TemplateError> {
    let version_object = ObjectInitializer::new(context).build();
    version_object
        .set(
            PropertyKey::from(JsString::from("major")),
            JsString::from(version.major.as_str()),
            false,
            context,
        )
        .map_err(|e| TemplateError::JsError(e.to_string()))?;
    version_object
        .set(
            PropertyKey::from(JsString::from("revision")),
            JsString::from(version.revision.as_str()),
            false,
            context,
        )
        .map_err(|e| TemplateError::JsError(e.to_string()))?;

    Ok(version_object.into())
}
