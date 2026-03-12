use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `Appearance` node specifies the visual properties of a geometry.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/appearance?version=R2025a).
    Appearance {
    /// Default: NULL
    material: Box<Node>,
    /// Default: NULL
    texture: Box<Node>,
    /// Default: NULL
    texture_transform: Box<Node>,
});

define_node!(
    /// The `Material` node specifies surface material properties.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/material?version=R2025a).
    Material {
    /// Default: 0.2
    ambient_intensity: SFFloat,
    /// Default: 0.8 0.8 0.8
    diffuse_color: SFColor,
    /// Default: 0 0 0
    emissive_color: SFColor,
    /// Default: 0.2
    shininess: SFFloat,
    /// Default: 0 0 0
    specular_color: SFColor,
    /// Default: 0.0
    transparency: SFFloat,
});

define_node!(
    /// The `PBRAppearance` node specifies a physically-based visual appearance.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/pbrappearance?version=R2025a).
    PBRAppearance {
    /// Default: 1 1 1
    base_color: SFColor,
    /// Default: NULL
    base_color_map: Box<Node>,
    /// Default: 0.0
    transparency: SFFloat,
    /// Default: 0.0
    roughness: SFFloat,
    /// Default: NULL
    roughness_map: Box<Node>,
    /// Default: 1.0
    metalness: SFFloat,
    /// Default: NULL
    metalness_map: Box<Node>,
    /// Default: 1.0
    ibl_strength: SFFloat,
    /// Default: NULL
    normal_map: Box<Node>,
    /// Default: 1.0
    normal_map_factor: SFFloat,
    /// Default: NULL
    occlusion_map: Box<Node>,
    /// Default: 1.0
    occlusion_map_strength: SFFloat,
    /// Default: 0 0 0
    emissive_color: SFColor,
    /// Default: NULL
    emissive_color_map: Box<Node>,
    /// Default: 1.0
    emissive_intensity: SFFloat,
    /// Default: NULL
    texture_transform: Box<Node>,
});

define_node!(
    /// The `ImageTexture` node defines a texture map.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/imagetexture?version=R2025a).
    ImageTexture {
    /// Default: []
    url: MFString,
    /// Default: TRUE
    repeat_s: SFBool,
    /// Default: TRUE
    repeat_t: SFBool,
    /// Default: 4
    filtering: SFInt32,
});

define_node!(
    /// The `TextureTransform` node defines a 2D transformation for texture coordinates.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/texturetransform?version=R2025a).
    TextureTransform {
    /// Default: 0 0
    center: SFVec2f,
    /// Default: 0.0
    rotation: SFFloat,
    /// Default: 1 1
    scale: SFVec2f,
    /// Default: 0 0
    translation: SFVec2f,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appearance_new() {
        let app = Appearance::new();
        assert!(app.material.is_none());
    }

    #[test]
    fn test_material_new() {
        assert!(Material::new().shininess.is_none());
    }

    #[test]
    fn test_pbr_appearance_new() {
        let pbr = PBRAppearance::new();
        assert!(pbr.base_color.is_none());
    }

    #[test]
    fn test_image_texture_new() {
        assert!(ImageTexture::new().filtering.is_none());
    }

    #[test]
    fn test_texture_transform_new() {
        assert!(TextureTransform::new().scale.is_none());
    }
}
