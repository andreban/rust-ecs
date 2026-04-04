use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::{Entity, EntityId};

/// Manages many-to-many relationships between entities and named groups.
///
/// An entity can belong to multiple groups simultaneously; a group can contain
/// multiple entities.
#[derive(Clone, Default)]
pub struct GroupManager {
    inner: Rc<RefCell<GroupManagerInner>>,
}

impl GroupManager {
    /// Adds `entity` to the group identified by `group`.
    pub fn add_entity_to_group(&self, entity: &Entity, group: &str) {
        self.inner.borrow_mut().add_entity_to_group(entity, group);
    }

    /// Removes `entity` from the group identified by `group`.
    pub fn remove_entity_from_group(&self, entity: &Entity, group: &str) {
        self.inner
            .borrow_mut()
            .remove_entity_from_group(entity, group);
    }

    /// Removes `entity` from all groups it belongs to.
    pub fn remove_entity(&self, entity: &Entity) {
        self.inner.borrow_mut().remove_entity(entity);
    }

    /// Returns `true` if `group` contains `entity`.
    /// Returns `true` if `group` contains `entity`.
    pub fn group_contains_entity(&self, group: &str, entity: &Entity) -> bool {
        self.inner.borrow().group_contains_entity(group, entity)
    }

    /// Returns `true` if `entity` belongs to `group`.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entity(id: EntityId) -> Entity {
        Entity::new(id)
    }

    #[test]
    fn add_entity_to_group_then_contains() {
        let gm = GroupManager::default();
        let entity = make_entity(1);
        gm.add_entity_to_group(&entity, "enemies");
        assert!(gm.group_contains_entity("enemies", &entity));
    }

    #[test]
    fn entity_not_in_group_returns_false() {
        let gm = GroupManager::default();
        let entity = make_entity(1);
        assert!(!gm.group_contains_entity("enemies", &entity));
    }

    #[test]
    fn entity_in_group_matches_group_contains_entity() {
        let gm = GroupManager::default();
        let entity = make_entity(1);
        gm.add_entity_to_group(&entity, "allies");
        assert_eq!(
            gm.entity_in_group(&entity, "allies"),
            gm.group_contains_entity("allies", &entity)
        );
    }

    #[test]
    fn remove_entity_from_group() {
        let gm = GroupManager::default();
        let entity = make_entity(1);
        gm.add_entity_to_group(&entity, "enemies");
        gm.remove_entity_from_group(&entity, "enemies");
        assert!(!gm.group_contains_entity("enemies", &entity));
    }

    #[test]
    fn entity_can_be_in_multiple_groups() {
        let gm = GroupManager::default();
        let entity = make_entity(1);
        gm.add_entity_to_group(&entity, "enemies");
        gm.add_entity_to_group(&entity, "bosses");
        assert!(gm.group_contains_entity("enemies", &entity));
        assert!(gm.group_contains_entity("bosses", &entity));
    }

    #[test]
    fn remove_entity_removes_from_all_groups() {
        let gm = GroupManager::default();
        let entity = make_entity(1);
        gm.add_entity_to_group(&entity, "enemies");
        gm.add_entity_to_group(&entity, "bosses");
        gm.remove_entity(&entity);
        assert!(!gm.group_contains_entity("enemies", &entity));
        assert!(!gm.group_contains_entity("bosses", &entity));
    }

    #[test]
    fn empty_group_name_is_valid() {
        let gm = GroupManager::default();
        let entity = make_entity(1);
        gm.add_entity_to_group(&entity, "");
        assert!(gm.group_contains_entity("", &entity));
    }

    #[test]
    fn multiple_entities_in_same_group() {
        let gm = GroupManager::default();
        let e1 = make_entity(1);
        let e2 = make_entity(2);
        gm.add_entity_to_group(&e1, "enemies");
        gm.add_entity_to_group(&e2, "enemies");
        gm.remove_entity(&e1);
        assert!(!gm.group_contains_entity("enemies", &e1));
        assert!(gm.group_contains_entity("enemies", &e2));
    }
}
