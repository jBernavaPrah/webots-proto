use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `Physics` node specifies the physical properties of a Solid.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/physics?version=R2025a).
    Physics {
    /// Default: 1000.0
    density: SFFloat,
    /// Default: -1.0
    mass: SFFloat,
    /// Default: NULL
    center_of_mass: SFVec3f,
    /// Default: NULL
    inertia_matrix: MFFloat, // [SFFloat; 2] or [SFFloat; 6]
    /// Default: NULL
    damping: Box<Node>,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_new() {
        assert!(Physics::new().density.is_none());
    }
}
