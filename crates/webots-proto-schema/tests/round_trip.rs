use webots_proto_schema::r2025a::{Node, R2025aCodec, Robot};

#[test]
fn test_robot_round_trip() {
    let input = r#"#VRML_SIM R2025a utf8
Robot {
  translation 0 1 0
  children [
    Solid {
      translation 1 0 0
      name "child_solid"
      boundingObject Box {
        size 0.1 0.1 0.1
      }
      physics Physics {
        density -1
        mass 1
        centerOfMass 1 2 3
        inertiaMatrix [ 1 0 1 ]
      }
    }
    DEF MyShape Shape {
      castShadows FALSE
    }
    USE MyShape
  ]
  name "my_robot"
  supervisor TRUE
}
"#;

    // Deserialize
    let codec = R2025aCodec::new();
    let robot: Robot = codec.decode(input).unwrap();

    // Verify fields
    assert_eq!(robot.translation.as_ref().unwrap().unwrap_value().y, 1.0);
    assert_eq!(robot.name, "my_robot");
    assert!(*robot.supervisor.as_ref().unwrap().unwrap_value());
    assert_eq!(robot.children.as_ref().unwrap().unwrap_value().len(), 3);

    let children = robot.children.as_ref().unwrap();
    if let Node::Solid(s) = &children.unwrap_value()[0] {
        assert_eq!(s.name, "child_solid");
        assert_eq!(s.translation.as_ref().unwrap().unwrap_value().x, 1.0);

        // Verify boundingObject
        if let Some(bo) = &s.bounding_object {
            if let Node::Box(b) = &**bo.unwrap_value() {
                assert_eq!(b.size.as_ref().unwrap().unwrap_value().x, 0.1);
            } else {
                panic!("Expected Box");
            }
        } else {
            panic!("Expected boundingObject");
        }

        // Verify physics
        if let Some(ph) = &s.physics {
            if let Node::Physics(p) = &**ph.unwrap_value() {
                assert_eq!(*p.mass.as_ref().unwrap().unwrap_value(), 1.0);
                // Accessing Option fields safely
                if let Some(com) = &p.center_of_mass {
                    assert_eq!(com.unwrap_value().x, 1.0);
                } else {
                    panic!("Expected centerOfMass");
                }
                if let Some(im) = &p.inertia_matrix {
                    assert_eq!(im.unwrap_value().len(), 3);
                } else {
                    panic!("Expected inertiaMatrix");
                }
            } else {
                panic!("Expected Physics");
            }
        } else {
            panic!("Expected physics");
        }
    } else {
        panic!("Expected Solid");
    }

    if let Node::Def(name, node) = &children.unwrap_value()[1] {
        assert_eq!(name, "MyShape");
        if let Node::Shape(s) = node.as_ref() {
            assert!(!(*s.cast_shadows.as_ref().unwrap().unwrap_value()));
        } else {
            panic!("Expected Shape inside Def");
        }
    } else {
        panic!("Expected Def");
    }

    if let Node::Use(u) = &children.unwrap_value()[2] {
        assert_eq!(u, "MyShape");
    } else {
        panic!("Expected Use");
    }

    // Serialize
    let output = codec.encode(&robot).unwrap();
    println!("Serialized: {}", output);

    // Verify deserialize back
    let output_with_header = format!("#VRML_SIM R2025a utf8\n{}", output);
    let robot2: Robot = codec.decode(&output_with_header).unwrap();
    assert_eq!(robot2.translation.as_ref().unwrap().unwrap_value().y, 1.0);
    let children2 = robot2.children.as_ref().unwrap();
    if let Node::Def(name, _) = &children2.unwrap_value()[1] {
        assert_eq!(name, "MyShape");
    } else {
        panic!("Expected Def in round trip");
    }
    if let Node::Use(u) = &children2.unwrap_value()[2] {
        assert_eq!(u, "MyShape");
    } else {
        panic!("Expected Use in round trip");
    }
}

#[test]
fn test_skip_optional_fields() {
    let input = "#VRML_SIM R2025a utf8\nRobot { name \"test\" }";
    let codec = R2025aCodec::new();
    let robot: Robot = codec.decode(input).unwrap();
    let output = codec.encode(&robot).unwrap();
    assert!(!output.contains("physics"));
    assert!(!output.contains("NULL"));
}
