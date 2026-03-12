use super::Node;
use crate::types::*;
use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

define_node!(
    /// The `Slot` node allows modular extension of nodes.
    ///
    /// See [Webots Reference](https://cyberbotics.com/doc/reference/slot?version=R2025a).
    Slot {
    /// Default: ""
    #[serde(rename = "type")]
    type_: SFString,
    /// Default: NULL
    end_point: Box<Node>,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_new() {
        let slot = Slot::new();
        assert!(slot.type_.is_none());
        assert!(slot.end_point.is_none());
    }
}
