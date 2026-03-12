use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `Shape` node is the fundamental visual node in VRML/Webots.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/shape?version=R2025a).
    Shape {
    /// Default: NULL
    appearance: Box<Node>,
    /// Default: NULL
    geometry: Box<Node>,
    /// Default: TRUE
    cast_shadows: SFBool,
    /// Default: TRUE
    is_pickable: SFBool,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_new() {
        let shape = Shape::new();
        assert!(shape.cast_shadows.is_none());
    }
}
