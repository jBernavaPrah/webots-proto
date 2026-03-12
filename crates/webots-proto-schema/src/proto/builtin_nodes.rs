//! Built-in Webots node schemas.

use crate::proto::schema::WebotsNode;
use crate::proto::validation::{NodeSchema, SchemaField};
use crate::versions::r2025a::nodes::*;

pub(crate) fn get_builtin_schema(type_name: &str) -> Option<NodeSchema> {
    type SchemaBuilder = fn() -> NodeSchema;
    type BuiltinSchemaEntry = (&'static str, SchemaBuilder);
    let schemas: [BuiltinSchemaEntry; 86] = [
        // --- Core ---
        ("WorldInfo", || schema::<WorldInfo>()),
        ("Viewpoint", || schema::<Viewpoint>()),
        // --- Grouping and Transform ---
        ("Group", || schema::<Group>()),
        ("Pose", || schema::<Pose>()),
        ("Transform", || schema::<Transform>()),
        ("Billboard", || schema::<Billboard>()),
        // --- Shapes and Appearance ---
        ("Shape", || schema::<Shape>()),
        ("Appearance", || schema::<Appearance>()),
        ("PBRAppearance", || schema::<PBRAppearance>()),
        ("Material", || schema::<Material>()),
        ("ImageTexture", || schema::<ImageTexture>()),
        ("TextureTransform", || schema::<TextureTransform>()),
        // --- Geometry ---
        ("Box", || schema::<BoxNode>()),
        ("Sphere", || schema::<Sphere>()),
        ("Capsule", || schema::<Capsule>()),
        ("Cylinder", || schema::<Cylinder>()),
        ("Plane", || schema::<Plane>()),
        ("Cone", || schema::<Cone>()),
        ("ElevationGrid", || schema::<ElevationGrid>()),
        ("IndexedFaceSet", || schema::<IndexedFaceSet>()),
        ("IndexedLineSet", || schema::<IndexedLineSet>()),
        ("PointSet", || schema::<PointSet>()),
        ("Mesh", || schema::<Mesh>()),
        ("CadShape", || schema::<CadShape>()),
        // --- Robots and Solids ---
        ("Robot", || schema::<Robot>()),
        ("Solid", || schema::<Solid>()),
        ("Fluid", || schema::<Fluid>()),
        // --- Sensors ---
        ("Accelerometer", || schema::<Accelerometer>()),
        ("Altimeter", || schema::<Altimeter>()),
        ("Camera", || schema::<Camera>()),
        ("Compass", || schema::<Compass>()),
        ("DistanceSensor", || schema::<DistanceSensor>()),
        ("GPS", || schema::<GPS>()),
        ("Gyro", || schema::<Gyro>()),
        ("InertialUnit", || schema::<InertialUnit>()),
        ("Lidar", || schema::<Lidar>()),
        ("LightSensor", || schema::<LightSensor>()),
        ("PositionSensor", || schema::<PositionSensor>()),
        ("Radar", || schema::<Radar>()),
        ("RangeFinder", || schema::<RangeFinder>()),
        ("TouchSensor", || schema::<TouchSensor>()),
        // --- Motors and Actuators ---
        ("RotationalMotor", || schema::<RotationalMotor>()),
        ("LinearMotor", || schema::<LinearMotor>()),
        ("Brake", || schema::<Brake>()),
        ("Propeller", || schema::<Propeller>()),
        ("Muscle", || schema::<Muscle>()),
        // --- Joints ---
        ("HingeJoint", || schema::<HingeJoint>()),
        ("Hinge2Joint", || schema::<Hinge2Joint>()),
        ("BallJoint", || schema::<BallJoint>()),
        ("SliderJoint", || schema::<SliderJoint>()),
        ("JointParameters", || schema::<JointParameters>()),
        ("HingeJointParameters", || schema::<HingeJointParameters>()),
        ("BallJointParameters", || schema::<BallJointParameters>()),
        // --- Devices ---
        ("LED", || schema::<LED>()),
        ("Emitter", || schema::<Emitter>()),
        ("Receiver", || schema::<Receiver>()),
        ("Connector", || schema::<Connector>()),
        ("Display", || schema::<Display>()),
        ("Pen", || schema::<Pen>()),
        ("Speaker", || schema::<Speaker>()),
        ("Charger", || schema::<Charger>()),
        ("VacuumGripper", || schema::<VacuumGripper>()),
        ("Track", || schema::<Track>()),
        ("TrackWheel", || schema::<TrackWheel>()),
        ("Skin", || schema::<Skin>()),
        // --- Environment ---
        ("Background", || schema::<Background>()),
        ("Fog", || schema::<Fog>()),
        // --- Lights ---
        ("DirectionalLight", || schema::<DirectionalLight>()),
        ("PointLight", || schema::<PointLight>()),
        ("SpotLight", || schema::<SpotLight>()),
        // --- Physics and Properties ---
        ("Physics", || schema::<Physics>()),
        ("Damping", || schema::<Damping>()),
        ("ContactProperties", || schema::<ContactProperties>()),
        ("ImmersionProperties", || schema::<ImmersionProperties>()),
        ("SolidReference", || schema::<SolidReference>()),
        // --- Rendering ---
        ("Color", || schema::<Color>()),
        ("Coordinate", || schema::<Coordinate>()),
        ("Normal", || schema::<Normal>()),
        ("TextureCoordinate", || schema::<TextureCoordinate>()),
        // --- Miscellaneous ---
        ("Slot", || schema::<Slot>()),
        ("Focus", || schema::<Focus>()),
        ("Lens", || schema::<Lens>()),
        ("LensFlare", || schema::<LensFlare>()),
        ("Recognition", || schema::<Recognition>()),
        ("Zoom", || schema::<Zoom>()),
        ("RectangleArena", || schema::<RectangleArena>()),
    ];

    for (name, builder) in schemas {
        if name == type_name {
            return Some(builder());
        }
    }
    None
}

fn schema<T: WebotsNode>() -> NodeSchema {
    NodeSchema {
        name: T::node_name(),
        fields: T::all_fields()
            .iter()
            .map(|(name, field_type)| SchemaField {
                name: to_webots_field_name(name),
                field_type: field_type.clone(),
            })
            .collect(),
    }
}

// Convert Rust snake_case field identifiers to the camelCase names used in PROTO syntax.
fn to_webots_field_name(name: &str) -> String {
    if !name.contains('_') {
        return name.to_string();
    }

    let mut output = String::with_capacity(name.len());
    let mut capitalize_next = false;
    for character in name.chars() {
        if character == '_' {
            capitalize_next = true;
            continue;
        }

        if capitalize_next {
            output.extend(character.to_uppercase());
            capitalize_next = false;
        } else {
            output.push(character);
        }
    }

    output
}
