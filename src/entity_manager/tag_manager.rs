use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{Entity, EntityId};

#[derive(Clone, Default)]
pub struct TagManager {
    inner: Rc<RefCell<TagManagerInner>>,
}

impl TagManager {
    pub fn set_tag(&self, entity: Entity, tag: &str) {
        self.inner.borrow_mut().set_tag(entity, tag);
    }

    pub fn remove_tag(&self, entity: Entity) {
        self.inner.borrow_mut().remove_tag(entity);
    }

    pub fn has_tag(&self, entity: Entity, tag: &str) -> bool {
        self.inner.borrow().has_tag(entity, tag)
    }

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
