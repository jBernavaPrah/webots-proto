use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

// Alias required by macro-generated fields that use MFNode in Solid-derived nodes.
pub type MFNode = Vec<Node>;

define_node!(
    /// The `ContactProperties` node defines contact properties between solids.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/contactproperties?version=R2025a).
    ContactProperties {
    /// Default: "default"
    material1: SFString,
    /// Default: "default"
    material2: SFString,
    /// Default: [1.0]
    coulomb_friction: MFFloat,
    /// Default: 0 0
    friction_rotation: SFVec2f,
    /// Default: 0 0 0
    rolling_friction: SFVec3f,
    /// Default: 0.5
    bounce: SFFloat,
    /// Default: 0.01
    bounce_velocity: SFFloat,
    /// Default: []
    force_dependent_slip: MFFloat,
    /// Default: 0.2
    soft_erp: SFFloat,
    /// Default: 0.001
    soft_cfm: SFFloat,
    /// Default: "sounds/bump.wav"
    bump_sound: SFString,
    /// Default: "sounds/roll.wav"
    roll_sound: SFString,
    /// Default: "sounds/slide.wav"
    slide_sound: SFString,
    /// Default: 10
    max_contact_joints: SFInt32,
});

define_node!(
    /// The `Damping` node can be used to slow down a body.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/damping?version=R2025a).
    Damping {
    /// Default: 0.2
    linear: SFFloat,
    /// Default: 0.2
    angular: SFFloat,
});

define_node!(
    /// The `SolidReference` node can be used to refer to a solid.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/solidreference?version=R2025a).
    SolidReference {
    solid_name: SFString,
});

define_solid!(
    /// The `Fluid` node represents a possibly unbounded fluid volume.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/fluid?version=R2025a).
    Fluid {
    /// Default: 1000.0
    density: SFFloat,
    /// Default: 0.001
    viscosity: SFFloat,
    /// Default: 0 0 0
    stream_velocity: SFVec3f,
});

define_node!(
    /// The `ImmersionProperties` node specifies dynamical interactions with a fluid.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/immersionproperties?version=R2025a).
    ImmersionProperties {
    fluid_name: SFString,
    /// Default: "immersed area"
    reference_area: SFString,
    /// Default: 0 0 0
    drag_force_coefficients: SFVec3f,
    /// Default: 0 0 0
    drag_torque_coefficients: SFVec3f,
    /// Default: 0.0
    viscous_resistance_force_coefficient: SFFloat,
    /// Default: 0.0
    viscous_resistance_torque_coefficient: SFFloat,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_properties_new() {
        assert!(ContactProperties::new().material1.is_none());
        assert!(Damping::new().linear.is_none());
        assert_eq!(Fluid::new("fluid").name, "fluid");
        assert!(ImmersionProperties::new().fluid_name.is_none());
    }
}
