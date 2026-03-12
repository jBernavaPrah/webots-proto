use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

use crate::versions::r2025a::enums::Controller;

define_solid!(
    /// The `Robot` node models a robot.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/robot?version=R2025a).
    Robot {
    /// Default: "\<generic\>"
    /// Possible values: `"<none>"`, `"<generic>"`, `"<extern>"`, or a specific controller name.
    controller: Controller,
    /// Default: []
    controller_args: MFString,
    /// Default: ""
    custom_data: SFString,
    /// Default: FALSE
    supervisor: SFBool,
    /// Default: TRUE
    synchronization: SFBool,
    /// Default: []
    /// [current energy, max energy, initial energy]
    battery: MFFloat,
    /// Default: 10.0
    /// [0, inf)
    cpu_consumption: SFFloat,
    /// Default: FALSE
    self_collision: SFBool,
    /// Default: "\<generic\>"
    window: SFString,
    /// Default: "\<none\>"
    remote_control: SFString,
});

pub type MFNode = Vec<Node>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_new() {
        let robot = Robot::new("robot");
        assert_eq!(robot.name, "robot");
        assert!(robot.controller.is_none());
        assert!(robot.translation.is_none());
    }
}
