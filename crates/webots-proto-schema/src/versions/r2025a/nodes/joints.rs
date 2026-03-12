use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

pub type MFNode = Vec<Node>;

define_node!(
    /// The `JointParameters` node specifies the parameters of a joint.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/jointparameters?version=R2025a).
    JointParameters {
    /// Default: 0.0
    position: SFFloat,
    /// Default: 0 0 1
    axis: SFVec3f,
    /// Default: 0.0
    min_stop: SFFloat,
    /// Default: 0.0
    max_stop: SFFloat,
    /// Default: 0.0
    spring_constant: SFFloat,
    /// Default: 0.0
    damping_constant: SFFloat,
    /// Default: 0.0
    static_friction: SFFloat,
});

define_node!(
    /// The `HingeJointParameters` node specifies parameters for a hinge joint.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/hingejointparameters?version=R2025a).
    HingeJointParameters {
    // JointParameters fields
    /// Default: 0.0
    position: SFFloat,
    /// Default: 1 0 0
    axis: SFVec3f,
    /// Default: 0.0
    min_stop: SFFloat,
    /// Default: 0.0
    max_stop: SFFloat,
    /// Default: 0.0
    spring_constant: SFFloat,
    /// Default: 0.0
    damping_constant: SFFloat,
    /// Default: 0.0
    static_friction: SFFloat,

    // HingeJointParameters fields
    /// Default: 0 0 0
    anchor: SFVec3f,
    /// Default: 0.0
    suspension_spring_constant: SFFloat,
    /// Default: 0.0
    suspension_damping_constant: SFFloat,
    /// Default: 1 0 0
    suspension_axis: SFVec3f,
    /// Default: -1.0
    stop_erp: SFFloat,
    /// Default: -1.0
    stop_cfm: SFFloat,
});

define_node!(
    /// The `BallJointParameters` node specifies parameters for a ball joint.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/balljointparameters?version=R2025a).
    BallJointParameters {
    // JointParameters fields
    /// Default: 0.0
    position: SFFloat,
    /// Default: 0 0 1
    axis: SFVec3f,
    /// Default: 0.0
    min_stop: SFFloat,
    /// Default: 0.0
    max_stop: SFFloat,
    /// Default: 0.0
    spring_constant: SFFloat,
    /// Default: 0.0
    damping_constant: SFFloat,
    /// Default: 0.0
    static_friction: SFFloat,

    // BallJointParameters fields
    /// Default: 0 0 0
    anchor: SFVec3f,
});

define_node!(
    /// The `HingeJoint` node models a hinge joint (1 DOF).
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/hingejoint?version=R2025a).
    HingeJoint {
    /// Default: NULL
    joint_parameters: Box<Node>,
    /// Default: NULL
    end_point: Box<Node>,
    /// Default: []
    device: MFNode,
    /// Default: 0.0
    position: SFFloat,
});

define_node!(
    /// The `Hinge2Joint` node models a hinge-2 joint (2 DOF).
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/hinge2joint?version=R2025a).
    Hinge2Joint {
    /// Default: NULL
    joint_parameters: Box<Node>, // HingeJointParameters
    /// Default: NULL
    joint_parameters2: Box<Node>, // JointParameters
    /// Default: NULL
    end_point: Box<Node>,
    /// Default: []
    device: MFNode,
    /// Default: []
    device2: MFNode,
    /// Default: 0.0
    position: SFFloat,
    /// Default: 0.0
    position2: SFFloat,
});

define_node!(
    /// The `BallJoint` node models a ball joint (3 DOF).
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/balljoint?version=R2025a).
    BallJoint {
    /// Default: NULL
    joint_parameters: Box<Node>, // BallJointParameters
    /// Default: NULL
    joint_parameters2: Box<Node>, // JointParameters
    /// Default: NULL
    joint_parameters3: Box<Node>, // JointParameters
    /// Default: NULL
    end_point: Box<Node>,
    /// Default: []
    device: MFNode,
    /// Default: []
    device2: MFNode,
    /// Default: []
    device3: MFNode,
    /// Default: 0.0
    position: SFFloat,
    /// Default: 0.0
    position2: SFFloat,
    /// Default: 0.0
    position3: SFFloat,
});

define_node!(
    /// The `SliderJoint` node models a slider joint (1 DOF translation).
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/sliderjoint?version=R2025a).
    SliderJoint {
    /// Default: NULL
    joint_parameters: Box<Node>,
    /// Default: NULL
    end_point: Box<Node>,
    /// Default: []
    device: MFNode,
    /// Default: 0.0
    position: SFFloat,
});

define_device!(
    /// The `Brake` node models a brake device.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/brake?version=R2025a).
    Brake {});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joints_new() {
        assert!(JointParameters::new().axis.is_none());
        assert!(HingeJointParameters::new().axis.is_none());
        assert_eq!(Brake::new("brake").name, "brake");
    }
}
