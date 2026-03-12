use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `Viewpoint` node defines the viewing position and orientation in the 3D window.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/viewpoint?version=R2025a).
    Viewpoint {
    /// Default: 0.785398
    field_of_view: SFFloat,
    /// Default: 0 0 1 0
    orientation: SFRotation,
    /// Default: 0 0 10
    position: SFVec3f,
    /// Default: ""
    description: SFString,
    /// Default: 0.05
    near: SFFloat,
    /// Default: 0.0
    far: SFFloat,
    /// Default: 1.0
    exposure: SFFloat,
    /// Default: ""
    follow: SFString,
    /// Default: "None"
    follow_type: SFString, // "None", "Tracking Shot", "Mounted Shot", "Pan and Tilt Shot"
    /// Default: 0.0
    follow_smoothness: SFFloat, // [0, 1]
    /// Default: 0.0
    ambient_occlusion_radius: SFFloat,
    /// Default: -1.0
    bloom_threshold: SFFloat,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewpoint_new() {
        let v = Viewpoint::new();
        assert!(v.field_of_view.is_none());
    }
}
