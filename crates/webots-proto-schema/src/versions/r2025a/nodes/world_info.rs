use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Alias for MFNode fields declared as `Vec<Node>`.
pub type MFNode = Vec<Node>;

define_node!(
    /// The `WorldInfo` node contains global parameters of the world.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/worldinfo?version=R2025a).
    WorldInfo {
    /// Default: []
    info: MFString,
    /// Default: ""
    title: SFString,
    /// Default: ""
    window: SFString,
    /// Default: 9.81
    gravity: SFFloat,
    /// Default: 0.00001
    cfm: SFFloat,
    /// Default: 0.2
    erp: SFFloat,
    /// Default: NULL
    physics: SFString,
    /// Default: 32.0
    basic_time_step: SFFloat,
    /// Default: 60.0
    fps: SFFloat,
    /// Default: 1
    optimal_thread_count: SFInt32,
    /// Default: 0.0
    physics_disable_time: SFFloat,
    /// Default: 0.01
    physics_disable_linear_threshold: SFFloat,
    /// Default: 0.01
    physics_disable_angular_threshold: SFFloat,
    /// Default: NULL
    default_damping: Box<Node>,
    /// Default: 0.0
    ink_evaporation: SFFloat,
    /// Default: "ENU"
    coordinate_system: SFString,
    /// Default: "local"
    gps_coordinate_system: SFString,
    /// Default: []
    gps_reference: MFVec3f,
    /// Default: 0.1
    line_scale: SFFloat,
    /// Default: 30.0
    drag_force_scale: SFFloat,
    /// Default: 30.0
    drag_torque_scale: SFFloat,
    /// Default: 0
    random_seed: SFInt32,
    /// Default: []
    contact_properties: MFNode,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_info_new() {
        let world_info_node = WorldInfo::new();
        assert!(world_info_node.gravity.is_none());
        assert!(world_info_node.coordinate_system.is_none());
    }
}
