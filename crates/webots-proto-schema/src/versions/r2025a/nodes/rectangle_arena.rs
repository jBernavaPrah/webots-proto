use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `RectangleArena` PROTO defines a simple walled arena.
    ///
    /// Note: This is a PROTO, not a built-in node, but commonly used.
    RectangleArena {
    /// Default: 1 1
    floor_size: SFVec2f,
    /// Default: 0.5 0.5
    floor_tile_size: SFVec2f,
    /// Default: "chequered"
    floor_appearance: SFString,
    /// Default: 0.1
    wall_height: SFFloat,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_arena_new() {
        let ra = RectangleArena::new();
        assert!(ra.floor_size.is_none());
    }
}
