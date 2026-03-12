use super::Node;
use crate::types::*;
use crate::versions::r2025a::enums::{ConnectorType, EmitterType, ReceiverType};
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

pub type MFNode = Vec<Node>;

define_solid!(
    /// The `Emitter` node is used to model radio, serial or infra-red emitters.
    ///
    /// An `Emitter` node must be added to the children of a robot or a supervisor.
    /// Please note that an emitter can send data but it cannot receive data.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/emitter?version=R2025a).
    Emitter {
    /// {"radio", "serial", "infra-red"}
    #[serde(rename = "type")]
    #[setters(rename = "with_type")]
    r#type: EmitterType,
    /// {-1} or [0, inf)
    range: SFFloat,
    /// {-1} or [0, inf)
    max_range: SFFloat,
    /// {-1} or [0, 2*pi]
    aperture: SFFloat,
    /// [0, inf)
    channel: SFInt32,
    /// {-1} or [0, inf)
    baud_rate: SFInt32,
    /// [1, inf)
    byte_size: SFInt32,
    /// {-1} or [0, inf)
    buffer_size: SFInt32,
    /// []
    allowed_channels: MFInt32,
});

define_solid!(
    /// The `Receiver` node is used to model radio, serial or infra-red receivers.
    ///
    /// Please note that a `Receiver` can receive data but it cannot send it.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/receiver?version=R2025a).
    Receiver {
    /// {"radio", "serial", "infra-red"}
    #[serde(rename = "type")]
    #[setters(rename = "with_type")]
    r#type: ReceiverType,
    /// {-1} or [0, 2*pi]
    aperture: SFFloat,
    /// [0, inf)
    channel: SFInt32,
    /// {-1} or [0, inf)
    baud_rate: SFInt32,
    /// [1, inf)
    byte_size: SFInt32,
    /// {-1} or [0, inf)
    buffer_size: SFInt32,
    /// [0, inf)
    signal_strength_noise: SFFloat,
    /// [0, inf)
    direction_noise: SFFloat,
    /// []
    allowed_channels: MFInt32,
});

define_solid!(
    /// `Connector` nodes are used to simulate mechanical docking systems.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/connector?version=R2025a).
    Connector {
    /// {"symmetric", "active", "passive"}
    #[serde(rename = "type")]
    #[setters(rename = "with_type")]
    r#type: ConnectorType,
    /// {TRUE, FALSE}
    is_locked: SFBool,
    /// {TRUE, FALSE}
    auto_lock: SFBool,
    /// {TRUE, FALSE}
    unilateral_lock: SFBool,
    /// {TRUE, FALSE}
    unilateral_unlock: SFBool,
    /// [0, inf)
    distance_tolerance: SFFloat,
    /// [0, pi]
    axis_tolerance: SFFloat,
    /// [0, pi]
    rotation_tolerance: SFFloat,
    /// [0, inf)
    number_of_rotations: SFInt32,
    /// {TRUE, FALSE}
    snap: SFBool,
    /// {-1} or [0, inf)
    tensile_strength: SFFloat,
    /// {-1} or [0, inf)
    shear_strength: SFFloat,
});

define_solid!(
    /// The `LED` node is used to model a light emitting diode.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/led?version=R2025a).
    LED {
    /// []
    color: MFColor,
    /// {TRUE, FALSE}
    gradual: SFBool,
});

define_solid!(
    /// The `Display` node allows handling a 2D pixel array.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/display?version=R2025a).
    Display {
    /// [1, inf)
    width: SFInt32,
    /// [1, inf)
    height: SFInt32,
});

define_solid!(
    /// The `Pen` node models a pen attached to a mobile robot.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/pen?version=R2025a).
    Pen {
    /// any color
    ink_color: SFColor,
    /// [0, 1]
    ink_density: SFFloat,
    /// [0, inf)
    lead_size: SFFloat,
    /// [0, inf)
    max_distance: SFFloat,
    /// {TRUE, FALSE}
    write: SFBool,
});

define_solid!(
    /// The `Speaker` node represents a loudspeaker device.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/speaker?version=R2025a).
    Speaker {});

define_solid!(
    /// The `Charger` node is used to model a battery charger.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/charger?version=R2025a).
    Charger {
    /// []
    battery: MFFloat,
    /// [0, inf)
    radius: SFFloat,
    /// any color
    emissive_color: SFColor,
    /// {TRUE, FALSE}
    gradual: SFBool,
});

define_solid!(
    /// The `VacuumGripper` node is used to simulate vacuum suction links.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/vacuumgripper?version=R2025a).
    VacuumGripper {
    /// {TRUE, FALSE}
    is_on: SFBool,
    /// {-1} or [0, inf)
    tensile_strength: SFFloat,
    /// {-1} or [0, inf)
    shear_strength: SFFloat,
    /// [1, inf)
    contact_points: SFInt32,
});

define_device!(
    /// The `Skin` node can be used to simulate soft mesh animation.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/skin?version=R2025a).
    Skin {
    // Skin fields (Transform + others)
    /// any vector
    translation: SFVec3f,
    /// unit axis, (-inf, inf) angle
    rotation: SFRotation,
    /// any vector
    scale: SFVec3f,

    /// any string
    model_url: SFString,
    /// []
    appearance: MFNode,
    /// []
    bones: MFNode,
    /// {TRUE, FALSE}
    cast_shadows: SFBool,
    /// [0, inf)
    translation_step: SFFloat,
    /// [0, inf)
    rotation_step: SFFloat,
});

define_solid!(
    /// The `Track` node defines a track object for conveyor belts or tank robots.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/track?version=R2025a).
    Track {
        /// []
        device: MFNode,
        /// any vector
        texture_animation: SFVec2f,
        /// {Shape, Group, Transform, PROTO}
        animated_geometry: Box<Node>,
        /// [0, inf)
        geometries_count: SFInt32,
    }
);

crate::define_node!(
    /// The `TrackWheel` node helps setup a wheel of a track system.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/trackwheel?version=R2025a).
    TrackWheel {
    /// any vector
    position: SFVec2f,
    /// (0, inf)
    radius: SFFloat,
    /// {TRUE, FALSE}
    inner: SFBool,
    /// []
    children: MFNode,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emitter_new() {
        let emitter = Emitter::new("myemitter");
        assert_eq!(emitter.name, "myemitter");
        assert!(emitter.r#type.is_none());
    }

    #[test]
    fn test_led_new() {
        let led = LED::new("myled");
        assert_eq!(led.name, "myled");
        assert!(led.color.is_none());
    }
}
