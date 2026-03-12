use webots_proto::Proto;
use webots_proto::{ProtoExt, RenderOptions};

#[test]
fn test_workflow_valid_proto() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO MyRobot [
  field SFVec3f translation 0 0 0
  field SFString name "robot"
]
{
  Robot {
    translation IS translation
    name IS name
  }
}
"#;

    // 1. Parse
    let doc: Proto = input.parse().expect("Parse failed");

    // 2. Validate
    let diagnostics = doc.validate().expect("Validation failed");
    assert!(
        !diagnostics.has_errors(),
        "Validation failed: {:?}",
        diagnostics
    );

    // 3. Render (noop for this proto but checks it doesn't crash)
    let rendered = doc
        .render(&RenderOptions::default())
        .expect("Render failed");
    // render_templates returns the body. For this simple proto, it should just be the nodes.
    // However, write_node writes "Robot { ... }".
    assert!(rendered.contains("Robot {"));

    // 4. Write
    let output = doc.to_lossless_string().expect("Write failed");
    assert_eq!(output, input);
}

#[test]
fn test_workflow_template_proto() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO MyBox [
  field SFVec3f size 1 1 1
]
{
  Box {
    size %<= fields.size.value.x >% %<= fields.size.value.y >% %<= fields.size.value.z >%
  }
}
"#;

    let doc: Proto = input.parse().expect("Parse failed");

    let diagnostics = doc.validate().expect("Validation failed");
    assert!(!diagnostics.has_errors());

    let rendered = doc
        .render(&RenderOptions::default())
        .expect("Render failed");
    // Check if values were interpolated (defaults are 1 1 1)
    assert!(rendered.contains("Box {"));
    // The whitespace might vary depending on how evaluation works, but "size 1 1 1" should be present if logic is correct
    // Or "size 1.0 1.0 1.0" depending on formatting of numbers in JS engine output?
    // The engine probably outputs numbers as strings.
    assert!(rendered.contains("size 1 1 1") || rendered.contains("size 1.0 1.0 1.0"));
}

#[test]
fn test_workflow_invalid_proto() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO MyRobot [
  field SFInt32 val "string"
]
{
  Robot {}
}
"#;

    let doc: Proto = input.parse().expect("Parse failed");
    let diagnostics = doc.validate().expect("Validation failed");
    assert!(diagnostics.has_errors());

    let error = diagnostics
        .iter()
        .find(|d| d.message.contains("Type mismatch"));
    assert!(error.is_some());
}
