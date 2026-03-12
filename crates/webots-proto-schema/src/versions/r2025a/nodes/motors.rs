use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

pub type MFNode = Vec<Node>;

define_motor!(
    /// Abstract Motor node containing common fields.
    Motor {});

define_motor!(
    /// The `RotationalMotor` node is used to drive a rotational joint.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/rotationalmotor?version=R2025a).
    RotationalMotor {
    // RotationalMotor specific
    /// Default: 10
    max_torque: SFFloat,
});

impl RotationalMotor {
    pub fn default_sound() -> String {
        "https://raw.githubusercontent.com/cyberbotics/webots/{{ webots.version.major }}/projects/default/worlds/sounds/rotational_motor.wav".to_string()
    }
}

define_motor!(
    /// The `LinearMotor` node is used to drive a linear joint.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/linearmotor?version=R2025a).
    LinearMotor {
    // LinearMotor specific
    /// Default: 10
    max_force: SFFloat,
});

impl LinearMotor {
    pub fn default_sound() -> String {
        "https://raw.githubusercontent.com/cyberbotics/webots/{{ webots.version.major }}/projects/default/worlds/sounds/linear_motor.wav".to_string()
    }
}

define_node!(
    /// The `Muscle` node displays the contraction of an artificial muscle.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/muscle?version=R2025a).
    Muscle {
    /// Default: 0.01
    volume: SFFloat,
    /// Default: 0 0 0
    start_offset: SFVec3f,
    /// Default: 0 0 0
    end_offset: SFVec3f,
    /// Default: []
    color: MFColor,
    /// Default: TRUE
    cast_shadows: SFBool,
    /// Default: TRUE
    visible: SFBool,
});

define_solid!(
    /// The `Propeller` node models a propeller.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/propeller?version=R2025a).
    Propeller {
    /// Default: 1 0 0
    shaft_axis: SFVec3f,
    /// Default: 0 0 0
    center_of_thrust: SFVec3f,
    /// Default: 1 0
    thrust_constants: SFVec2f,
    /// Default: 1 0
    torque_constants: SFVec2f,
    /// Default: 75.4
    fast_helix_threshold: SFFloat,
    /// Default: NULL
    device: Box<Node>,
    /// Default: NULL
    fast_helix: Box<Node>,
    /// Default: NULL
    slow_helix: Box<Node>,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motors_new() {
        let rot_motor = RotationalMotor::new("motor");
        assert_eq!(rot_motor.name, "motor");
        assert!(rot_motor.acceleration.is_none());
        assert!(rot_motor.max_torque.is_none());

        let lin_motor = LinearMotor::new("linmotor");
        assert_eq!(lin_motor.name, "linmotor");
        assert!(lin_motor.acceleration.is_none());
        assert!(lin_motor.max_force.is_none());
    }

    #[test]
    fn test_default_sounds() {
        assert!(RotationalMotor::default_sound().contains("rotational_motor.wav"));
        assert!(LinearMotor::default_sound().contains("linear_motor.wav"));
    }
}
