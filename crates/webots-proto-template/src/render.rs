use std::collections::HashMap;

use derive_new::new;
use derive_setters::Setters;
use webots_proto_ast::proto::ast::{
    ArrayValue, FieldType, FieldValue, NumberSequence, Proto, ProtoBodyItem,
};
use webots_proto_ast::proto::writer::ProtoWriter;

use crate::template::types::{
    TemplateContext, TemplateField, TemplateFieldBinding, TemplateWebotsVersion,
};
use crate::{TemplateError, TemplateEvaluator};

#[derive(Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option, into)]
pub struct RenderContext {
    pub world: Option<String>,
    pub proto: Option<String>,
    pub project_path: Option<String>,
    pub webots_version: Option<RenderWebotsVersion>,
    pub webots_home: Option<String>,
    pub temporary_files_path: Option<String>,
    pub os: Option<String>,
    pub id: Option<String>,
    pub coordinate_system: Option<String>,
}

#[derive(Debug, Clone, Default, new, Setters)]
#[setters(prefix = "with_", strip_option, into)]
pub struct RenderWebotsVersion {
    pub major: String,
    pub revision: String,
}

#[derive(Debug, Clone, Default, new, Setters)]
#[setters(prefix = "with_", strip_option)]
pub struct RenderOptions {
    pub field_overrides: HashMap<String, TemplateField>,
    pub context: RenderContext,
}

impl RenderOptions {
    pub fn with_field_overrides_from<I, K, V>(mut self, overrides: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<TemplateField>,
    {
        self.field_overrides = overrides
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        self
    }
}

pub fn render(proto: &Proto, options: &RenderOptions) -> Result<String, TemplateError> {
    let Some(proto_definition) = &proto.proto else {
        return Err(TemplateError::ValidationError(
            "Document does not contain a PROTO definition".to_string(),
        ));
    };

    let mut context_fields = HashMap::new();
    for field in &proto_definition.fields {
        let default_field_value = field
            .default_value
            .as_ref()
            .map(|default_value| {
                convert_to_template_field(default_value, &field.field_type).map_err(|error| {
                    TemplateError::ValidationError(format!("Field '{}': {}", field.name, error))
                })
            })
            .transpose()?;

        if let Some(override_field_value) = options.field_overrides.get(&field.name) {
            validate_override_type(&field.name, override_field_value, &field.field_type)?;
            let default_value_for_binding =
                default_field_value.unwrap_or_else(|| override_field_value.clone());
            context_fields.insert(
                field.name.clone(),
                TemplateFieldBinding::new(override_field_value.clone(), default_value_for_binding),
            );
            continue;
        }

        if let Some(default_value) = default_field_value {
            context_fields.insert(
                field.name.clone(),
                TemplateFieldBinding::new(default_value.clone(), default_value),
            );
        } else {
            return Err(TemplateError::ValidationError(format!(
                "Field '{}' has no default and no override value",
                field.name
            )));
        }
    }

    for override_field_name in options.field_overrides.keys() {
        if proto_definition
            .fields
            .iter()
            .all(|field| field.name != *override_field_name)
        {
            return Err(TemplateError::ValidationError(format!(
                "Unknown field override '{}': no matching PROTO interface field",
                override_field_name
            )));
        }
    }

    let body_content = if let Some(source) = &proto.source_content {
        if let (Some(first_item), Some(last_item)) =
            (proto_definition.body.first(), proto_definition.body.last())
        {
            let start = match first_item {
                ProtoBodyItem::Node(node) => node.span.start,
                ProtoBodyItem::Template(template) => template.span.start,
            };
            let end = match last_item {
                ProtoBodyItem::Node(node) => node.span.end,
                ProtoBodyItem::Template(template) => template.span.end,
            };
            source.get(start..end).unwrap_or("").to_string()
        } else {
            String::new()
        }
    } else {
        let writer = ProtoWriter::new();
        let mut body_content = String::new();
        for item in &proto_definition.body {
            match item {
                ProtoBodyItem::Node(node) => writer
                    .write_node(&mut body_content, node)
                    .map_err(|error| TemplateError::ValidationError(error.to_string()))?,
                ProtoBodyItem::Template(template) => writer
                    .write_template(&mut body_content, template)
                    .map_err(|error| TemplateError::ValidationError(error.to_string()))?,
            }
        }
        body_content
    };

    let template_context = convert_render_context(&options.context);
    TemplateEvaluator::with_context(template_context)
        .evaluate_with_environment(&body_content, &context_fields)
}

fn convert_render_context(context: &RenderContext) -> TemplateContext {
    TemplateContext {
        world: context.world.clone(),
        proto: context.proto.clone(),
        project_path: context.project_path.clone(),
        webots_home: context.webots_home.clone(),
        temporary_files_path: context.temporary_files_path.clone(),
        os: context.os.clone(),
        id: context.id.clone(),
        coordinate_system: context.coordinate_system.clone(),
        webots_version: context.webots_version.as_ref().map(|version| {
            TemplateWebotsVersion::new(version.major.clone(), version.revision.clone())
        }),
    }
}

fn convert_to_template_field(
    value: &FieldValue,
    field_type: &FieldType,
) -> Result<TemplateField, String> {
    if let (FieldValue::Bool(boolean), FieldType::SFBool) = (value, field_type) {
        return Ok(TemplateField::SFBool(*boolean));
    }
    if let (FieldValue::Int(integer, _), FieldType::SFInt32) = (value, field_type) {
        return Ok(TemplateField::SFInt32(convert_i64_to_i32(*integer)?));
    }
    if let (FieldValue::Int(integer, _), FieldType::SFFloat) = (value, field_type) {
        return Ok(TemplateField::SFFloat(*integer as f64));
    }
    if let (FieldValue::Float(float, _), FieldType::SFFloat) = (value, field_type) {
        return Ok(TemplateField::SFFloat(*float));
    }
    if let (FieldValue::String(string), FieldType::SFString) = (value, field_type) {
        return Ok(TemplateField::SFString(string.clone()));
    }
    if let (FieldValue::Vec2f(vector), FieldType::SFVec2f) = (value, field_type) {
        return Ok(TemplateField::SFVec2f(vector[0], vector[1]));
    }
    if let (FieldValue::Vec3f(vector), FieldType::SFVec3f) = (value, field_type) {
        return Ok(TemplateField::SFVec3f(vector[0], vector[1], vector[2]));
    }
    if let (FieldValue::Rotation(vector), FieldType::SFRotation) = (value, field_type) {
        return Ok(TemplateField::SFRotation(
            vector[0], vector[1], vector[2], vector[3],
        ));
    }
    if let (FieldValue::Color(vector), FieldType::SFColor) = (value, field_type) {
        return Ok(TemplateField::SFColor(vector[0], vector[1], vector[2]));
    }
    if let (FieldValue::Node(node), FieldType::SFNode) = (value, field_type) {
        let mut content = String::new();
        ProtoWriter::new()
            .write_node(&mut content, node)
            .map_err(|_| "Failed to serialize node".to_string())?;
        return Ok(TemplateField::SFNode(content));
    }
    if let (FieldValue::Null, FieldType::SFNode) = (value, field_type) {
        return Ok(TemplateField::SFNode("NULL".into()));
    }

    if let (FieldValue::NumberSequence(sequence), FieldType::SFVec2f) = (value, field_type)
        && sequence.elements.len() == 2
    {
        let numbers = extract_numbers_as_vec(sequence)?;
        return Ok(TemplateField::SFVec2f(numbers[0], numbers[1]));
    }
    if let (FieldValue::NumberSequence(sequence), FieldType::SFVec3f) = (value, field_type)
        && sequence.elements.len() == 3
    {
        let numbers = extract_numbers_as_vec(sequence)?;
        return Ok(TemplateField::SFVec3f(numbers[0], numbers[1], numbers[2]));
    }
    if let (FieldValue::NumberSequence(sequence), FieldType::SFColor) = (value, field_type)
        && sequence.elements.len() == 3
    {
        let numbers = extract_numbers_as_vec(sequence)?;
        return Ok(TemplateField::SFColor(numbers[0], numbers[1], numbers[2]));
    }
    if let (FieldValue::NumberSequence(sequence), FieldType::SFRotation) = (value, field_type)
        && sequence.elements.len() == 4
    {
        let numbers = extract_numbers_as_vec(sequence)?;
        return Ok(TemplateField::SFRotation(
            numbers[0], numbers[1], numbers[2], numbers[3],
        ));
    }
    if let (FieldValue::NumberSequence(sequence), FieldType::SFFloat) = (value, field_type)
        && sequence.elements.len() == 1
    {
        let numbers = extract_numbers_as_vec(sequence)?;
        return Ok(TemplateField::SFFloat(numbers[0]));
    }
    if let (FieldValue::NumberSequence(sequence), FieldType::SFInt32) = (value, field_type)
        && sequence.elements.len() == 1
    {
        let numbers = extract_numbers_as_int(sequence)?;
        return Ok(TemplateField::SFInt32(numbers[0]));
    }

    if let (FieldValue::Array(array), FieldType::MFBool) = (value, field_type) {
        return Ok(TemplateField::MFBool(extract_mf_values(
            array,
            |field_value| {
                if let FieldValue::Bool(boolean) = field_value {
                    Ok(*boolean)
                } else {
                    Err("Expected Bool".to_string())
                }
            },
        )?));
    }
    if let (FieldValue::Array(array), FieldType::MFInt32) = (value, field_type) {
        return Ok(TemplateField::MFInt32(extract_mf_values(
            array,
            |field_value| {
                if let FieldValue::Int(integer, _) = field_value {
                    Ok(convert_i64_to_i32(*integer)?)
                } else {
                    Err("Expected Int".to_string())
                }
            },
        )?));
    }
    if let (FieldValue::Array(array), FieldType::MFFloat) = (value, field_type) {
        return Ok(TemplateField::MFFloat(extract_mf_values(
            array,
            |field_value| {
                if let FieldValue::Float(float, _) = field_value {
                    Ok(*float)
                } else if let FieldValue::Int(integer, _) = field_value {
                    Ok(*integer as f64)
                } else {
                    Err("Expected Float".to_string())
                }
            },
        )?));
    }
    if let (FieldValue::Array(array), FieldType::MFString) = (value, field_type) {
        return Ok(TemplateField::MFString(extract_mf_values(
            array,
            |field_value| {
                if let FieldValue::String(string) = field_value {
                    Ok(string.clone())
                } else {
                    Err("Expected String".to_string())
                }
            },
        )?));
    }
    if let (FieldValue::Array(array), FieldType::MFVec2f) = (value, field_type) {
        return Ok(TemplateField::MFVec2f(
            extract_grouped_vectors(array, 2)?
                .into_iter()
                .map(|values| (values[0], values[1]))
                .collect(),
        ));
    }
    if let (FieldValue::Array(array), FieldType::MFVec3f) = (value, field_type) {
        return Ok(TemplateField::MFVec3f(
            extract_grouped_vectors(array, 3)?
                .into_iter()
                .map(|values| (values[0], values[1], values[2]))
                .collect(),
        ));
    }
    if let (FieldValue::Array(array), FieldType::MFRotation) = (value, field_type) {
        return Ok(TemplateField::MFRotation(extract_mf_values(
            array,
            |field_value| {
                if let FieldValue::Rotation(vector) = field_value {
                    Ok((vector[0], vector[1], vector[2], vector[3]))
                } else {
                    Err("Expected Rotation".to_string())
                }
            },
        )?));
    }
    if let (FieldValue::Array(array), FieldType::MFColor) = (value, field_type) {
        return Ok(TemplateField::MFColor(extract_mf_values(
            array,
            |field_value| {
                if let FieldValue::Color(vector) = field_value {
                    Ok((vector[0], vector[1], vector[2]))
                } else {
                    Err("Expected Color".to_string())
                }
            },
        )?));
    }
    if let (FieldValue::Array(array), FieldType::MFNode) = (value, field_type) {
        return Ok(TemplateField::MFNode(extract_mf_values(
            array,
            |field_value| {
                if let FieldValue::Node(node) = field_value {
                    let mut content = String::new();
                    ProtoWriter::new()
                        .write_node(&mut content, node)
                        .map_err(|_| "Failed to serialize node".to_string())?;
                    Ok(content)
                } else if matches!(field_value, FieldValue::Null) {
                    Ok("NULL".to_string())
                } else {
                    Err("Expected Node".to_string())
                }
            },
        )?));
    }

    Err(format!(
        "Unsupported field conversion from {:?} to {:?}",
        value, field_type
    ))
}

fn validate_override_type(
    field_name: &str,
    value: &TemplateField,
    field_type: &FieldType,
) -> Result<(), TemplateError> {
    let type_matches = matches!(
        (value, field_type),
        (TemplateField::SFBool(_), FieldType::SFBool)
            | (TemplateField::SFInt32(_), FieldType::SFInt32)
            | (TemplateField::SFFloat(_), FieldType::SFFloat)
            | (TemplateField::SFString(_), FieldType::SFString)
            | (TemplateField::SFVec2f(_, _), FieldType::SFVec2f)
            | (TemplateField::SFVec3f(_, _, _), FieldType::SFVec3f)
            | (TemplateField::SFRotation(_, _, _, _), FieldType::SFRotation)
            | (TemplateField::SFColor(_, _, _), FieldType::SFColor)
            | (TemplateField::SFNode(_), FieldType::SFNode)
            | (TemplateField::MFBool(_), FieldType::MFBool)
            | (TemplateField::MFInt32(_), FieldType::MFInt32)
            | (TemplateField::MFFloat(_), FieldType::MFFloat)
            | (TemplateField::MFString(_), FieldType::MFString)
            | (TemplateField::MFVec2f(_), FieldType::MFVec2f)
            | (TemplateField::MFVec3f(_), FieldType::MFVec3f)
            | (TemplateField::MFRotation(_), FieldType::MFRotation)
            | (TemplateField::MFColor(_), FieldType::MFColor)
            | (TemplateField::MFNode(_), FieldType::MFNode)
    );

    if type_matches {
        Ok(())
    } else {
        Err(TemplateError::ValidationError(format!(
            "Field '{}' override type mismatch: expected {:?}",
            field_name, field_type
        )))
    }
}

fn extract_numbers_as_vec(sequence: &NumberSequence) -> Result<Vec<f64>, String> {
    sequence
        .elements
        .iter()
        .map(|element| match &element.value {
            FieldValue::Float(value, _) => Ok(*value),
            FieldValue::Int(value, _) => Ok(*value as f64),
            other => Err(format!("Expected numeric element, got {:?}", other)),
        })
        .collect()
}

fn extract_numbers_as_int(sequence: &NumberSequence) -> Result<Vec<i32>, String> {
    sequence
        .elements
        .iter()
        .map(|element| match &element.value {
            FieldValue::Int(value, _) => convert_i64_to_i32(*value),
            other => Err(format!("Expected integer element, got {:?}", other)),
        })
        .collect()
}

fn extract_mf_values<T>(
    array: &ArrayValue,
    converter: impl Fn(&FieldValue) -> Result<T, String>,
) -> Result<Vec<T>, String> {
    array
        .elements
        .iter()
        .map(|element| converter(&element.value))
        .collect()
}

fn extract_grouped_vectors(array: &ArrayValue, width: usize) -> Result<Vec<Vec<f64>>, String> {
    let mut numbers = Vec::new();
    for element in &array.elements {
        match &element.value {
            FieldValue::Vec2f(vector) if width == 2 => numbers.push(vec![vector[0], vector[1]]),
            FieldValue::Vec3f(vector) if width == 3 => {
                numbers.push(vec![vector[0], vector[1], vector[2]])
            }
            FieldValue::NumberSequence(sequence) => {
                let values = extract_numbers_as_vec(sequence)?;
                if values.len() != width {
                    return Err(format!("Expected Vec{width}f"));
                }
                numbers.push(values);
            }
            FieldValue::Int(value, _) => {
                if let Some(last) = numbers.last_mut()
                    && last.len() < width
                {
                    last.push(*value as f64);
                } else {
                    numbers.push(vec![*value as f64]);
                }
            }
            FieldValue::Float(value, _) => {
                if let Some(last) = numbers.last_mut()
                    && last.len() < width
                {
                    last.push(*value);
                } else {
                    numbers.push(vec![*value]);
                }
            }
            _ => return Err(format!("Expected Vec{width}f")),
        }
    }

    if numbers.iter().any(|group| group.len() != width) {
        return Err(format!("Expected Vec{width}f"));
    }

    Ok(numbers)
}

fn convert_i64_to_i32(value: i64) -> Result<i32, String> {
    i32::try_from(value).map_err(|_| format!("Integer {} does not fit in Int32", value))
}
