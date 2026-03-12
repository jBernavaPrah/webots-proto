use webots_proto_schema::r2025a::R2025aCodec;
use webots_proto_schema::r2025a::nodes::Robot;
use webots_proto_schema::types::SFVec3f;

#[test]
fn test_to_string_behavior() {
    let robot = Robot::new("test_robot").with_translation(SFVec3f::new(0.0, 1.0, 0.0));

    let output = R2025aCodec::new().encode(&robot).unwrap();
    println!("Output: {}", output);

    // Header included
    assert!(output.starts_with("#VRML_SIM R2025a utf8"));
}

#[test]
fn test_from_str_r2025a_strict() {
    let content = r#"#VRML_SIM R2025a utf8
Robot {
  name "strict_robot"
}
"#;
    let robot: Robot = R2025aCodec::new().decode(content).unwrap();
    assert_eq!(robot.name, "strict_robot");
}

#[test]
fn test_from_str_r2025a_fail_wrong_header() {
    let content = r#"#VRML_SIM R2023b utf8
Robot {
  name "old_robot"
}
"#;
    let result: Result<Robot, _> = R2025aCodec::new().decode(content);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Unsupported header"));
}

#[test]
fn test_from_str_r2025a_fail_missing_header() {
    let content = r#"
Robot {
  name "no_header"
}
"#;
    let result: Result<Robot, _> = R2025aCodec::new().decode(content);
    assert!(result.is_err());
}
