use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `Focus` node defines a controllable focus for a Camera device.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/focus?version=R2025a).
    Focus {
    /// Default: 0.0
    focal_distance: SFFloat,
    /// Default: 0.0
    focal_length: SFFloat,
    /// Default: 0.0
    max_focal_distance: SFFloat,
    /// Default: 0.0
    min_focal_distance: SFFloat,
});

define_node!(
    /// The `Lens` node simulates camera image distortion.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/lens?version=R2025a).
    Lens {
    /// Default: 0.5 0.5
    center: SFVec2f,
    /// Default: 0 0
    radial_coefficients: SFVec2f,
    /// Default: 0 0
    tangential_coefficients: SFVec2f,
});

define_node!(
    /// The `LensFlare` node simulates lens flares.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/lensflare?version=R2025a).
    LensFlare {
    /// Default: 0.4
    transparency: SFFloat,
    /// Default: 1.5
    scale: SFFloat,
    /// Default: -0.9
    bias: SFFloat,
    /// Default: 0.6
    dispersal: SFFloat,
    /// Default: 4
    samples: SFInt32,
    /// Default: 0.4
    halo_width: SFFloat,
    /// Default: 2.0
    chroma_distortion: SFFloat,
    /// Default: 2
    blur_iterations: SFInt32,
});

define_node!(
    /// The `Recognition` node provides object recognition capability.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/recognition?version=R2025a).
    Recognition {
    /// Default: 100.0
    max_range: SFFloat,
    /// Default: -1
    max_objects: SFInt32,
    /// Default: 1
    occlusion: SFInt32,
    /// Default: 1 0 0
    frame_color: SFColor,
    /// Default: 1
    frame_thickness: SFInt32,
    /// Default: FALSE
    segmentation: SFBool,
});

define_node!(
    /// The `Zoom` node defines a controllable zoom for a Camera device.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/zoom?version=R2025a).
    Zoom {
    /// Default: 1.5
    max_field_of_view: SFFloat,
    /// Default: 0.5
    min_field_of_view: SFFloat,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optics_new() {
        assert!(Focus::new().focal_distance.is_none());
        assert!(Lens::new().center.is_none());
        assert!(LensFlare::new().scale.is_none());
        assert!(Recognition::new().max_range.is_none());
        assert!(Zoom::new().max_field_of_view.is_none());
    }
}
