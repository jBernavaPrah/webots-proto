use std::collections::HashMap;
use webots_proto_template::{
    TemplateContext, TemplateError, TemplateEvaluator, TemplateField, TemplateFieldBinding,
    TemplateWebotsVersion,
};

fn to_bindings(fields: &HashMap<String, TemplateField>) -> HashMap<String, TemplateFieldBinding> {
    let mut bindings = HashMap::with_capacity(fields.len());
    for (field_name, field_value) in fields {
        bindings.insert(
            field_name.clone(),
            TemplateFieldBinding::new(field_value.clone(), field_value.clone()),
        );
    }
    bindings
}

#[test]
fn test_simple_template() {
    let template = "Thing { info \"Hello %<=\"World\">%!\" }";
    let fields = HashMap::new();
    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert_eq!(result, "Thing { info \"Hello World!\" }");
}

#[test]
fn test_field_access() {
    let template = "Thing { info \"Value: %<= fields.foo.value >%\" }";
    let mut fields = HashMap::new();
    fields.insert(
        "foo".to_string(),
        TemplateField::SFString("bar".to_string()),
    );

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert_eq!(result, "Thing { info \"Value: bar\" }");
}

#[test]
fn test_complex_logic() {
    let template = "Thing { info \"%< if (fields.check.value) { >%Yes%< } else { >%No%< } >%\" }";
    let mut fields = HashMap::new();
    fields.insert("check".to_string(), TemplateField::SFBool(true));

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert_eq!(result, "Thing { info \"Yes\" }");
}

#[test]
fn test_template_parser_strings() {
    let template = "Thing { info \"Start %< var s = \"%> is not end\"; >% End\" }";
    let fields = HashMap::new();
    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert_eq!(result, "Thing { info \"Start  End\" }");
}

#[test]
fn test_template_literal_expression() {
    let template = "Thing { info \"%<= `prefix ${fields.foo.value}` >%\" }";
    let mut fields = HashMap::new();
    fields.insert(
        "foo".to_string(),
        TemplateField::SFString("bar".to_string()),
    );

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert_eq!(result, "Thing { info \"prefix bar\" }");
}

#[test]
fn test_math_deterministic() {
    let template =
        "Thing { info [ \"Random: %<= typeof Math.random >%\", \"Sin: %<= Math.sin(0) >%\" ] }";
    let fields = HashMap::new();
    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert_eq!(
        result,
        "Thing { info [ \"Random: undefined\", \"Sin: 0\" ] }"
    );
}

#[test]
fn test_procedural_geometry() {
    // Simulates a procedural Box with repeated structure
    let template = r#"
Group {
  children [
    %< for (var i = 0; i < fields.count.value; i++) { >%
       Transform {
         translation %<= i * 2 >% 0 0
         children [
           Box { size 1 1 1 }
         ]
       }
    %< } >%
  ]
}
"#;
    let mut fields = HashMap::new();
    fields.insert("count".to_string(), TemplateField::SFInt32(2));

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();

    // We expect valid VRML output. White space might vary, but content should match.
    // The output will contain escaped newlines or actual newlines depending on how we constructed chunks.
    // Our logic handles newlines in chunks.
    // We can rely on validation passing, and spot check usage.
    assert!(result.contains("Group {"));
    assert!(result.contains("Transform {"));
    assert!(result.contains("translation 0 0 0")); // i=0
    assert!(result.contains("translation 2 0 0")); // i=1
    assert!(!result.contains("translation 4 0 0")); // i=2 shouldn't exist
}

#[test]
fn test_fields_readonly() {
    let template = "Thing { info %< fields.foo.value = 'mutated'; >% \"%<= fields.foo.value >%\" }";
    let mut fields = HashMap::new();
    fields.insert(
        "foo".to_string(),
        TemplateField::SFString("original".to_string()),
    );

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    // Read-only fields should preserve the original value.
    assert!(result.contains("original"));
}

#[test]
fn test_invalid_vrml_generated() {
    // Deliberately missing closing brace to force a parse error on the generated output.
    let template = "Invalid { %<= fields.foo.value >%";
    let mut fields = HashMap::new();
    fields.insert("foo".to_string(), TemplateField::SFString("".to_string()));

    let result =
        TemplateEvaluator::new().evaluate_with_environment(template, &to_bindings(&fields));
    assert!(result.is_err());
    match result.unwrap_err() {
        TemplateError::ValidationError(message) => {
            assert!(message.contains("Syntax"));
        }
        other => panic!("Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_vector_js_representation() {
    let template = "Vec { x %<= fields.v.value.x >% y %<= fields.v.value.y >% }";
    let mut fields = HashMap::new();
    fields.insert("v".to_string(), TemplateField::SFVec2f(1.0, 2.0));

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert!(result.contains("x 1"));
    assert!(result.contains("y 2"));
}

#[test]
fn test_template_non_ascii_text() {
    let template = "Thing { info \"Olá %<= fields.word.value >% 🌍\" }";
    let mut fields = HashMap::new();
    fields.insert(
        "word".to_string(),
        TemplateField::SFString("mundo".to_string()),
    );

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert_eq!(result, "Thing { info \"Olá mundo 🌍\" }");
}

#[test]
fn test_template_literal_interpolation_with_marker() {
    let template = "Thing { info \"%<= `value ${\"inside >% marker\"}` >%\" }";
    let fields = HashMap::new();

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert_eq!(result, "Thing { info \"value inside >% marker\" }");
}

#[test]
fn test_vector_index_access() {
    let template = "Vec { x %<= fields.v.value[0] >% y %<= fields.v.value[1] >% }";
    let mut fields = HashMap::new();
    fields.insert("v".to_string(), TemplateField::SFVec2f(1.0, 2.0));

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert!(result.contains("x 1"));
    assert!(result.contains("y 2"));
}

#[test]
fn test_js_runtime_error() {
    let template = "Start %< var x = nonexistent; >% End";
    let fields = HashMap::new();
    let result =
        TemplateEvaluator::new().evaluate_with_environment(template, &to_bindings(&fields));
    assert!(result.is_err());
    let err = result.unwrap_err();
    println!("JS Error: {:?}", err);
    // Ideally we want to see line number here
}

#[test]
fn test_output_buffer_override_prevented() {
    let template = "Thing { info \"safe\" %< __webots_output_buffer.push('hacked'); >% }";
    let fields = HashMap::new();
    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert!(!result.contains("hacked"));
}

#[test]
fn test_output_writer_override_prevented() {
    let template = "Thing { info \"%< __webots_write = function(_) {}; >%safe\" }";
    let fields = HashMap::new();
    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &to_bindings(&fields))
        .unwrap();
    assert!(result.contains("safe"));
}

#[test]
fn test_field_binding_exposes_default_value() {
    let template = "Thing { info \"%<= fields.count.value >%|%<= fields.count.defaultValue >%\" }";
    let mut fields = HashMap::new();
    fields.insert(
        "count".to_string(),
        TemplateFieldBinding::new(TemplateField::SFInt32(42), TemplateField::SFInt32(7)),
    );

    let result = TemplateEvaluator::new()
        .evaluate_with_environment(template, &fields)
        .unwrap();
    assert_eq!(result, "Thing { info \"42|7\" }");
}

#[test]
fn test_context_is_available() {
    let template = "Thing { info \"%<= context.os >%|%<= context.webots_version.major >%\" }";
    let context = TemplateContext::default()
        .with_os("linux")
        .with_webots_version(TemplateWebotsVersion::new(
            "R2025a".to_string(),
            "0".to_string(),
        ));

    let result = TemplateEvaluator::with_context(context)
        .evaluate_with_environment(template, &HashMap::new())
        .unwrap();
    assert_eq!(result, "Thing { info \"linux|R2025a\" }");
}
