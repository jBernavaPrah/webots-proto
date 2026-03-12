use webots_proto_schema::r2025a::R2025aCodec;
use webots_proto_schema::r2025a::nodes::Robot;
use webots_proto_schema::types::ProtoField;

#[test]
fn test_proto_deserialization() {
    let proto = r#"#VRML_SIM R2025a utf8
Robot {
  translation IS translation
  name IS name
  supervisor TRUE
}
"#;

    let robot: Robot = R2025aCodec::new()
        .decode(proto)
        .expect("Should deserialize Robot");

    if let Some(ProtoField::Is(s)) = &robot.translation {
        assert_eq!(s, "translation");
    } else {
        panic!("translation should be IS translation");
    }

    if let ProtoField::Is(s) = &robot.name {
        assert_eq!(s, "name");
    } else {
        panic!("name should be IS name");
    }
}
