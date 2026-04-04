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
