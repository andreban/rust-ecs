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

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::sync::OnceLock;
    use super::ComponentSignature;
    use crate::{Component, get_next_component_type_id};

    struct A;
    impl Component for A {
        fn get_type_id() -> usize {
            static TYPE_ID: OnceLock<usize> = OnceLock::new();
            *TYPE_ID.get_or_init(get_next_component_type_id)
        }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }

    struct B;
    impl Component for B {
        fn get_type_id() -> usize {
            static TYPE_ID: OnceLock<usize> = OnceLock::new();
            *TYPE_ID.get_or_init(get_next_component_type_id)
        }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }

    struct C;
    impl Component for C {
        fn get_type_id() -> usize {
            static TYPE_ID: OnceLock<usize> = OnceLock::new();
            *TYPE_ID.get_or_init(get_next_component_type_id)
        }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }

    #[test]
    fn default_signature_is_empty() {
        let sig = ComponentSignature::default();
        let mut non_empty = ComponentSignature::default();
        non_empty.require_component::<A>();
        assert!(!sig.is_superset(&non_empty));
    }

    #[test]
    fn require_component_sets_bit() {
        let mut sig = ComponentSignature::default();
        sig.require_component::<A>();
        let mut expected = ComponentSignature::default();
        expected.require_component::<A>();
        assert_eq!(sig, expected);
    }

    #[test]
    fn remove_component_clears_bit() {
        let mut sig = ComponentSignature::default();
        sig.require_component::<A>();
        sig.remove_component::<A>();
        assert_eq!(sig, ComponentSignature::default());
    }

    #[test]
    fn require_same_component_twice_is_idempotent() {
        let mut sig = ComponentSignature::default();
        sig.require_component::<A>();
        let after_first = sig.clone();
        sig.require_component::<A>();
        assert_eq!(sig, after_first);
    }

    #[test]
    fn is_subset_true_when_all_bits_present() {
        let mut ab = ComponentSignature::default();
        ab.require_component::<A>();
        ab.require_component::<B>();

        let mut abc = ComponentSignature::default();
        abc.require_component::<A>();
        abc.require_component::<B>();
        abc.require_component::<C>();

        assert!(ab.is_subset(&abc));
    }

    #[test]
    fn is_subset_false_when_bit_missing() {
        let mut ab = ComponentSignature::default();
        ab.require_component::<A>();
        ab.require_component::<B>();

        let mut ac = ComponentSignature::default();
        ac.require_component::<A>();
        ac.require_component::<C>();

        assert!(!ab.is_subset(&ac));
    }

    #[test]
    fn is_superset_true() {
        let mut abc = ComponentSignature::default();
        abc.require_component::<A>();
        abc.require_component::<B>();
        abc.require_component::<C>();

        let mut ab = ComponentSignature::default();
        ab.require_component::<A>();
        ab.require_component::<B>();

        assert!(abc.is_superset(&ab));
    }

    #[test]
    fn is_superset_false() {
        let mut ac = ComponentSignature::default();
        ac.require_component::<A>();
        ac.require_component::<C>();

        let mut ab = ComponentSignature::default();
        ab.require_component::<A>();
        ab.require_component::<B>();

        assert!(!ac.is_superset(&ab));
    }

    #[test]
    fn empty_signature_is_subset_of_any() {
        let empty = ComponentSignature::default();

        let mut non_empty = ComponentSignature::default();
        non_empty.require_component::<A>();
        non_empty.require_component::<B>();

        assert!(empty.is_subset(&non_empty));
        assert!(empty.is_subset(&ComponentSignature::default()));
    }
}
