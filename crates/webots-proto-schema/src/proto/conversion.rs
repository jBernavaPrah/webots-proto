use super::ast::{
    ArrayElement, ArrayValue, AstNode, AstNodeKind, FieldValue, NodeBodyElement, NodeField,
    NumberSequence, NumberSequenceElement,
};
use super::builtin_nodes::get_builtin_schema;
use super::{ProtoError, ProtoResult};
use crate::proto::ast::FieldType;
use crate::proto::span::Span;
use crate::versions::r2025a::nodes::Node;
use serde_json::{Map, Number, Value};

pub(crate) fn ast_to_r2025a_node(node: &AstNode) -> ProtoResult<Node> {
    let value = ast_node_to_json(node)?;
    serde_json::from_value(value).map_err(|error| {
        ProtoError::SerializationError(format!(
            "Failed to deserialize typed node from AST: {error}"
        ))
    })
}

pub(crate) fn r2025a_node_to_ast(node: &Node) -> ProtoResult<AstNode> {
    let value = serde_json::to_value(node).map_err(|error| {
        ProtoError::SerializationError(format!(
            "Failed to serialize typed node to JSON value: {error}"
        ))
    })?;
    json_to_ast_node(&value)
}

pub(crate) fn ast_node_to_json(node: &AstNode) -> ProtoResult<Value> {
    match &node.kind {
        AstNodeKind::Use { use_name } => {
            let mut object = Map::new();
            object.insert("Use".to_string(), Value::String(use_name.clone()));
            Ok(Value::Object(object))
        }
        AstNodeKind::Node {
            type_name,
            def_name,
            fields,
        } => {
            let mut payload = Map::new();
            for element in fields {
                let NodeBodyElement::Field(field) = element else {
                    continue;
                };

                let expected_type = get_builtin_schema(type_name)
                    .and_then(|schema| schema.get_field_type(&field.name));

                if matches!(expected_type, Some(FieldType::SFNode))
                    && matches!(field.value, FieldValue::Null)
                {
                    // Typed node structs model explicit NULL SFNode as None/omitted.
                    continue;
                }

                let field_json = match &field.value {
                    FieldValue::Is(name) => {
                        let mut is_obj = Map::new();
                        is_obj.insert("Is".to_string(), Value::String(name.clone()));
                        Value::Object(is_obj)
                    }
                    value => {
                        let mut value_obj = Map::new();
                        value_obj.insert(
                            "Value".to_string(),
                            ast_field_value_to_json(value, expected_type.as_ref())?,
                        );
                        Value::Object(value_obj)
                    }
                };

                payload.insert(field.name.clone(), field_json);
            }

            let mut typed_node = Map::new();
            typed_node.insert(type_name.clone(), Value::Object(payload));
            let base_value = Value::Object(typed_node);

            if let Some(def_name) = def_name {
                let mut def = Map::new();
                def.insert(
                    "Def".to_string(),
                    Value::Array(vec![Value::String(def_name.clone()), base_value]),
                );
                Ok(Value::Object(def))
            } else {
                Ok(base_value)
            }
        }
    }
}

fn ast_field_value_to_json(
    value: &FieldValue,
    expected_type: Option<&FieldType>,
) -> ProtoResult<Value> {
    if let Some(field_type) = expected_type {
        return ast_field_value_to_json_with_type(value, field_type);
    }

    match value {
        FieldValue::Bool(boolean) => Ok(Value::Bool(*boolean)),
        FieldValue::Int(integer, _) => Ok(Value::Number(Number::from(*integer))),
        FieldValue::Float(float, _) => number_from_f64(*float),
        FieldValue::String(string) => Ok(Value::String(string.clone())),
        FieldValue::Vec2f(vector) => Ok(Value::Array(
            vector
                .iter()
                .map(|number| number_from_f64(*number))
                .collect::<ProtoResult<Vec<_>>>()?,
        )),
        FieldValue::Vec3f(vector) | FieldValue::Color(vector) => Ok(Value::Array(
            vector
                .iter()
                .map(|number| number_from_f64(*number))
                .collect::<ProtoResult<Vec<_>>>()?,
        )),
        FieldValue::Rotation(rotation) => Ok(Value::Array(
            rotation
                .iter()
                .map(|number| number_from_f64(*number))
                .collect::<ProtoResult<Vec<_>>>()?,
        )),
        FieldValue::Node(node) => ast_node_to_json(node),
        FieldValue::Array(array) => Ok(Value::Array(
            array
                .elements
                .iter()
                .map(|element| ast_field_value_to_json(&element.value, None))
                .collect::<ProtoResult<Vec<_>>>()?,
        )),
        FieldValue::NumberSequence(sequence) => Ok(Value::Array(
            sequence
                .elements
                .iter()
                .map(|element| ast_field_value_to_json(&element.value, None))
                .collect::<ProtoResult<Vec<_>>>()?,
        )),
        FieldValue::Null => Ok(Value::Null),
        FieldValue::Raw(raw) => Ok(Value::String(raw.clone())),
        FieldValue::Template(_) => Err(ProtoError::SerializationError(
            "Template values are not supported in typed node conversion".to_string(),
        )),
        FieldValue::Is(name) => {
            let mut is_obj = Map::new();
            is_obj.insert("Is".to_string(), Value::String(name.clone()));
            Ok(Value::Object(is_obj))
        }
    }
}

fn ast_field_value_to_json_with_type(
    value: &FieldValue,
    field_type: &FieldType,
) -> ProtoResult<Value> {
    match field_type {
        FieldType::SFBool => match value {
            FieldValue::Bool(boolean) => Ok(Value::Bool(*boolean)),
            FieldValue::Int(integer, _) => Ok(Value::Bool(*integer != 0)),
            _ => type_mismatch("SFBool", value),
        },
        FieldType::SFInt32 => match value {
            FieldValue::Int(integer, _) => Ok(Value::Number(Number::from(*integer))),
            FieldValue::Float(float, _) => Ok(Value::Number(Number::from(*float as i64))),
            FieldValue::NumberSequence(sequence) => {
                let first = sequence.elements.first().ok_or_else(|| {
                    ProtoError::SerializationError("Empty number sequence for SFInt32".to_string())
                })?;
                ast_field_value_to_json_with_type(&first.value, field_type)
            }
            _ => type_mismatch("SFInt32", value),
        },
        FieldType::SFFloat => match value {
            FieldValue::Float(float, _) => number_from_f64(*float),
            FieldValue::Int(integer, _) => number_from_f64(*integer as f64),
            FieldValue::NumberSequence(sequence) => {
                let first = sequence.elements.first().ok_or_else(|| {
                    ProtoError::SerializationError("Empty number sequence for SFFloat".to_string())
                })?;
                ast_field_value_to_json_with_type(&first.value, field_type)
            }
            _ => type_mismatch("SFFloat", value),
        },
        FieldType::SFString => match value {
            FieldValue::String(string) => Ok(Value::String(string.clone())),
            _ => type_mismatch("SFString", value),
        },
        FieldType::SFVec2f => numeric_tuple_json(value, 2),
        FieldType::SFVec3f | FieldType::SFColor => numeric_tuple_json(value, 3),
        FieldType::SFRotation => numeric_tuple_json(value, 4),
        FieldType::SFNode => match value {
            FieldValue::Node(node) => ast_node_to_json(node),
            FieldValue::Null => Ok(Value::Null),
            _ => type_mismatch("SFNode", value),
        },
        FieldType::MFBool
        | FieldType::MFInt32
        | FieldType::MFFloat
        | FieldType::MFString
        | FieldType::MFVec2f
        | FieldType::MFVec3f
        | FieldType::MFRotation
        | FieldType::MFColor
        | FieldType::MFNode => mf_value_to_json(value, field_type),
        FieldType::Unknown(_) => ast_field_value_to_json(value, None),
    }
}

fn numeric_tuple_json(value: &FieldValue, expected_len: usize) -> ProtoResult<Value> {
    let numbers = flatten_numeric_value(value)?;
    if numbers.len() != expected_len {
        return Err(ProtoError::SerializationError(format!(
            "Expected {expected_len} numbers, got {}",
            numbers.len()
        )));
    }

    Ok(Value::Array(
        numbers
            .into_iter()
            .map(number_from_f64)
            .collect::<ProtoResult<Vec<_>>>()?,
    ))
}

fn mf_value_to_json(value: &FieldValue, field_type: &FieldType) -> ProtoResult<Value> {
    let FieldValue::Array(array) = value else {
        return type_mismatch("MF* array", value);
    };

    let values = array
        .elements
        .iter()
        .map(|element| match field_type {
            FieldType::MFBool => {
                ast_field_value_to_json_with_type(&element.value, &FieldType::SFBool)
            }
            FieldType::MFInt32 => {
                ast_field_value_to_json_with_type(&element.value, &FieldType::SFInt32)
            }
            FieldType::MFFloat => {
                ast_field_value_to_json_with_type(&element.value, &FieldType::SFFloat)
            }
            FieldType::MFString => {
                ast_field_value_to_json_with_type(&element.value, &FieldType::SFString)
            }
            FieldType::MFVec2f => {
                ast_field_value_to_json_with_type(&element.value, &FieldType::SFVec2f)
            }
            FieldType::MFVec3f => {
                ast_field_value_to_json_with_type(&element.value, &FieldType::SFVec3f)
            }
            FieldType::MFRotation => {
                ast_field_value_to_json_with_type(&element.value, &FieldType::SFRotation)
            }
            FieldType::MFColor => {
                ast_field_value_to_json_with_type(&element.value, &FieldType::SFColor)
            }
            FieldType::MFNode => {
                ast_field_value_to_json_with_type(&element.value, &FieldType::SFNode)
            }
            _ => unreachable!("Only MF* fields are expected in this helper"),
        })
        .collect::<ProtoResult<Vec<_>>>()?;

    Ok(Value::Array(values))
}

fn flatten_numeric_value(value: &FieldValue) -> ProtoResult<Vec<f64>> {
    match value {
        FieldValue::Vec2f(vector) => Ok(vec![vector[0], vector[1]]),
        FieldValue::Vec3f(vector) => Ok(vec![vector[0], vector[1], vector[2]]),
        FieldValue::Color(vector) => Ok(vec![vector[0], vector[1], vector[2]]),
        FieldValue::Rotation(rotation) => {
            Ok(vec![rotation[0], rotation[1], rotation[2], rotation[3]])
        }
        FieldValue::NumberSequence(sequence) => sequence
            .elements
            .iter()
            .map(|element| match &element.value {
                FieldValue::Float(float, _) => Ok(*float),
                FieldValue::Int(integer, _) => Ok(*integer as f64),
                other => number_mismatch("number", other),
            })
            .collect(),
        FieldValue::Array(array) => array
            .elements
            .iter()
            .map(|element| match &element.value {
                FieldValue::Float(float, _) => Ok(*float),
                FieldValue::Int(integer, _) => Ok(*integer as f64),
                other => number_mismatch("number", other),
            })
            .collect(),
        _ => Err(ProtoError::SerializationError(format!(
            "Expected numeric tuple, got {value:?}"
        ))),
    }
}

fn json_to_ast_node(value: &Value) -> ProtoResult<AstNode> {
    let object = value.as_object().ok_or_else(|| {
        ProtoError::ParseError("Typed node JSON value is not an object".to_string())
    })?;

    if object.len() != 1 {
        return Err(ProtoError::ParseError(
            "Typed node JSON object must contain exactly one variant key".to_string(),
        ));
    }

    let (variant, payload) = object
        .iter()
        .next()
        .ok_or_else(|| ProtoError::ParseError("Missing node variant key".to_string()))?;

    if variant == "Use" {
        let name = payload.as_str().ok_or_else(|| {
            ProtoError::ParseError("Use variant payload must be a string".to_string())
        })?;
        return Ok(AstNode::new(
            AstNodeKind::Use {
                use_name: name.to_string(),
            },
            Span::default(),
        ));
    }

    if variant == "Def" {
        let array = payload.as_array().ok_or_else(|| {
            ProtoError::ParseError("Def variant payload must be [name, node]".to_string())
        })?;

        if array.len() != 2 {
            return Err(ProtoError::ParseError(
                "Def variant payload must have exactly 2 elements".to_string(),
            ));
        }

        let def_name = array[0]
            .as_str()
            .ok_or_else(|| ProtoError::ParseError("Def name must be a string".to_string()))?;
        let mut inner = json_to_ast_node(&array[1])?;
        if let AstNodeKind::Node { def_name: name, .. } = &mut inner.kind {
            *name = Some(def_name.to_string());
            return Ok(inner);
        }

        return Err(ProtoError::ParseError(
            "Def payload must wrap a standard node".to_string(),
        ));
    }

    let fields_object = payload.as_object().ok_or_else(|| {
        ProtoError::ParseError(format!(
            "Node variant '{variant}' payload must be an object"
        ))
    })?;

    let mut fields = Vec::new();
    for (field_name, field_payload) in fields_object {
        let expected_type =
            get_builtin_schema(variant).and_then(|schema| schema.get_field_type(field_name));

        let value = if let Some(field_object) = field_payload.as_object() {
            if let Some(is_ref) = field_object.get("Is") {
                let field_name = is_ref.as_str().ok_or_else(|| {
                    ProtoError::ParseError(format!(
                        "Field '{field_name}' IS binding must be a string"
                    ))
                })?;
                FieldValue::Is(field_name.to_string())
            } else if let Some(inner_value) = field_object.get("Value") {
                json_to_ast_field_value(inner_value, expected_type.as_ref())?
            } else {
                json_to_ast_field_value(field_payload, expected_type.as_ref())?
            }
        } else {
            json_to_ast_field_value(field_payload, expected_type.as_ref())?
        };

        fields.push(NodeBodyElement::Field(NodeField::new(
            field_name.clone(),
            value,
            Span::default(),
        )));
    }

    Ok(AstNode::new(
        AstNodeKind::Node {
            type_name: variant.to_string(),
            def_name: None,
            fields,
        },
        Span::default(),
    ))
}

fn json_to_ast_field_value(
    value: &Value,
    expected_type: Option<&FieldType>,
) -> ProtoResult<FieldValue> {
    if value.is_null() {
        return Ok(FieldValue::Null);
    }

    if let Some(field_type) = expected_type {
        return json_to_ast_field_value_with_type(value, field_type);
    }

    match value {
        Value::Bool(boolean) => Ok(FieldValue::Bool(*boolean)),
        Value::Number(number) => {
            if let Some(integer) = number.as_i64() {
                Ok(FieldValue::Int(integer, None))
            } else {
                Ok(FieldValue::Float(number.as_f64().unwrap_or_default(), None))
            }
        }
        Value::String(string) => Ok(FieldValue::String(string.clone())),
        Value::Array(array) => {
            let elements = array
                .iter()
                .map(|item| Ok(ArrayElement::new(json_to_ast_field_value(item, None)?)))
                .collect::<ProtoResult<Vec<_>>>()?;
            Ok(FieldValue::Array(ArrayValue::new().with_elements(elements)))
        }
        Value::Object(_) => {
            if let Ok(node) = json_to_ast_node(value) {
                Ok(FieldValue::Node(Box::new(node)))
            } else {
                Ok(FieldValue::Raw(value.to_string()))
            }
        }
        _ => Err(ProtoError::ParseError(format!(
            "Unsupported JSON field value: {value}"
        ))),
    }
}

fn json_to_ast_field_value_with_type(
    value: &Value,
    field_type: &FieldType,
) -> ProtoResult<FieldValue> {
    match field_type {
        FieldType::SFBool => Ok(FieldValue::Bool(value.as_bool().ok_or_else(|| {
            ProtoError::ParseError(format!("Expected bool for SFBool, got {value}"))
        })?)),
        FieldType::SFInt32 => {
            let integer = value.as_i64().ok_or_else(|| {
                ProtoError::ParseError(format!("Expected integer for SFInt32, got {value}"))
            })?;
            Ok(FieldValue::Int(integer, None))
        }
        FieldType::SFFloat => {
            let float = value.as_f64().ok_or_else(|| {
                ProtoError::ParseError(format!("Expected number for SFFloat, got {value}"))
            })?;
            Ok(FieldValue::Float(float, None))
        }
        FieldType::SFString => {
            let string = value.as_str().ok_or_else(|| {
                ProtoError::ParseError(format!("Expected string for SFString, got {value}"))
            })?;
            Ok(FieldValue::String(string.to_string()))
        }
        FieldType::SFVec2f => fixed_vec_to_field_value(value, 2),
        FieldType::SFVec3f => fixed_vec_to_field_value(value, 3),
        FieldType::SFRotation => fixed_vec_to_field_value(value, 4),
        FieldType::SFColor => fixed_vec_to_field_value(value, 3),
        FieldType::SFNode => {
            if value.is_null() {
                return Ok(FieldValue::Null);
            }
            Ok(FieldValue::Node(Box::new(json_to_ast_node(value)?)))
        }
        FieldType::MFBool
        | FieldType::MFInt32
        | FieldType::MFFloat
        | FieldType::MFString
        | FieldType::MFVec2f
        | FieldType::MFVec3f
        | FieldType::MFRotation
        | FieldType::MFColor
        | FieldType::MFNode => {
            let array = value.as_array().ok_or_else(|| {
                ProtoError::ParseError(format!("Expected array for {field_type:?}, got {value}"))
            })?;

            let element_type = match field_type {
                FieldType::MFBool => FieldType::SFBool,
                FieldType::MFInt32 => FieldType::SFInt32,
                FieldType::MFFloat => FieldType::SFFloat,
                FieldType::MFString => FieldType::SFString,
                FieldType::MFVec2f => FieldType::SFVec2f,
                FieldType::MFVec3f => FieldType::SFVec3f,
                FieldType::MFRotation => FieldType::SFRotation,
                FieldType::MFColor => FieldType::SFColor,
                FieldType::MFNode => FieldType::SFNode,
                _ => unreachable!("MF type expected"),
            };

            let elements = array
                .iter()
                .map(|item| {
                    let field_value = json_to_ast_field_value_with_type(item, &element_type)?;
                    Ok(ArrayElement::new(field_value))
                })
                .collect::<ProtoResult<Vec<_>>>()?;

            Ok(FieldValue::Array(ArrayValue::new().with_elements(elements)))
        }
        FieldType::Unknown(_) => json_to_ast_field_value(value, None),
    }
}

fn fixed_vec_to_field_value(value: &Value, expected_len: usize) -> ProtoResult<FieldValue> {
    let numbers = value
        .as_array()
        .ok_or_else(|| ProtoError::ParseError(format!("Expected number array, got {value}")))?;

    if numbers.len() != expected_len {
        return Err(ProtoError::ParseError(format!(
            "Expected {expected_len} values, got {}",
            numbers.len()
        )));
    }

    let parsed = numbers
        .iter()
        .map(|entry| {
            entry.as_f64().ok_or_else(|| {
                ProtoError::ParseError(format!("Expected numeric entry, got {entry}"))
            })
        })
        .collect::<ProtoResult<Vec<_>>>()?;

    match expected_len {
        2 => Ok(FieldValue::Vec2f([parsed[0], parsed[1]])),
        3 => Ok(FieldValue::Vec3f([parsed[0], parsed[1], parsed[2]])),
        4 => Ok(FieldValue::Rotation([
            parsed[0], parsed[1], parsed[2], parsed[3],
        ])),
        _ => {
            let elements = parsed
                .into_iter()
                .map(|number| NumberSequenceElement::new(FieldValue::Float(number, None)))
                .collect::<Vec<_>>();
            Ok(FieldValue::NumberSequence(
                NumberSequence::new().with_elements(elements),
            ))
        }
    }
}

fn number_from_f64(value: f64) -> ProtoResult<Value> {
    Number::from_f64(value).map(Value::Number).ok_or_else(|| {
        ProtoError::SerializationError(format!("Invalid floating-point value: {value}"))
    })
}

fn type_mismatch(expected: &str, value: &FieldValue) -> ProtoResult<Value> {
    Err(ProtoError::SerializationError(format!(
        "Expected {expected}, got {value:?}"
    )))
}

fn number_mismatch(expected: &str, value: &FieldValue) -> ProtoResult<f64> {
    Err(ProtoError::SerializationError(format!(
        "Expected {expected}, got {value:?}"
    )))
}
