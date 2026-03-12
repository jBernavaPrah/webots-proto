use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Alias for MFNode fields declared as `Vec<Node>`.
pub type MFNode = Vec<Node>;

/// The `Group` node is a grouping node that contains children.
///
/// See [Webots Reference](https://cyberbotics.com/doc/reference/group?version=R2025a).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, new, Setters)]
#[serde(rename_all = "camelCase")]
#[setters(prefix = "with_", strip_option, into)]
pub struct Group {
    /// Default: []
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub children: Option<crate::types::ProtoField<MFNode>>,
}

impl crate::proto::schema::WebotsNode for Group {
    fn node_name() -> &'static str {
        "Group"
    }

    fn all_fields() -> &'static [(&'static str, crate::proto::ast::FieldType)] {
        &[("children", crate::proto::ast::FieldType::MFNode)]
    }
}

define_node!(
    /// The `Pose` node is a grouping node that defines a coordinate system for its children.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/pose?version=R2025a).
    Pose {
    /// Default: 0 0 0
    translation: SFVec3f,
    /// Default: 0 0 1 0
    rotation: SFRotation,
    /// Default: []
    children: MFNode,
});

define_node!(
    /// The `Billboard` node is a grouping node that auto-rotates to face the camera.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/billboard?version=R2025a).
    Billboard {
    /// Default: []
    children: MFNode,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_new() {
        let group_node = Group::new();
        assert!(group_node.children.is_none());
    }

    #[test]
    fn test_pose_new() {
        let pose_node = Pose::new();
        assert!(pose_node.rotation.is_none());
    }

    #[test]
    fn test_billboard_new() {
        let billboard_node = Billboard::new();
        assert!(billboard_node.children.is_none());
    }
}
