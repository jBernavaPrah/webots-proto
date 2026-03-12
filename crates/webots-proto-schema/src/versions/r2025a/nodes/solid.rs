use super::Node;

use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

pub type MFNode = Vec<Node>;

define_solid!(
    /// The `Solid` node represents a rigid body.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/solid?version=R2025a).
    Solid {});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SFVec3f;

    #[test]
    fn test_solid_new() {
        let solid = Solid::new("solid");
        assert_eq!(solid.name, "solid");
        assert!(solid.rotation.is_none());
        assert!(solid.children.is_none());
    }

    #[test]
    fn test_solid_setters() {
        let solid = Solid::new("mysolid").with_translation(SFVec3f {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });

        assert_eq!(solid.name, "mysolid");
        assert_eq!(solid.translation.unwrap().unwrap_value().y, 2.0);
    }
}
