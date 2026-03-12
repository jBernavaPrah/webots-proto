use crate::proto::ast::FieldType;
use crate::proto::schema::WebotsFieldType;
use derive_new::new;
use derive_setters::Setters;
use serde::de::{self, Deserializer, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

pub type SFBool = bool;
pub type SFInt32 = i32;
pub type SFFloat = f64;
pub type SFString = String;

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize, new, Setters, Default)]
#[setters(prefix = "with_", strip_option, into)]
#[serde(from = "(f64, f64)", into = "(f64, f64)")]
pub struct SFVec2f {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for SFVec2f {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}
impl From<SFVec2f> for (f64, f64) {
    fn from(v: SFVec2f) -> Self {
        (v.x, v.y)
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize, new, Setters, Default)]
#[setters(prefix = "with_", strip_option, into)]
#[serde(from = "(f64, f64, f64)", into = "(f64, f64, f64)")]
pub struct SFVec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<(f64, f64, f64)> for SFVec3f {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}
impl From<SFVec3f> for (f64, f64, f64) {
    fn from(v: SFVec3f) -> Self {
        (v.x, v.y, v.z)
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize, new, Setters, Default)]
#[setters(prefix = "with_", strip_option, into)]
#[serde(from = "(f64, f64, f64, f64)", into = "(f64, f64, f64, f64)")]
pub struct SFRotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub angle: f64,
}

impl From<(f64, f64, f64, f64)> for SFRotation {
    fn from((x, y, z, angle): (f64, f64, f64, f64)) -> Self {
        Self { x, y, z, angle }
    }
}
impl From<SFRotation> for (f64, f64, f64, f64) {
    fn from(v: SFRotation) -> Self {
        (v.x, v.y, v.z, v.angle)
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize, new, Setters, Default)]
#[setters(prefix = "with_", strip_option, into)]
#[serde(from = "(f64, f64, f64)", into = "(f64, f64, f64)")]
pub struct SFColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl From<(f64, f64, f64)> for SFColor {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        Self { r, g, b }
    }
}
impl From<SFColor> for (f64, f64, f64) {
    fn from(v: SFColor) -> Self {
        (v.r, v.g, v.b)
    }
}

pub type MFBool = Vec<SFBool>;
pub type MFInt32 = Vec<SFInt32>;
pub type MFFloat = Vec<SFFloat>;
pub type MFString = Vec<SFString>;
pub type MFVec2f = Vec<SFVec2f>;
pub type MFVec3f = Vec<SFVec3f>;
pub type MFColor = Vec<SFColor>;
pub type MFRotation = Vec<SFRotation>;

#[derive(Debug, Clone, PartialEq)]
pub enum ProtoField<T> {
    Value(T),
    Is(String),
}

impl<T: Default> Default for ProtoField<T> {
    fn default() -> Self {
        ProtoField::Value(T::default())
    }
}

impl<T> From<T> for ProtoField<T> {
    fn from(v: T) -> Self {
        ProtoField::Value(v)
    }
}

impl<'a> From<&'a str> for ProtoField<String> {
    fn from(s: &'a str) -> Self {
        ProtoField::Value(s.to_string())
    }
}

impl<T> ProtoField<T> {
    pub fn value(&self) -> Option<&T> {
        match self {
            ProtoField::Value(v) => Some(v),
            ProtoField::Is(_) => None,
        }
    }

    pub fn unwrap_value(&self) -> &T {
        match self {
            ProtoField::Value(v) => v,
            ProtoField::Is(s) => panic!("Expected Value, found IS {}", s),
        }
    }
}

impl<T: PartialEq> PartialEq<T> for ProtoField<T> {
    fn eq(&self, other: &T) -> bool {
        match self {
            ProtoField::Value(v) => v == other,
            ProtoField::Is(_) => false,
        }
    }
}

impl PartialEq<&str> for ProtoField<String> {
    fn eq(&self, other: &&str) -> bool {
        match self {
            ProtoField::Value(v) => v == *other,
            ProtoField::Is(_) => false,
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for ProtoField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const VARIANTS: &[&str] = &["Value", "Is"];
        deserializer.deserialize_enum("ProtoField", VARIANTS, ProtoFieldVisitor(PhantomData))
    }
}

struct ProtoFieldVisitor<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for ProtoFieldVisitor<T> {
    type Value = ProtoField<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a value or IS reference")
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let (variant, variant_access) = data.variant::<String>()?;
        match variant.as_str() {
            "Is" => {
                let value = variant_access.newtype_variant()?;
                Ok(ProtoField::Is(value))
            }
            "Value" => {
                let value = variant_access.newtype_variant()?;
                Ok(ProtoField::Value(value))
            }
            _ => Err(de::Error::unknown_variant(&variant, &["Value", "Is"])),
        }
    }
}

impl<T: Serialize> Serialize for ProtoField<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ProtoField::Value(v) => {
                serializer.serialize_newtype_variant("ProtoField", 0, "Value", v)
            }
            ProtoField::Is(s) => serializer.serialize_newtype_variant("ProtoField", 1, "Is", s),
        }
    }
}

impl WebotsFieldType for bool {
    const FIELD_TYPE: FieldType = FieldType::SFBool;
}

impl WebotsFieldType for i32 {
    const FIELD_TYPE: FieldType = FieldType::SFInt32;
}

impl WebotsFieldType for f64 {
    const FIELD_TYPE: FieldType = FieldType::SFFloat;
}

impl WebotsFieldType for String {
    const FIELD_TYPE: FieldType = FieldType::SFString;
}

impl WebotsFieldType for SFVec2f {
    const FIELD_TYPE: FieldType = FieldType::SFVec2f;
}

impl WebotsFieldType for SFVec3f {
    const FIELD_TYPE: FieldType = FieldType::SFVec3f;
}

impl WebotsFieldType for SFRotation {
    const FIELD_TYPE: FieldType = FieldType::SFRotation;
}

impl WebotsFieldType for SFColor {
    const FIELD_TYPE: FieldType = FieldType::SFColor;
}

impl WebotsFieldType for Vec<bool> {
    const FIELD_TYPE: FieldType = FieldType::MFBool;
}

impl WebotsFieldType for Vec<i32> {
    const FIELD_TYPE: FieldType = FieldType::MFInt32;
}

impl WebotsFieldType for Vec<f64> {
    const FIELD_TYPE: FieldType = FieldType::MFFloat;
}

impl WebotsFieldType for Vec<String> {
    const FIELD_TYPE: FieldType = FieldType::MFString;
}

impl WebotsFieldType for Vec<SFVec2f> {
    const FIELD_TYPE: FieldType = FieldType::MFVec2f;
}

impl WebotsFieldType for Vec<SFVec3f> {
    const FIELD_TYPE: FieldType = FieldType::MFVec3f;
}

impl WebotsFieldType for Vec<SFRotation> {
    const FIELD_TYPE: FieldType = FieldType::MFRotation;
}

impl WebotsFieldType for Vec<SFColor> {
    const FIELD_TYPE: FieldType = FieldType::MFColor;
}
