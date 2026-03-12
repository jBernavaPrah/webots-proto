use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

pub type MFNode = Vec<Node>;

define_node!(
    /// The `Transform` node is a grouping node that defines a coordinate system (with scale).
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/transform?version=R2025a).
    Transform {
    /// Default: 0 0 0
    translation: SFVec3f,
    /// Default: 0 0 1 0
    rotation: SFRotation,
    /// Default: 1 1 1
    scale: SFVec3f,
    /// Default: []
    children: MFNode,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_new() {
        let t = Transform::new();
        assert!(t.scale.is_none());
    }
}
