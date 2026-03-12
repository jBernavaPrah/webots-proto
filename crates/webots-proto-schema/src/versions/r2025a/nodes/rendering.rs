use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `TextureCoordinate` node specifies 2D texture coordinates.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/texturecoordinate?version=R2025a).
    TextureCoordinate {
    /// Default: []
    point: MFVec2f,
});

define_node!(
    /// The `Color` node defines a set of RGB colors.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/color?version=R2025a).
    Color {
    /// Default: []
    color: MFColor,
});

define_node!(
    /// The `Coordinate` node defines a set of 3D coordinates.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/coordinate?version=R2025a).
    Coordinate {
    /// Default: []
    point: MFVec3f,
});

define_node!(
    /// The `Normal` node specifies a set of 3D normals.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/normal?version=R2025a).
    Normal {
    /// Default: []
    vector: MFVec3f,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendering_new() {
        assert!(Color::new().color.is_none());
    }
}
