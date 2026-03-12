use webots_proto_ast::proto::parser::Parser;
use webots_proto_schema::{Severity, validate};

#[test]
fn test_validate_wrong_field_type() {
    let input = r#"#VRML_SIM R2025a utf8
Robot {
  name 123
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    assert!(diagnostics.has_errors());
    let errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    for err in &errors {
        println!("Error: {}", err.message);
    }
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message.contains("Type mismatch")));
}

#[test]
fn test_validate_unknown_field() {
    let input = r#"#VRML_SIM R2025a utf8
Robot {
    unknownField "value"
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    let warnings: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();
    assert!(!warnings.is_empty());
    assert!(warnings[0].message.contains("Unknown field"));
}

#[test]
fn test_validate_unknown_node() {
    let input = r#"#VRML_SIM R2025a utf8
UnknownNode {
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    let warnings: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();
    assert!(!warnings.is_empty());
    assert!(warnings[0].message.contains("Unknown node type"));
}

#[test]
fn test_validate_undefined_is_binding() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO MyProto [
] {
  Robot {
    name IS myName
  }
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    assert!(diagnostics.has_errors());
    let errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    assert!(
        errors
            .iter()
            .any(|e| e.message.contains("Undefined IS reference"))
    );
}

#[test]
fn test_validate_valid_proto() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO MyProto [
  field SFString name "my_robot"
] {
  Robot {
    name IS name
  }
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    assert!(!diagnostics.has_errors());
}

#[test]
fn test_validate_vec3f_sequence() {
    let input = r#"#VRML_SIM R2025a utf8
Robot {
    translation 1 0 0
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    assert!(!diagnostics.has_errors());
}

#[test]
fn test_validate_invalid_vec3f_sequence() {
    let input = r#"#VRML_SIM R2025a utf8
Robot {
    translation 1 0
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    assert!(diagnostics.has_errors());
    let errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    assert!(errors[0].message.contains("Invalid number sequence length"));
}

#[test]
fn test_validate_invalid_use() {
    let input = r#"#VRML_SIM R2025a utf8
Group {
    children [
        USE MY_NODE
    ]
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    assert!(diagnostics.has_errors());
    let errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    assert!(
        errors
            .iter()
            .any(|e| e.message.contains("Undefined USE reference"))
    );
}

#[test]
fn test_validate_valid_def_use() {
    let input = r#"#VRML_SIM R2025a utf8
Group {
    children [
        DEF MY_NODE Shape {
        }
        USE MY_NODE
    ]
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    let errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    if !errors.is_empty() {
        let msg = errors
            .iter()
            .map(|e| format!("{} at {:?}", e.message, e.span))
            .collect::<Vec<_>>()
            .join("\n");
        panic!("Validation failed:\n{}", msg);
    }
}

#[test]
fn test_validate_restrictions_enforced() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO Sample [
  field SFString { "adult", "kid" } size "baby"
] {
  Robot {
    name IS size
  }
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    assert!(diagnostics.has_errors());
    let errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    assert!(
        errors
            .iter()
            .any(|e| e.message.contains("restriction") || e.message.contains("restrictions"))
    );
}

#[test]
fn test_validate_camel_case_fields() {
    let input = r#"#VRML_SIM R2025a utf8
Robot {
    controllerArgs ["arg1" "arg2"]
    customData "payload"
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    let warnings: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();
    assert!(warnings.is_empty());
}

#[test]
fn test_warns_when_solid_physics_has_no_inertia_source() {
    let input = r#"#VRML_SIM R2025a utf8
Solid {
  physics Physics {
    mass 1
  }
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    let warnings: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();
    assert!(
        warnings
            .iter()
            .any(|w| w.message.contains("Undefined inertia matrix"))
    );
}

#[test]
fn test_does_not_warn_when_solid_has_bounding_object() {
    let input = r#"#VRML_SIM R2025a utf8
Solid {
  boundingObject Box {
    size 1 1 1
  }
  physics Physics {
    mass 1
  }
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    let warnings: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();
    assert!(
        !warnings
            .iter()
            .any(|w| w.message.contains("Undefined inertia matrix"))
    );
}

#[test]
fn test_warns_on_duplicate_default_sibling_solid_names() {
    let input = r#"#VRML_SIM R2025a utf8
Solid {
  children [
    HingeJoint {
      endPoint Solid {
      }
    }
    HingeJoint {
      endPoint Solid {
      }
    }
  ]
}
"#;
    let mut parser = Parser::new(input);
    let doc = parser.parse_document().unwrap();
    let diagnostics = validate(&doc);

    let warnings: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();
    assert!(
        warnings
            .iter()
            .any(|w| w.message.contains("unique among sibling Solid nodes"))
    );
}
