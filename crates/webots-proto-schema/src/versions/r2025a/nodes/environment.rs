use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `Background` node defines the background used for rendering the 3D world.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/background?version=R2025a).
    Background {
    /// Color of the sky when no texture is present.
    /// Default: [0 0 0]
    sky_color: MFColor,
    /// Texture URL for the back face of the skybox.
    /// Default: []
    back_url: MFString,
    /// Texture URL for the bottom face of the skybox.
    /// Default: []
    bottom_url: MFString,
    /// Texture URL for the front face of the skybox.
    /// Default: []
    front_url: MFString,
    /// Texture URL for the left face of the skybox.
    /// Default: []
    left_url: MFString,
    /// Texture URL for the right face of the skybox.
    /// Default: []
    right_url: MFString,
    /// Texture URL for the top face of the skybox.
    /// Default: []
    top_url: MFString,
    /// Irradiance texture URL for the back face.
    /// Default: []
    back_irradiance_url: MFString,
    /// Irradiance texture URL for the bottom face.
    /// Default: []
    bottom_irradiance_url: MFString,
    /// Irradiance texture URL for the front face.
    /// Default: []
    front_irradiance_url: MFString,
    /// Irradiance texture URL for the left face.
    /// Default: []
    left_irradiance_url: MFString,
    /// Irradiance texture URL for the right face.
    /// Default: []
    right_irradiance_url: MFString,
    /// Irradiance texture URL for the top face.
    /// Default: []
    top_irradiance_url: MFString,
    /// Luminosity scale factor.
    /// Default: 1.0
    luminosity: SFFloat,
});

define_node!(
    /// The `Fog` node simulates atmospheric effects.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/fog?version=R2025a).
    Fog {
    /// Color of the fog.
    /// Default: 1 1 1
    color: SFColor,
    /// Type of fog ("LINEAR", "EXPONENTIAL", "EXPONENTIAL2").
    /// Default: "LINEAR"
    fog_type: SFString,
    /// Distance at which objects are totally obscured.
    /// Default: 0.0
    visibility_range: SFFloat,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_new() {
        assert!(Background::new().luminosity.is_none());
        assert!(Fog::new().fog_type.is_none());
    }
}
