use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `CadShape` node imports a 3D geometry from a file (Collada or Wavefront OBJ).
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/cadshape?version=R2025a).
    CadShape {
    /// Default: []
    url: MFString,
    /// Default: TRUE
    ccw: SFBool,
    /// Default: TRUE
    cast_shadows: SFBool,
    /// Default: TRUE
    is_pickable: SFBool,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cad_shape_new() {
        let cs = CadShape::new();
        assert!(cs.ccw.is_none());
    }
}
