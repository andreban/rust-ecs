use fixedbitset::FixedBitSet;

use crate::Component;

const MAX_COMPONENTS: usize = 32;

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
    pub fn require_component<C: Component>(&mut self) {
        let type_id = C::get_type_id();
        self.signature.set(type_id, true);
    }

    pub fn remove_component<C: Component>(&mut self) {
        let type_id = C::get_type_id();
        self.signature.set(type_id, false);
    }

    pub fn is_subset(&self, other: &ComponentSignature) -> bool {
        self.signature.is_subset(&other.signature)
    }

    pub fn is_superset(&self, other: &ComponentSignature) -> bool {
        self.signature.is_superset(&other.signature)
    }
}
