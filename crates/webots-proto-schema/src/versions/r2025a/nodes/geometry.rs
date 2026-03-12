use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `Box` node specifies a rectangular parallelepiped box.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/box?version=R2025a).
    BoxNode("Box") {
    /// Default: 2 2 2
    size: SFVec3f,
});

define_node!(
    /// The `Sphere` node specifies a sphere.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/sphere?version=R2025a).
    Sphere {
    /// Default: 1.0
    radius: SFFloat,
    /// Default: 1
    subdivision: SFInt32,
    /// Default: TRUE
    ico: SFBool,
});

define_node!(
    /// The `Capsule` node specifies a capsule geometry.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/capsule?version=R2025a).
    Capsule {
    /// Default: TRUE
    bottom: SFBool,
    /// Default: 2.0
    height: SFFloat,
    /// Default: 1.0
    radius: SFFloat,
    /// Default: TRUE
    side: SFBool,
    /// Default: TRUE
    top: SFBool,
    /// Default: 12
    subdivision: SFInt32,
});

define_node!(
    /// The `Cylinder` node specifies a cylinder geometry.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/cylinder?version=R2025a).
    Cylinder {
    /// Default: TRUE
    bottom: SFBool,
    /// Default: 2.0
    height: SFFloat,
    /// Default: 1.0
    radius: SFFloat,
    /// Default: TRUE
    side: SFBool,
    /// Default: TRUE
    top: SFBool,
    /// Default: 12
    subdivision: SFInt32,
});

define_node!(
    /// The `Plane` node specifies a flat plane.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/plane?version=R2025a).
    Plane {
    /// Default: 1 1
    size: SFVec2f,
});

define_node!(
    /// The `Cone` node specifies a cone geometry.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/cone?version=R2025a).
    Cone {
    /// Default: 0.5
    bottom_radius: SFFloat,
    /// Default: 1.0
    height: SFFloat,
    /// Default: TRUE
    side: SFBool,
    /// Default: TRUE
    bottom: SFBool,
    /// Default: 12
    subdivision: SFInt32,
});

define_node!(
    /// The `ElevationGrid` node specifies a uniform rectangular grid of varying height.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/elevationgrid?version=R2025a).
    ElevationGrid {
    /// Default: []
    height: MFFloat,
    /// Default: 0
    x_dimension: SFInt32,
    /// Default: 1.0
    x_spacing: SFFloat,
    /// Default: 0
    z_dimension: SFInt32,
    /// Default: 1.0
    z_spacing: SFFloat,
    /// Default: 0.0
    thickness: SFFloat,
});

define_node!(
    /// The `IndexedFaceSet` node specifies a 3D shape formed by constructing faces from vertices.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/indexedfaceset?version=R2025a).
    IndexedFaceSet {
    /// Default: NULL
    coord: Box<super::Node>,
    /// Default: NULL
    normal: Box<super::Node>,
    /// Default: NULL
    tex_coord: Box<super::Node>,
    /// Default: TRUE
    solid: SFBool,
    /// Default: TRUE
    convex: SFBool,
    /// Default: []
    coord_index: MFInt32,
    /// Default: []
    normal_index: MFInt32,
    /// Default: []
    tex_coord_index: MFInt32,
    /// Default: -1.0
    crease_angle: SFFloat,
});

define_node!(
    /// The `IndexedLineSet` node specifies a 3D geometry formed by constructing lines from vertices.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/indexedlineset?version=R2025a).
    IndexedLineSet {
    /// Default: NULL
    coord: Box<super::Node>,
    /// Default: []
    coord_index: MFInt32,
});

define_node!(
    /// The `PointSet` node specifies a set of 3D points.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/pointset?version=R2025a).
    PointSet {
    /// Default: NULL
    coord: Box<super::Node>,
    /// Default: NULL
    color: Box<super::Node>,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_new() {
        assert!(BoxNode::new().size.is_none());
        assert!(Sphere::new().radius.is_none());
        assert!(Capsule::new().height.is_none());
        assert!(Cylinder::new().radius.is_none());
        assert!(Plane::new().size.is_none());
    }
}
