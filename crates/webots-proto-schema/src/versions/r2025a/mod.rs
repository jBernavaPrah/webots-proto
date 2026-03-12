pub mod enums;
pub mod nodes;
pub use enums::*;
pub use nodes::*;

// Re-export common types
pub use crate::error::Error;
pub use crate::types;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

#[derive(Debug, Clone, Default)]
pub struct R2025aCodec;

impl R2025aCodec {
    pub fn new() -> Self {
        Self
    }

    /// Serializes a Webots R2025a node type to a string, including the version header.
    pub fn encode<T>(&self, value: &T) -> crate::error::Result<String>
    where
        T: Serialize + 'static,
    {
        let serialized = serde_json::to_value(value)
            .map_err(|error| crate::error::Error::Serialization(error.to_string()))?;
        let typed_node = self.value_to_node::<T>(serialized)?;
        let ast = crate::r2025a_node_to_ast(&typed_node)
            .map_err(|error| crate::error::Error::Serialization(format!("{error:?}")))?;

        let mut document = crate::proto::ast::Proto::new();
        document.header = Some(crate::proto::ast::Header::new(
            "R2025a".to_string(),
            "utf8".to_string(),
            None,
            crate::proto::span::Span::default(),
        ));
        document.root_nodes.push(ast);

        let content = document
            .to_canonical_string()
            .map_err(|error| crate::error::Error::Serialization(error.to_string()))?;
        Ok(content)
    }

    /// Deserializes a Webots R2025a PROTO/Node from a string, enforcing the version header.
    pub fn decode<T>(&self, input: &str) -> crate::error::Result<T>
    where
        T: DeserializeOwned,
    {
        let document: crate::proto::ast::Proto =
            input
                .parse()
                .map_err(|error: webots_proto_ast::ProtoError| {
                    crate::error::Error::Deserialization(error.to_string())
                })?;

        let header = document.header.ok_or_else(|| {
            crate::error::Error::Deserialization("Missing VRML header".to_string())
        })?;
        if header.version != "R2025a" || header.encoding != "utf8" {
            return Err(crate::error::Error::Deserialization(format!(
                "Unsupported header: #VRML_SIM {} {}",
                header.version, header.encoding
            )));
        }

        let first_node = document.root_nodes.first().ok_or_else(|| {
            crate::error::Error::Deserialization("Document has no root node".to_string())
        })?;
        let typed_node = crate::ast_to_r2025a_node(first_node)
            .map_err(|error| crate::error::Error::Deserialization(format!("{error:?}")))?;
        let node_value = serde_json::to_value(&typed_node)
            .map_err(|error| crate::error::Error::Deserialization(error.to_string()))?;

        match serde_json::from_value::<T>(node_value.clone()) {
            Ok(value) => Ok(value),
            Err(_) => {
                let payload = unwrap_single_variant_payload(node_value)?;
                serde_json::from_value(payload)
                    .map_err(|error| crate::error::Error::Deserialization(error.to_string()))
            }
        }
    }

    fn value_to_node<T>(&self, value: Value) -> crate::error::Result<Node>
    where
        T: Serialize + 'static,
    {
        if let Ok(node) = serde_json::from_value::<Node>(value.clone()) {
            return Ok(node);
        }

        let inferred_type_name = infer_node_type_name::<T>();
        let mut wrapped = Map::new();
        wrapped.insert(inferred_type_name.to_string(), value);

        serde_json::from_value(Value::Object(wrapped))
            .map_err(|error| crate::error::Error::Serialization(error.to_string()))
    }
}

fn unwrap_single_variant_payload(value: Value) -> crate::error::Result<Value> {
    let object = value.as_object().ok_or_else(|| {
        crate::error::Error::Deserialization("Typed node value is not an object".to_string())
    })?;
    if object.len() != 1 {
        return Err(crate::error::Error::Deserialization(
            "Typed node value has unexpected shape".to_string(),
        ));
    }

    object
        .iter()
        .next()
        .map(|(_, payload)| payload.clone())
        .ok_or_else(|| {
            crate::error::Error::Deserialization("Typed node value has no payload".to_string())
        })
}

fn infer_node_type_name<T>() -> &'static str {
    let type_name = std::any::type_name::<T>();
    let short = type_name.rsplit("::").next().unwrap_or(type_name);
    match short {
        "BoxNode" => "Box",
        _ => short,
    }
}
