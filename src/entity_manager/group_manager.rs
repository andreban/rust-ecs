use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::{Entity, EntityId};

#[derive(Clone, Default)]
pub struct GroupManager {
    inner: Rc<RefCell<GroupManagerInner>>,
}

impl GroupManager {
    pub fn add_entity_to_group(&self, entity: &Entity, group: &str) {
        self.inner.borrow_mut().add_entity_to_group(entity, group);
    }

    pub fn remove_entity_from_group(&self, entity: &Entity, group: &str) {
        self.inner
            .borrow_mut()
            .remove_entity_from_group(entity, group);
    }

    pub fn remove_entity(&self, entity: &Entity) {
        self.inner.borrow_mut().remove_entity(entity);
    }

    pub fn group_contains_entity(&self, group: &str, entity: &Entity) -> bool {
        self.inner.borrow().group_contains_entity(group, entity)
    }

    pub fn entity_in_group(&self, entity: &Entity, group: &str) -> bool {
        self.inner.borrow().entity_in_group(entity, group)
    }
}

#[derive(Default)]
pub struct GroupManagerInner {
    entity_groups: HashMap<EntityId, HashSet<String>>,
    group_entities: HashMap<String, HashSet<EntityId>>,
}

impl GroupManagerInner {
    pub fn add_entity_to_group(&mut self, entity: &Entity, group: &str) {
        self.entity_groups
            .entry(entity.id())
            .or_default()
            .insert(group.to_string());
        self.group_entities
            .entry(group.to_string())
            .or_default()
            .insert(entity.id());
    }

    pub fn remove_entity_from_group(&mut self, entity: &Entity, group: &str) {
        if let Some(groups) = self.entity_groups.get_mut(&entity.id()) {
            groups.remove(group);
        }

        if let Some(entities) = self.group_entities.get_mut(group) {
            entities.remove(&entity.id());
        }
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        if let Some(groups) = self.entity_groups.remove(&entity.id()) {
            for group in groups {
                if let Some(entities) = self.group_entities.get_mut(&group) {
                    entities.remove(&entity.id());
                }
            }
        }
    }

    pub fn group_contains_entity(&self, group: &str, entity: &Entity) -> bool {
        if let Some(entities) = self.group_entities.get(group) {
            entities.contains(&entity.id())
        } else {
            false
        }
    }

    pub fn entity_in_group(&self, entity: &Entity, group: &str) -> bool {
        if let Some(groups) = self.entity_groups.get(&entity.id()) {
            groups.contains(group)
        } else {
            false
        }
    }
}
