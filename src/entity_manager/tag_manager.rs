use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{Entity, EntityId};

/// Manages a one-to-one mapping between entities and string tags.
///
/// Each entity can hold at most one tag. Assigning a new tag replaces the
/// previous one.
#[derive(Clone, Default)]
pub struct TagManager {
    inner: Rc<RefCell<TagManagerInner>>,
}

impl TagManager {
    /// Associates `tag` with `entity`. Replaces any existing tag on the entity.
    pub fn set_tag(&self, entity: Entity, tag: &str) {
        self.inner.borrow_mut().set_tag(entity, tag);
    }

    /// Removes the tag from `entity`, if any.
    /// Removes the tag from `entity`, if any.
    pub fn remove_tag(&self, entity: Entity) {
        self.inner.borrow_mut().remove_tag(entity);
    }

    /// Returns `true` if `entity` is currently tagged with `tag`.
    pub fn has_tag(&self, entity: Entity, tag: &str) -> bool {
        self.inner.borrow().has_tag(entity, tag)
    }

    /// Returns the entity associated with `tag`, or `None` if no entity has
    /// that tag.
    pub fn get_entity(&self, tag: &str) -> Option<Entity> {
        self.inner.borrow().get_entity(tag)
    }
}

#[derive(Default)]
pub struct TagManagerInner {
    entity_tag: HashMap<EntityId, String>,
    tag_entity: HashMap<String, EntityId>,
}

impl TagManagerInner {
    pub fn set_tag(&mut self, entity: Entity, tag: &str) {
        // If another entity already holds this tag, clear its entity_tag entry.
        if let Some(&old_entity_id) = self.tag_entity.get(tag) {
            if old_entity_id != entity.id() {
                self.entity_tag.remove(&old_entity_id);
            }
        }
        // If this entity already has a different tag, remove that tag's reverse mapping.
        if let Some(old_tag) = self.entity_tag.get(&entity.id()) {
            if old_tag != tag {
                self.tag_entity.remove(&old_tag.clone());
            }
        }
        self.entity_tag.insert(entity.id(), tag.to_string());
        self.tag_entity.insert(tag.to_string(), entity.id());
    }

    pub fn remove_tag(&mut self, entity: Entity) {
        if let Some(tag) = self.entity_tag.remove(&entity.id()) {
            self.tag_entity.remove(&tag);
        }
    }

    pub fn has_tag(&self, entity: Entity, tag: &str) -> bool {
        match self.entity_tag.get(&entity.id()) {
            Some(t) => t == tag,
            None => false,
        }
    }

    pub fn get_entity(&self, tag: &str) -> Option<Entity> {
        self.tag_entity.get(tag).map(|id| Entity::new(*id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entity(id: EntityId) -> Entity {
        Entity::new(id)
    }

    #[test]
    fn set_tag_then_has_tag() {
        let tm = TagManager::default();
        let entity = make_entity(1);
        tm.set_tag(entity, "player");
        assert!(tm.has_tag(entity, "player"));
    }

    #[test]
    fn has_tag_wrong_tag_returns_false() {
        let tm = TagManager::default();
        let entity = make_entity(1);
        tm.set_tag(entity, "player");
        assert!(!tm.has_tag(entity, "enemy"));
    }

    #[test]
    fn get_entity_returns_tagged_entity() {
        let tm = TagManager::default();
        let entity = make_entity(1);
        tm.set_tag(entity, "player");
        assert_eq!(tm.get_entity("player"), Some(entity));
    }

    #[test]
    fn get_entity_unknown_tag_returns_none() {
        let tm = TagManager::default();
        assert_eq!(tm.get_entity("unknown"), None);
    }

    #[test]
    fn remove_tag_clears_lookup() {
        let tm = TagManager::default();
        let entity = make_entity(1);
        tm.set_tag(entity, "player");
        tm.remove_tag(entity);
        assert!(!tm.has_tag(entity, "player"));
        assert_eq!(tm.get_entity("player"), None);
    }

    #[test]
    fn set_tag_overwrites_previous_tag() {
        let tm = TagManager::default();
        let entity = make_entity(1);
        tm.set_tag(entity, "player");
        tm.set_tag(entity, "boss");
        assert!(!tm.has_tag(entity, "player"));
        assert!(tm.has_tag(entity, "boss"));
    }

    #[test]
    fn set_tag_overwrites_existing_entity_for_tag() {
        let tm = TagManager::default();
        let entity1 = make_entity(1);
        let entity2 = make_entity(2);
        tm.set_tag(entity1, "player");
        tm.set_tag(entity2, "player");
        assert_eq!(tm.get_entity("player"), Some(entity2));
        assert!(!tm.has_tag(entity1, "player"));
    }
}
