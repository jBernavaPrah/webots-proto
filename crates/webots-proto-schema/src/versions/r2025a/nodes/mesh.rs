use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `Mesh` node imports a 3D mesh from a file (STL, OBJ, DAE).
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/mesh?version=R2025a).
    Mesh {
    /// Default: []
    url: MFString,
    /// Default: -1
    material_index: SFInt32,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_new() {
        let m = Mesh::new();
        assert!(m.material_index.is_none());
    }
}
