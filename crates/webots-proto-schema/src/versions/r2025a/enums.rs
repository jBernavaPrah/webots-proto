use crate::proto::ast::FieldType;
use crate::proto::schema::WebotsFieldType;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Defines the controller type for a Robot.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Controller {
    None,
    #[default]
    Generic,
    Extern,
    Specific(String),
}

impl Serialize for Controller {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Controller::None => serializer.serialize_str("<none>"),
            Controller::Generic => serializer.serialize_str("<generic>"),
            Controller::Extern => serializer.serialize_str("<extern>"),
            Controller::Specific(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for Controller {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "<none>" => Ok(Controller::None),
            "<generic>" => Ok(Controller::Generic),
            "<extern>" => Ok(Controller::Extern),
            _ => Ok(Controller::Specific(s)),
        }
    }
}

impl WebotsFieldType for Controller {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum LidarType {
    #[serde(rename = "fixed")]
    #[default]
    Fixed,
    #[serde(rename = "rotating")]
    Rotating,
}

impl WebotsFieldType for LidarType {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Projection {
    #[serde(rename = "planar")]
    Planar,
    #[serde(rename = "cylindrical")]
    #[default]
    Cylindrical,
    #[serde(rename = "spherical")]
    Spherical,
}

impl WebotsFieldType for Projection {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum GpsType {
    #[serde(rename = "satellite")]
    #[default]
    Satellite,
    #[serde(rename = "laser")]
    Laser,
    #[serde(rename = "ground_truth")]
    GroundTruth,
}

impl WebotsFieldType for GpsType {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum DistanceSensorType {
    #[serde(rename = "generic")]
    #[default]
    Generic,
    #[serde(rename = "infra-red")]
    InfraRed,
    #[serde(rename = "sonar")]
    Sonar,
    #[serde(rename = "laser")]
    Laser,
}

impl WebotsFieldType for DistanceSensorType {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TouchSensorType {
    #[serde(rename = "bumper")]
    #[default]
    Bumper,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "force-3d")]
    Force3d,
}

impl WebotsFieldType for TouchSensorType {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum CoordinateSystem {
    #[serde(rename = "ENU")]
    #[default]
    Enu,
    #[serde(rename = "NUE")]
    Nue,
    #[serde(rename = "EUN")]
    Eun,
}

impl WebotsFieldType for CoordinateSystem {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum FogType {
    #[serde(rename = "LINEAR")]
    #[default]
    Linear,
    #[serde(rename = "EXPONENTIAL")]
    Exponential,
    #[serde(rename = "EXPONENTIAL2")]
    Exponential2,
}

impl WebotsFieldType for FogType {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

// Implement From<> conversions for ease of use
impl From<&str> for Controller {
    fn from(s: &str) -> Self {
        match s {
            "<none>" => Controller::None,
            "<generic>" => Controller::Generic,
            "<extern>" => Controller::Extern,
            _ => Controller::Specific(s.to_string()),
        }
    }
}

impl From<String> for Controller {
    fn from(s: String) -> Self {
        match s.as_str() {
            "<none>" => Controller::None,
            "<generic>" => Controller::Generic,
            "<extern>" => Controller::Extern,
            _ => Controller::Specific(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum EmitterType {
    #[serde(rename = "radio")]
    #[default]
    Radio,
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "infra-red")]
    InfraRed,
}

impl WebotsFieldType for EmitterType {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ReceiverType {
    #[serde(rename = "radio")]
    #[default]
    Radio,
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "infra-red")]
    InfraRed,
}

impl WebotsFieldType for ReceiverType {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ConnectorType {
    #[serde(rename = "symmetric")]
    #[default]
    Symmetric,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "passive")]
    Passive,
}

impl WebotsFieldType for ConnectorType {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}
