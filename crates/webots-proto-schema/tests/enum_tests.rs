#[cfg(test)]
mod tests {
    use webots_proto_schema::r2025a::R2025aCodec;
    use webots_proto_schema::r2025a::enums::Controller;
    use webots_proto_schema::r2025a::nodes::Robot;

    #[test]
    fn test_controller_enum_serialization() {
        let none = Controller::None;
        let generic = Controller::Generic;
        let extern_ctrl = Controller::Extern;
        let specific = Controller::Specific("my_ctrl".to_string());

        assert_eq!(serde_json::to_string(&none).unwrap(), "\"<none>\"");
        assert_eq!(serde_json::to_string(&generic).unwrap(), "\"<generic>\"");
        assert_eq!(serde_json::to_string(&extern_ctrl).unwrap(), "\"<extern>\"");
        assert_eq!(serde_json::to_string(&specific).unwrap(), "\"my_ctrl\"");
    }

    #[test]
    fn test_robot_controller_deserialization() {
        let proto = r#"#VRML_SIM R2025a utf8
            Robot {
                name "test_robot"
                controller "<extern>"
                children []
            }
        "#;

        let robot: Robot = R2025aCodec::new()
            .decode(proto)
            .expect("Failed to parse Robot");
        if let Controller::Extern = robot.controller.unwrap().unwrap_value() {
        } else {
            panic!("Expected Controller::Extern");
        }
    }
}
