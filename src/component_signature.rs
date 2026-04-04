use fixedbitset::FixedBitSet;

use crate::{Component, MAX_COMPONENTS};

/// A fixed-size bitset that records which component types an entity (or
/// system) requires.
///
/// Each bit position corresponds to a `ComponentTypeId`.
/// Capacity is capped at [`MAX_COMPONENTS`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ComponentSignature {
    signature: FixedBitSet,
}

impl Default for ComponentSignature {
    fn default() -> Self {
        Self { signature: FixedBitSet::with_capacity(MAX_COMPONENTS) }
    }
}

impl ComponentSignature {
    /// Sets the bit for component type `C`, marking it as required.
    pub fn require_component<C: Component>(&mut self) {
        let type_id = C::get_type_id();
        self.signature.set(type_id, true);
    }

    /// Clears the bit for component type `C`.
    pub fn remove_component<C: Component>(&mut self) {
        let type_id = C::get_type_id();
        self.signature.set(type_id, false);
    }

    /// Returns `true` if every bit set in `self` is also set in `other`.
    ///
    /// Used to check whether a system's required components are all present on
    /// an entity.
    pub fn is_subset(&self, other: &ComponentSignature) -> bool {
        self.signature.is_subset(&other.signature)
    }

    /// Returns `true` if every bit set in `other` is also set in `self`.
    pub fn is_superset(&self, other: &ComponentSignature) -> bool {
        self.signature.is_superset(&other.signature)
    }
}
