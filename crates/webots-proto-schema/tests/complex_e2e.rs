use webots_proto_schema::r2025a::{Node, R2025aCodec};

#[test]
fn test_complex_robot_e2e() {
    let input = r#"#VRML_SIM R2025a utf8
Robot {
  translation 0 0.5 0
  rotation 0 1 0 1.57
  children [
    HingeJoint {
      jointParameters HingeJointParameters {
        axis 0 1 0
        anchor 0 0.5 0
      }
      device [
        RotationalMotor {
          name "neck"
        }
        PositionSensor {
          name "neck_sensor"
        }
      ]
      endPoint Solid {
        name "head"
        translation 0 0.1 0
        children [
          Camera {
            name "camera"
            width 320
            height 240
          }
          Lidar {
            name "lidar"
            horizontalResolution 1024
            fieldOfView 3.14
          }
        ]
      }
    }
    DEF BODY Shape {
      appearance PBRAppearance {
        name "body_appearance"
        baseColor 1 0 0
        metalness 0.5
        roughness 0.2
      }
      geometry Box {
        size 0.5 0.5 0.5
      }
    }
  ]
  name "complex_robot"
  controller "my_controller"
  supervisor TRUE
}
"#;

    // Deserialize
    let node: Node = R2025aCodec::new().decode(input).unwrap();

    // Verify
    if let Node::Robot(robot) = &node {
        assert_eq!(robot.name, "complex_robot");
        assert_eq!(
            robot.controller.as_ref().unwrap().unwrap_value(),
            &webots_proto_schema::r2025a::enums::Controller::Specific("my_controller".to_string())
        );
        assert!(*robot.supervisor.as_ref().unwrap().unwrap_value());

        // Check HingeJoint
        let children = robot.children.as_ref().expect("children should exist");
        if let Node::HingeJoint(joint) = &children.unwrap_value()[0] {
            // Check device
            let device = joint.device.as_ref().expect("device should exist");
            assert_eq!(device.unwrap_value().len(), 2);

            // Check endPoint
            if let Some(ep) = &joint.end_point {
                if let Node::Solid(solid) = &**ep.unwrap_value() {
                    assert_eq!(solid.name, "head");
                    let solid_children = solid
                        .children
                        .as_ref()
                        .expect("solid children should exist");
                    assert_eq!(solid_children.unwrap_value().len(), 2);
                } else {
                    panic!("Expected Solid endPoint");
                }
            }
        } else {
            panic!("Expected HingeJoint");
        }
    } else {
        panic!("Expected Robot");
    }

    // Round Trip
    let output = R2025aCodec::new().encode(&node).unwrap();
    let output_with_header = format!("#VRML_SIM R2025a utf8\n{}", output);
    let node2: Node = R2025aCodec::new().decode(&output_with_header).unwrap();

    // Basic verification of round trip
    if let Node::Robot(robot) = &node2 {
        assert_eq!(robot.name, "complex_robot");
    } else {
        panic!("Expected Robot after round trip");
    }
}
