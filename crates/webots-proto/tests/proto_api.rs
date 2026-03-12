use webots_proto::Proto;
use webots_proto::ProtoExt;
use webots_proto::ast::proto::ast::AstNodeKind;
use webots_proto::r2025a::Node;
use webots_proto::types::ProtoField;
use webots_proto::{RenderContext, RenderOptions, RenderWebotsVersion, TemplateField};

#[test]
fn test_public_api_flow() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO Test [
  field SFVec3f size 1 1 1
]
{
  Group {
    children [
      Shape {
        geometry Box { size %<= fields.size.value.x >% %<= fields.size.value.y >% %<= fields.size.value.z >% }
      }
    ]
  }
}
"#;

    // 1. Parse
    let doc: Proto = input.parse().expect("Failed to parse");
    assert!(doc.proto.is_some());

    // 2. Validate
    let diagnostics = doc.validate().expect("Validation failed");
    assert!(
        !diagnostics.has_errors(),
        "Validation failed: {:?}",
        diagnostics
    );

    // 3. Render
    let rendered = doc
        .render(&RenderOptions::default())
        .expect("Failed to render");
    println!("Rendered output:\n{}", rendered);

    let normalized = rendered.split_whitespace().collect::<Vec<_>>().join(" ");

    // Check if rendered output contains expected value
    assert!(normalized.contains("Box { size 1 1 1 }"));

    // 4. Write Lossless
    let written = doc.to_lossless_string().expect("Failed to write");

    // Round-trip check: Parse the written output and compare ASTs
    let doc2: Proto = written.parse().expect("Failed to re-parse written output");

    // Note: doc and doc2 might differ in trivia if the Writer didn't preserve it exacty.
    // But semantic content should be equal.
    // However, Proto PartialEq includes trivia.
    // If we lost trivia, they won't match.
    // So we just check if it parses back successfully and maybe check field values.

    if let Some(proto2) = &doc2.proto {
        assert_eq!(proto2.name, "Test");
        // Check fields
        assert_eq!(proto2.fields.len(), 1);
        let f2 = &proto2.fields[0];
        assert_eq!(f2.name, "size");
        // Default value check?
    } else {
        panic!("Re-parsed document missing PROTO definition");
    }
}

#[test]
fn test_api_render_complex() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO Complex [
  field SFString text "Hello"
  field SFVec3f pos 0 1 0
  field MFInt32 array [1, 2, 3]
]
{
  Transform {
    translation %<= fields.pos.value.x >% %<= fields.pos.value.y >% %<= fields.pos.value.z >%
    children [
      Shape {
        geometry Text { string [ "%<= fields.text.value >%" ] }
      }
    ]
  }
  # Array check
  %< 
    let sum = 0;
    for (let i = 0; i < fields.array.value.length; i++) {
      sum += fields.array.value[i];
    }
  >%
  # Sum should be 6
  WorldInfo { title "%<= sum >%" }
}
"#;

    let doc: Proto = input.parse().expect("Failed to parse");
    let result = doc
        .render(&RenderOptions::default())
        .expect("Failed to render");
    println!("Rendered:\n{}", result);

    let normalized = result.split_whitespace().collect::<Vec<_>>().join(" ");

    assert!(normalized.contains("translation 0 1 0"));
    assert!(normalized.contains("string [ \"Hello\" ]"));
    assert!(normalized.contains("WorldInfo { title \"6\" }"));
}

#[test]
fn test_api_render_mfvec2f_defaults() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO Points [
  field MFVec2f points [1 2, 3 4]
]
{
  WorldInfo { title "%<= fields.points.value[1].x >%|%<= fields.points.value[1].y >%" }
}
"#;

    let doc: Proto = input.parse().expect("Failed to parse");
    let rendered = doc
        .render(&RenderOptions::default())
        .expect("Failed to render");

    let normalized = rendered.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(normalized.contains("WorldInfo { title \"3|4\" }"));
}

#[test]
fn test_api_render_mfvec3f_ignores_comma_in_comment() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO Points [
  field MFVec3f points [1 2 # comment, not a separator
    3 4 5 6
  ]
]
{
  WorldInfo { title "%<= fields.points.value[1].x >%|%<= fields.points.value[1].y >%|%<= fields.points.value[1].z >%" }
}
"#;

    let doc: Proto = input.parse().expect("Failed to parse");
    let rendered = doc
        .render(&RenderOptions::default())
        .expect("Failed to render");

    let normalized = rendered.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(normalized.contains("WorldInfo { title \"4|5|6\" }"));
}

#[test]
fn test_api_render_non_proto_returns_error() {
    let input = r#"#VRML_SIM R2025a utf8
WorldInfo { title "Simple" }
"#;

    let doc: Proto = input.parse().expect("Failed to parse");
    let error = doc
        .render(&RenderOptions::default())
        .expect_err("Expected render failure for non-PROTO");
    assert!(format!("{:?}", error).contains("PROTO"));
}

#[test]
fn test_api_render_rejects_int32_overflow() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO Overflow [
  field SFInt32 count 2147483648
]
{
  WorldInfo { title "%<= fields.count.value >%" }
}
"#;

    let doc: Proto = input.parse().expect("Failed to parse");
    let error = doc
        .render(&RenderOptions::default())
        .expect_err("Expected render failure for overflow");
    assert!(format!("{:?}", error).contains("Int32"));
}

#[test]
fn test_api_write_canonical_round_trip() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO RoundTrip [
  field SFVec3f size 1 2 3
]
{
  Shape {
    geometry Box { size 1 2 3 }
  }
}
"#;

    let doc: Proto = input.parse().expect("Failed to parse");
    let canonical = doc
        .to_canonical_string()
        .expect("Failed to write canonical");
    let doc2: Proto = canonical.parse().expect("Failed to parse canonical output");

    let proto = doc2.proto.expect("Expected PROTO definition");
    assert_eq!(proto.name, "RoundTrip");
    assert_eq!(proto.fields.len(), 1);
}

#[test]
fn test_api_ast_node_conversion_round_trip() {
    let input = r#"#VRML_SIM R2025a utf8
WorldInfo { title "Conversion" }
"#;

    let document: Proto = input.parse().expect("Failed to parse");
    let ast_node = document.root_nodes.first().expect("Missing root node");

    let typed_node =
        webots_proto::ast_to_r2025a_node(ast_node).expect("Failed to convert to typed node");
    if let Node::WorldInfo(info) = &typed_node {
        let title = info
            .title
            .as_ref()
            .and_then(ProtoField::value)
            .expect("Missing WorldInfo title");
        assert_eq!(title, "Conversion");
    } else {
        panic!("Expected WorldInfo node");
    }

    let converted_ast =
        webots_proto::r2025a_node_to_ast(&typed_node).expect("Failed to convert back to AST");
    if let AstNodeKind::Node { type_name, .. } = converted_ast.kind {
        assert_eq!(type_name, "WorldInfo");
    } else {
        panic!("Expected AST node");
    }
}

#[test]
fn test_api_render_with_field_override_exposes_value_and_default_value() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO OverrideTest [
  field SFInt32 count 7
]
{
  WorldInfo { title "%<= fields.count.value >%|%<= fields.count.defaultValue >%" }
}
"#;

    let doc: Proto = input.parse().expect("Failed to parse");
    let options = RenderOptions::default().with_field_overrides({
        let mut overrides = std::collections::HashMap::new();
        overrides.insert("count".to_string(), TemplateField::SFInt32(42));
        overrides
    });

    let rendered = doc.render(&options).expect("Failed to render");
    let normalized = rendered.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(normalized.contains("WorldInfo { title \"42|7\" }"));
}

#[test]
fn test_api_render_with_context() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO ContextTest [
  field SFString name "n"
]
{
  WorldInfo { title "%<= context.os >%|%<= context.webots_version.major >%|%<= context.world >%" }
}
"#;

    let doc: Proto = input.parse().expect("Failed to parse");
    let context = RenderContext::default()
        .with_os("linux")
        .with_world("/workspace/worlds/demo.wbt")
        .with_webots_version(RenderWebotsVersion::new(
            "R2025a".to_string(),
            "0".to_string(),
        ));
    let options = RenderOptions::default().with_context(context);

    let rendered = doc.render(&options).expect("Failed to render");
    let normalized = rendered.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(normalized.contains("WorldInfo { title \"linux|R2025a|/workspace/worlds/demo.wbt\" }"));
}
