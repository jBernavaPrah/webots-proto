use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `DirectionalLight` node defines a directional light source.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/directionallight?version=R2025a).
    DirectionalLight {
    /// Default: 0.0
    ambient_intensity: SFFloat,
    /// Default: 1 1 1
    color: SFColor,
    /// Default: 1.0
    intensity: SFFloat,
    /// Default: TRUE
    on: SFBool,
    /// Default: FALSE
    cast_shadows: SFBool,
    /// Default: 0 0 -1
    direction: SFVec3f,
});

define_node!(
    /// The `PointLight` node specifies a point light source.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/pointlight?version=R2025a).
    PointLight {
    /// Default: 0.0
    ambient_intensity: SFFloat,
    /// Default: 1 1 1
    color: SFColor,
    /// Default: 1.0
    intensity: SFFloat,
    /// Default: TRUE
    on: SFBool,
    /// Default: FALSE
    cast_shadows: SFBool,
    /// Default: 1 0 0
    attenuation: SFVec3f,
    /// Default: 0 0 0
    location: SFVec3f,
    /// Default: 100.0
    radius: SFFloat,
});

define_node!(
    /// The `SpotLight` node defines a light source that emits light in a cone.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/spotlight?version=R2025a).
    SpotLight {
    /// Default: 0.0
    ambient_intensity: SFFloat,
    /// Default: 1 1 1
    color: SFColor,
    /// Default: 1.0
    intensity: SFFloat,
    /// Default: TRUE
    on: SFBool,
    /// Default: FALSE
    cast_shadows: SFBool,
    /// Default: 1 0 0
    attenuation: SFVec3f,
    /// Default: 1.5708
    beam_width: SFFloat,
    /// Default: 0.785398
    cut_off_angle: SFFloat,
    /// Default: 0 0 -1
    direction: SFVec3f,
    /// Default: 0 0 0
    location: SFVec3f,
    /// Default: 100.0
    radius: SFFloat,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lighting_new() {
        assert!(DirectionalLight::new().direction.is_none());
        assert!(PointLight::new().radius.is_none());
        assert!(SpotLight::new().cut_off_angle.is_none());
    }
}
