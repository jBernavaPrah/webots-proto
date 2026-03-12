use crate::proto::ast::FieldType;
use crate::proto::schema::WebotsFieldType;
use serde::{Deserialize, Serialize};

pub mod appearance;
pub mod cad_shape;
pub mod devices;
pub mod environment;
pub mod geometry;
pub mod group_pose;
pub mod joints;
pub mod lighting;
pub mod mesh;
pub mod motors;
pub mod optics;
pub mod physics;
pub mod physics_properties;
pub mod rectangle_arena;
pub mod rendering;
pub mod robot;
pub mod sensors;
pub mod shape;
pub mod slot;
pub mod solid;
pub mod transform;
pub mod viewpoint;
pub mod world_info;

pub use appearance::{Appearance, ImageTexture, Material, PBRAppearance, TextureTransform};
pub use cad_shape::CadShape;
pub use devices::{
    Charger, Connector, Display, Emitter, LED, Pen, Receiver, Skin, Speaker, Track, TrackWheel,
    VacuumGripper,
};
pub use environment::{Background, Fog};
pub use geometry::{
    BoxNode, Capsule, Cone, Cylinder, ElevationGrid, IndexedFaceSet, IndexedLineSet, Plane,
    PointSet, Sphere,
};
pub use group_pose::{Billboard, Group, Pose};
pub use joints::{
    BallJoint, BallJointParameters, Brake, Hinge2Joint, HingeJoint, HingeJointParameters,
    JointParameters, SliderJoint,
};
pub use lighting::{DirectionalLight, PointLight, SpotLight};
pub use mesh::Mesh;
pub use motors::{LinearMotor, Muscle, Propeller, RotationalMotor};
pub use optics::{Focus, Lens, LensFlare, Recognition, Zoom};
pub use physics::Physics;
pub use physics_properties::{
    ContactProperties, Damping, Fluid, ImmersionProperties, SolidReference,
};
pub use rectangle_arena::RectangleArena;
pub use rendering::{Color, Coordinate, Normal, TextureCoordinate};
pub use robot::Robot;
pub use sensors::{
    Accelerometer, Altimeter, Camera, Compass, DistanceSensor, GPS, Gyro, InertialUnit, Lidar,
    LightSensor, PositionSensor, Radar, RangeFinder, TouchSensor,
};
pub use shape::Shape;
pub use slot::Slot;
pub use solid::Solid;
pub use transform::Transform;
pub use viewpoint::Viewpoint;
pub use world_info::WorldInfo;

/// Represents any Webots node.
///
/// This enum is the core type for deserializing Webots worlds or PROTOs where the specific node type
/// might vary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Node {
    WorldInfo(WorldInfo),
    Viewpoint(Viewpoint),
    Solid(Solid),
    Shape(Shape),
    Robot(Robot),
    RectangleArena(RectangleArena),
    Physics(Physics),
    Box(BoxNode),
    Sphere(Sphere),
    Capsule(Capsule),
    Cylinder(Cylinder),
    Plane(Plane),
    Group(Group),
    Pose(Pose),
    HingeJoint(HingeJoint),
    SliderJoint(SliderJoint),
    JointParameters(JointParameters),
    HingeJointParameters(HingeJointParameters),
    RotationalMotor(RotationalMotor),
    LinearMotor(LinearMotor),
    PositionSensor(PositionSensor),
    InertialUnit(InertialUnit),
    Accelerometer(Accelerometer),
    Gyro(Gyro),
    Compass(Compass),
    GPS(GPS),
    DistanceSensor(DistanceSensor),
    Camera(Camera),
    Lidar(Lidar),
    CadShape(CadShape),
    Mesh(Mesh),
    Background(Background),
    Fog(Fog),
    DirectionalLight(DirectionalLight),
    PointLight(PointLight),
    SpotLight(SpotLight),
    Appearance(Appearance),
    Material(Material),
    PBRAppearance(PBRAppearance),
    ImageTexture(ImageTexture),
    TextureTransform(TextureTransform),
    TextureCoordinate(TextureCoordinate),
    Color(Color),
    Coordinate(Coordinate),
    Normal(Normal),
    Cone(Cone),
    ElevationGrid(ElevationGrid),
    IndexedFaceSet(IndexedFaceSet),
    IndexedLineSet(IndexedLineSet),
    PointSet(PointSet),
    ContactProperties(ContactProperties),
    Damping(Damping),
    SolidReference(SolidReference),
    Fluid(Fluid),
    BallJoint(BallJoint),
    Hinge2Joint(Hinge2Joint),
    BallJointParameters(BallJointParameters),
    Brake(Brake),
    Altimeter(Altimeter),
    LightSensor(LightSensor),
    RangeFinder(RangeFinder),
    Radar(Radar),
    TouchSensor(TouchSensor),
    Emitter(Emitter),
    Receiver(Receiver),
    Connector(Connector),
    LED(LED),
    Display(Display),
    Pen(Pen),
    Speaker(Speaker),
    Charger(Charger),
    Slot(Slot),
    Transform(Transform),
    Billboard(Billboard),
    ImmersionProperties(ImmersionProperties),
    Focus(Focus),
    Lens(Lens),
    LensFlare(LensFlare),
    Recognition(Recognition),
    Zoom(Zoom),
    Muscle(Muscle),
    Propeller(Propeller),
    Skin(Skin),
    Track(Track),
    TrackWheel(TrackWheel),
    VacuumGripper(VacuumGripper),
    /// Represents a USE reference to a previously DEF'd node.
    Use(String),
    /// Represents a DEF definition, associating a name with a node instance.
    Def(String, Box<Node>),
}

impl WebotsFieldType for Node {
    const FIELD_TYPE: FieldType = FieldType::SFNode;
}

impl WebotsFieldType for Box<Node> {
    const FIELD_TYPE: FieldType = FieldType::SFNode;
}

impl WebotsFieldType for Vec<Node> {
    const FIELD_TYPE: FieldType = FieldType::MFNode;
}
