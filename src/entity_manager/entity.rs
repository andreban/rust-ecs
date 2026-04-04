use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(test)]
mod tests {
    use super::Entity;

    #[test]
    fn entity_new_has_correct_id() {
        let entity = Entity::new(5);
        assert_eq!(entity.id(), 5);
    }

    #[test]
    fn entity_copy() {
        let entity = Entity::new(42);
        let copy = entity;
        assert_eq!(entity.id(), copy.id());
    }

    #[test]
    fn entity_equality() {
        let a = Entity::new(7);
        let b = Entity::new(7);
        assert_eq!(a, b);
    }
}

/// Numeric identifier for a live entity, unique within a single ECS instance.
pub type EntityId = usize;

pub(super) fn get_next_entity_id() -> usize {
    static NEXT_ENTITY_ID: AtomicUsize = AtomicUsize::new(0);
    NEXT_ENTITY_ID.fetch_add(1, Ordering::Relaxed)
}

/// A lightweight, copyable handle that identifies a single entity in the ECS.
///
/// Entities are created via `EntityComponentSystem::create_entity` (or
/// `EntityManager::create_entity`). The handle is stable for the lifetime of
/// the entity; once an entity is destroyed the ID may be reused by future
/// entities.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    id: EntityId,
}

impl Entity {
    /// Creates an entity handle wrapping the given raw `EntityId`.
    pub fn new(id: EntityId) -> Self {
        Entity { id }
    }

    /// Returns the raw numeric ID of this entity.
    pub fn id(&self) -> EntityId {
        self.id
    }
}
