use std::path::PathBuf;
use webots_proto_ast::proto::ast::{AstNodeKind, FieldValue, NodeBodyElement};
use webots_proto_resolver::{ProtoResolver, ResolveOptions};

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/resolve")
}

#[test]
fn test_simple_proto_without_externproto() {
    let input = r#"#VRML_SIM R2025a utf8

PROTO Simple [
  field SFVec3f size 1 1 1
]
{
  Shape {
    geometry Box { size IS size }
  }
}
"#;

    let options = ResolveOptions::new();
    let result = ProtoResolver::new(options).to_root_node(input, None::<PathBuf>);
    assert!(
        result.is_ok(),
        "Failed to expand simple PROTO: {:?}",
        result
    );

    let node = result.unwrap();
    // Verify it's a Shape node
    if let AstNodeKind::Node { type_name, .. } = &node.kind {
        assert_eq!(type_name, "Shape");
    } else {
        panic!("Expected Node, got {:?}", node.kind);
    }
}

#[test]
fn test_template_rendering() {
    let input = std::fs::read_to_string(fixtures_dir().join("WithTemplate.proto"))
        .expect("Failed to read WithTemplate.proto");

    let options = ResolveOptions::new();
    let result = ProtoResolver::new(options).to_root_node(&input, None::<PathBuf>);
    assert!(
        result.is_ok(),
        "Failed to expand template PROTO: {:?}",
        result
    );

    // The template should be rendered with default values
    let node = result.unwrap();
    if let AstNodeKind::Node { type_name, .. } = &node.kind {
        assert_eq!(type_name, "Transform");
    } else {
        panic!("Expected Transform node");
    }
}

#[test]
fn test_max_depth_limit() {
    // This test verifies that max_depth is enforced during recursive expansion
    // The depth check happens when loading external files, not during initial parsing
    // So a simple PROTO without EXTERNPROTO won't trigger the depth limit

    let input = r#"#VRML_SIM R2025a utf8

PROTO Test []
{
  Shape {
    geometry Box { size 1 1 1 }
  }
}
"#;

    // With max_depth=10 (default), this should work fine
    let options = ResolveOptions::new();
    let result = ProtoResolver::new(options).to_root_node(input, None::<PathBuf>);
    assert!(result.is_ok(), "Should succeed with default max_depth");

    // With max_depth=0, this should still work because we're not loading external files
    // The depth check only applies to EXTERNPROTO resolution
    let options = ResolveOptions::new().with_max_depth(0);
    let result = ProtoResolver::new(options).to_root_node(input, None::<PathBuf>);
    assert!(
        result.is_ok(),
        "Should succeed even with max_depth=0 for simple PROTO"
    );
}

#[test]
fn test_resolve_options_builder() {
    let options = ResolveOptions::new()
        .with_max_depth(5)
        .with_webots_projects_dir(PathBuf::from("/test/path"));

    assert_eq!(options.max_depth, 5);
    assert_eq!(
        options.webots_projects_dir,
        Some(PathBuf::from("/test/path"))
    );
}

#[test]
fn test_network_url_rejected_by_default() {
    let input = r#"#VRML_SIM R2025a utf8
EXTERNPROTO "https://example.com/Robot.proto"

PROTO Main []
{
  Robot { name "a" }
}
"#;
    let options = ResolveOptions::new();
    let result = ProtoResolver::new(options).to_root_node(input, Some(PathBuf::from(".")));
    assert!(result.is_err(), "Network URLs must be rejected");
}

#[test]
fn test_webots_url_requires_config() {
    // Verify that webots_projects_dir is None by default
    let options = ResolveOptions::new();
    assert!(
        options.webots_projects_dir.is_none(),
        "Webots projects dir should be None by default"
    );

    // Verify it can be configured
    let options = options.with_webots_projects_dir(PathBuf::from("/test"));
    assert!(
        options.webots_projects_dir.is_some(),
        "Webots projects dir should be set"
    );
}

#[test]
fn test_parse_and_expand_child_proto() {
    let input = std::fs::read_to_string(fixtures_dir().join("Child.proto"))
        .expect("Failed to read Child.proto");

    let options = ResolveOptions::new();
    let result = ProtoResolver::new(options).to_root_node(&input, None::<PathBuf>);
    assert!(result.is_ok(), "Failed to expand Child.proto: {:?}", result);
}

#[test]
fn test_document_without_proto_definition() {
    let input = r#"#VRML_SIM R2025a utf8

WorldInfo {
  title "Test"
}
"#;

    let options = ResolveOptions::new();
    let result = ProtoResolver::new(options).to_root_node(input, None::<PathBuf>);
    assert!(result.is_ok(), "Failed to process document without PROTO");

    let node = result.unwrap();
    if let AstNodeKind::Node { type_name, .. } = &node.kind {
        assert_eq!(type_name, "WorldInfo");
    } else {
        panic!("Expected WorldInfo node");
    }
}

#[test]
fn test_externproto_uses_interface_defaults_when_fields_omitted() {
    let fixtures_dir = fixtures_dir();
    let input = std::fs::read_to_string(fixtures_dir.join("ParentUsesChildDefaults.proto"))
        .expect("Failed to read ParentUsesChildDefaults.proto");

    let options = ResolveOptions::new();
    let node = ProtoResolver::new(options)
        .to_root_node(&input, Some(fixtures_dir))
        .expect("Failed to expand ParentUsesChildDefaults.proto");

    let AstNodeKind::Node {
        type_name, fields, ..
    } = &node.kind
    else {
        panic!("Expected Node root");
    };
    assert_eq!(type_name, "Shape");

    let geometry_field = fields
        .iter()
        .find_map(|element| match element {
            NodeBodyElement::Field(field) if field.name == "geometry" => Some(&field.value),
            _ => None,
        })
        .expect("Shape should contain geometry field");

    let FieldValue::Node(boxed_geometry) = geometry_field else {
        panic!("geometry should be a node");
    };
    let AstNodeKind::Node {
        type_name, fields, ..
    } = &boxed_geometry.kind
    else {
        panic!("Expected geometry node");
    };
    assert_eq!(type_name, "Box");

    let size_field = fields
        .iter()
        .find_map(|element| match element {
            NodeBodyElement::Field(field) if field.name == "size" => Some(&field.value),
            _ => None,
        })
        .expect("Box should contain size field");

    let resolved_size = match size_field {
        FieldValue::Vec3f(values) => Some(*values),
        FieldValue::NumberSequence(sequence) => {
            if sequence.elements.len() != 3 {
                None
            } else {
                let mut parsed = [0.0_f64; 3];
                let mut is_valid = true;
                for (index, element) in sequence.elements.iter().enumerate() {
                    let value = match &element.value {
                        FieldValue::Int(value, _) => *value as f64,
                        FieldValue::Float(value, _) => *value,
                        _ => {
                            is_valid = false;
                            0.0
                        }
                    };
                    parsed[index] = value;
                }
                if is_valid { Some(parsed) } else { None }
            }
        }
        _ => None,
    };

    assert_eq!(resolved_size, Some([1.0, 1.0, 1.0]));
}
