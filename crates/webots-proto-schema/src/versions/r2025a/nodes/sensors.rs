use super::Node;
use crate::types::*;
use crate::versions::r2025a::enums::{
    DistanceSensorType, GpsType, LidarType, Projection, TouchSensorType,
};
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

pub type MFNode = Vec<Node>;

define_device!(
    /// The `PositionSensor` node monitors a joint position.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/positionsensor?version=R2025a).
    PositionSensor {
    /// [0, inf)
    noise: SFFloat,
    /// {-1, [0, inf)}
    resolution: SFFloat,
});

define_solid!(
    /// The `InertialUnit` node measures the roll, pitch and yaw angles.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/inertialunit?version=R2025a).
    InertialUnit {
    /// {TRUE, FALSE}
    x_axis: SFBool,
    /// {TRUE, FALSE}
    y_axis: SFBool,
    /// {TRUE, FALSE}
    z_axis: SFBool,
    /// {-1, [0, inf)}
    resolution: SFFloat,
    /// [0, inf)
    noise: SFFloat,
});

define_solid!(
    /// The `Accelerometer` node measures acceleration.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/accelerometer?version=R2025a).
    Accelerometer {
    /// lookup table
    lookup_table: MFVec3f,
    /// {TRUE, FALSE}
    x_axis: SFBool,
    /// {TRUE, FALSE}
    y_axis: SFBool,
    /// {TRUE, FALSE}
    z_axis: SFBool,
    /// [0, inf)
    resolution: SFFloat,
});

define_solid!(
    /// The `Gyro` node measures angular velocity.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/gyro?version=R2025a).
    Gyro {
    /// lookup table
    lookup_table: MFVec3f,
    /// {TRUE, FALSE}
    x_axis: SFBool,
    /// {TRUE, FALSE}
    y_axis: SFBool,
    /// {TRUE, FALSE}
    z_axis: SFBool,
    /// [0, inf)
    resolution: SFFloat,
});

define_solid!(
    /// The `Compass` node measures the direction of the north.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/compass?version=R2025a).
    Compass {
    /// lookup table
    lookup_table: MFVec3f,
    /// {TRUE, FALSE}
    x_axis: SFBool,
    /// {TRUE, FALSE}
    y_axis: SFBool,
    /// {TRUE, FALSE}
    z_axis: SFBool,
    /// {-1, [0, inf)}
    resolution: SFFloat,
});

define_solid!(
    /// The `GPS` node determines the absolute position of the robot.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/gps?version=R2025a).
    GPS {
    /// {"satellite", "laser", "ground_truth"}
    #[serde(rename = "type")]
    #[setters(rename = "with_type")]
    r#type: GpsType,
    /// [0, inf)
    accuracy: SFFloat,
    /// [0, inf)
    noise_correlation: SFFloat,
    /// {-1, [0, inf)}
    resolution: SFFloat,
    /// [0, inf)
    speed_noise: SFFloat,
    /// {-1, [0, inf)}
    speed_resolution: SFFloat,
});

define_solid!(
    /// The `DistanceSensor` node measures the distance to obstacles.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/distancesensor?version=R2025a).
    DistanceSensor {
    /// lookup table
    lookup_table: MFVec3f,
    /// {"generic", "infra-red", "sonar", "laser"}
    #[serde(rename = "type")]
    #[setters(rename = "with_type")]
    r#type: DistanceSensorType,
    /// [1, inf)
    number_of_rays: SFInt32,
    /// [0, 2*pi]
    aperture: SFFloat,
    /// [0, inf)
    gaussian_width: SFFloat,
    /// {-1, [0, inf)}
    resolution: SFFloat,
    /// [0, inf)
    red_color_sensitivity: SFFloat,
});

define_solid!(
    /// The `Camera` node models a camera sensor.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/camera?version=R2025a).
    Camera {
        /// [0, pi]
        field_of_view: SFFloat,
        /// [0, inf)
        width: SFInt32,
        /// [0, inf)
        height: SFInt32,
        /// {"planar", "spherical", "cylindrical"}
        projection: Projection,
        /// [0, inf)
        near: SFFloat,
        /// [0, inf)
        far: SFFloat,
        /// [near, inf)
        exposure: SFFloat,
        /// {TRUE, FALSE}
        anti_aliasing: SFBool,
        /// [0, inf)
        ambient_occlusion_radius: SFFloat,
        /// [-1, inf)
        bloom_threshold: SFFloat,
        /// [0, inf)
        motion_blur: SFFloat,
        /// [0, 1]
        noise: SFFloat,
        /// any string
        noise_mask_url: SFString,
        /// {Lens, PROTO}
        lens: Box<Node>,
        /// {Focus, PROTO}
        focus: Box<Node>,
        /// {Zoom, PROTO}
        zoom: Box<Node>,
        /// {Recognition, PROTO}
        recognition: Box<Node>,
        /// {LensFlare, PROTO}
        lens_flare: Box<Node>,
    }
);

define_solid!(
    /// The `Lidar` node models a lidar sensor.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/lidar?version=R2025a).
    Lidar {
        /// [-pi/2, pi/2]
        tilt_angle: SFFloat,
        /// [0, inf)
        horizontal_resolution: SFInt32,
        /// [0, 2*pi]
        field_of_view: SFFloat,
        /// [0, pi]
        vertical_field_of_view: SFFloat,
        /// [0, inf)
        number_of_layers: SFInt32,
        /// [0, inf)
        near: SFFloat,
        /// [near, inf)
        min_range: SFFloat,
        /// [minRange, inf)
        max_range: SFFloat,
        /// {"fixed", "rotating"}
        #[serde(rename = "type")]
        #[setters(rename = "with_type")]
        r#type: LidarType,
        /// {"planar", "cylindrical"}
        projection: Projection,
        /// [0, inf)
        noise: SFFloat,
        /// {-1, [0, inf)}
        resolution: SFFloat,
        /// [minFrequency, maxFrequency]
        default_frequency: SFFloat,
        /// [0, maxFrequency)
        min_frequency: SFFloat,
        /// [minFrequency, inf)
        max_frequency: SFFloat,
        /// {Solid (or derived), PROTO}
        rotating_head: Box<Node>,
    }
);

define_solid!(
    /// The `Altimeter` node models an altimeter sensor.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/altimeter?version=R2025a).
    Altimeter {
    /// [0, inf)
    accuracy: SFFloat,
    /// {-1, [0, inf)}
    resolution: SFFloat,
});

define_solid!(
    /// The `LightSensor` node models a photo-transistor.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/lightsensor?version=R2025a).
    LightSensor {
    /// lookup table
    lookup_table: MFVec3f,
    /// any color
    color_filter: SFColor,
    /// {TRUE, FALSE}
    occlusion: SFBool,
    /// {-1, [0, inf)}
    resolution: SFFloat,
});

define_solid!(
    /// The `RangeFinder` node is used to model a depth camera.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/rangefinder?version=R2025a).
    RangeFinder {
        /// [0, 2*pi]
        field_of_view: SFFloat,
        /// [0, inf)
        width: SFInt32,
        /// [0, inf)
        height: SFInt32,
        /// {"planar", "spherical", "cylindrical"}
        projection: Projection,
        /// [0, inf)
        near: SFFloat,
        /// [near, maxRange]
        min_range: SFFloat,
        /// [minRange, inf)
        max_range: SFFloat,
        /// [0, inf)
        motion_blur: SFFloat,
        /// [0, inf)
        noise: SFFloat,
        /// {-1, [0, inf)}
        resolution: SFFloat,
        /// {Lens, PROTO}
        lens: Box<Node>,
    }
);

define_solid!(
    /// The `Radar` node models a radar sensor.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/radar?version=R2025a).
    Radar {
    /// [0, maxRange)
    min_range: SFFloat,
    /// (minRange, inf)
    max_range: SFFloat,
    /// [0, pi]
    horizontal_field_of_view: SFFloat,
    /// [0, pi]
    vertical_field_of_view: SFFloat,
    /// [0, inf)
    min_absolute_radial_speed: SFFloat,
    /// [0, maxRadialSpeed]
    min_radial_speed: SFFloat,
    /// {-1, [minRadialSpeed, inf)}
    max_radial_speed: SFFloat,
    /// [0, inf)
    cell_distance: SFFloat,
    /// [0, inf)
    cell_speed: SFFloat,
    /// [0, inf)
    range_noise: SFFloat,
    /// [0, inf)
    speed_noise: SFFloat,
    /// [0, inf)
    angular_noise: SFFloat,
    /// (-inf, inf)
    antenna_gain: SFFloat,
    /// [0, inf)
    frequency: SFFloat,
    /// (-inf, inf)
    transmitted_power: SFFloat,
    /// (-inf, inf)
    min_detectable_signal: SFFloat,
    /// {TRUE, FALSE}
    occlusion: SFBool,
});

define_solid!(
    /// The `TouchSensor` node is used to model a bumper or a force sensor.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/touchsensor?version=R2025a).
    TouchSensor {
    /// {"bumper", "force", "force-3d"}
    #[serde(rename = "type")]
    #[setters(rename = "with_type")]
    r#type: TouchSensorType,
    /// lookup table
    lookup_table: MFVec3f,
    /// {-1, [0, inf)}
    resolution: SFFloat,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lidar_new() {
        assert_eq!(Lidar::new("lidar").name, "lidar");
        assert!(Lidar::new("lidar").r#type.is_none());
    }

    #[test]
    fn test_camera_new() {
        assert!(Camera::new("cam").width.is_none());
    }
}
