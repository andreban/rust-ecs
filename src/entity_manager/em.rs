use std::{
    any::Any,
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::ComponentSignature;

use super::{
    entity::get_next_entity_id, Component, ComponentTypeId, Entity, EntityId, GroupManager,
    TagManager,
};

/// A cloneable handle to the entity/component storage.
///
/// All clones share the same underlying data via `Rc<RefCell<…>>`. The
/// primary instance lives inside `EntityComponentSystem`; clones are handed
/// to systems during each update so
/// they can query and mutate components without borrowing the whole ECS.
#[derive(Clone)]
pub struct EntityManager {
    pub(crate) inner: Rc<RefCell<EntityManagerInner>>,
}

impl EntityManager {
    /// Creates a new, empty [`EntityManager`].
    pub fn new() -> Self {
        EntityManager { inner: Rc::new(RefCell::new(EntityManagerInner::new())) }
    }

    /// Commits pending spawns and despawns. Called automatically by
    /// `EntityComponentSystem::update`.
    pub fn update(&self) {
        self.inner.borrow_mut().update();
    }

    /// Creates a new entity and enqueues it for spawning on the next update.
    pub fn create_entity(&self) -> Entity {
        self.inner.borrow_mut().create_entity()
    }

    /// Enqueues an entity for destruction on the next update.
    pub fn destroy_entity(&self, entity: Entity) {
        self.inner.borrow_mut().destroy_entity(entity);
    }

    /// Adds an entity to a named group.
    pub fn add_entity_to_group(&self, entity: &Entity, group: &str) {
        self.inner
            .borrow_mut()
            .group_manager
            .add_entity_to_group(entity, group);
    }

    /// Associates a string tag with an entity (one tag per entity).
    pub fn set_tag(&self, entity: Entity, tag: &str) {
        self.inner.borrow_mut().tag_manager.set_tag(entity, tag);
    }

    /// Returns the component of type `C` attached to `entity`, or `None` if
    /// the entity does not have that component.
    pub fn get_component<C: Component + 'static>(
        &self,
        entity: &Entity,
    ) -> Option<Rc<RefCell<Box<C>>>> {
        self.inner.borrow().get_component::<C>(entity)
    }

    /// Attaches a component to an entity. Does not update system membership;
    /// prefer `EntityComponentSystem::add_component` for that.
    pub fn add_component<C: Component + 'static>(&self, entity: Entity, component: C) {
        self.inner.borrow_mut().add_component(entity, component);
    }

    /// Returns a cloned handle to the `TagManager`.
    pub fn tag_manager(&self) -> TagManager {
        self.inner.borrow().tag_manager.clone()
    }

    /// Returns a cloned handle to the `GroupManager`.
    pub fn group_manager(&self) -> GroupManager {
        self.inner.borrow().group_manager.clone()
    }

    // pub fn create_query<T>(&self) -> Query<T> {
    //     Query::new(self)
    // }
}

pub struct EntityManagerInner {
    pub(crate) components: HashMap<ComponentTypeId, HashMap<EntityId, Rc<dyn Any>>>,
    pub(crate) entities: HashMap<EntityId, Entity>,
    pub(crate) entities_to_spawn: HashSet<Entity>,
    pub(crate) entities_to_despawn: HashSet<Entity>,
    pub(crate) entity_component_signatures: HashMap<EntityId, ComponentSignature>,
    tag_manager: TagManager,
    group_manager: GroupManager,
}

impl EntityManagerInner {
    pub fn new() -> Self {
        EntityManagerInner {
            components: HashMap::new(),
            entities: HashMap::new(),
            entities_to_spawn: HashSet::new(),
            entities_to_despawn: HashSet::new(),
            entity_component_signatures: HashMap::new(),
            tag_manager: Default::default(),
            group_manager: Default::default(),
        }
    }

    pub fn update(&mut self) {
        // Add entities waiting to be created to systems.
        for entity in &mut self.entities_to_spawn.drain() {
            self.entities.insert(entity.id(), entity);
        }

        // Despawn entities waiting to be killed from systems.
        for entity in &mut self.entities_to_despawn.drain() {
            // Remove Signature.
            self.entity_component_signatures.remove(&entity.id());

            // Remove the components...
            for v in &mut self.components.values_mut() {
                v.remove(&entity.id());
            }

            // Remove entity.
            self.entities.remove(&entity.id());
        }
    }

    /// Creates a new entity and enqueues it to be added in the next update.
    pub fn create_entity(&mut self) -> Entity {
        let id: EntityId = get_next_entity_id();
        let entity = Entity::new(id);
        self.entities_to_spawn.insert(entity);
        entity
    }

    /// Enqueues an entity to be destroyed in the next update.
    pub fn destroy_entity(&mut self, entity: Entity) {
        // Remove components...
        self.entities_to_despawn.insert(entity);
    }

    /// Adds the Component `C` to the entity.
    pub fn add_component<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        let component_type_id = C::get_type_id();
        let component = Rc::new(RefCell::new(Box::new(component)));

        // Update the entity signature to indicate that the entity has the component.
        self.entity_component_signatures
            .entry(entity.id())
            .or_default()
            .require_component::<C>();

        // Add the component to the component storage.
        self.components
            .entry(component_type_id)
            .or_default()
            .insert(entity.id(), component);
    }

    /// Retrieves the component of type `C` from the entity, if available.
    pub fn get_component<C: Component + 'static>(
        &self,
        entity: &Entity,
    ) -> Option<Rc<RefCell<Box<C>>>> {
        let component_type_id = C::get_type_id();
        let entity_id = entity.id();
        let components = self.components.get(&component_type_id)?;
        let component = components.get(&entity_id)?.clone();
        let component: Rc<RefCell<Box<C>>> = component.downcast().ok()?;
        Some(component)
    }

    /// Removes the Component `C` from the entity.
    pub fn remove_component<C: Component + 'static>(&mut self, entity: Entity) {
        let component_type_id = C::get_type_id();
        if let Some(component) = self.components.get_mut(&component_type_id) {
            component.remove(&entity.id());
        }

        if let Some(signature) = self.entity_component_signatures.get_mut(&entity.id()) {
            signature.remove_component::<C>();
        }
    }

    /// Returns immutable borrows of every component of type `C`.
    ///
    /// Returns an empty `Vec` if no entity has a component of that type.
    pub fn query<C: Component + 'static>(&self) -> Vec<Ref<'_, C>> {
        let component_type_id = C::get_type_id();
        let Some(components) = self.components.get(&component_type_id) else {
            return vec![];
        };
        components
            .values()
            .filter_map(|c| (c as &dyn Any).downcast_ref::<RefCell<C>>())
            .map(|c| c.borrow())
            .collect()
    }

    /// Returns `RefCell` references to every component of type `C` for
    /// interior mutation.
    ///
    /// Returns an empty `Vec` if no entity has a component of that type.
    pub fn query_mut<C: Component + 'static>(&mut self) -> Vec<&RefCell<C>> {
        let component_type_id = C::get_type_id();
        let Some(components) = self.components.get_mut(&component_type_id) else {
            return vec![];
        };
        components
            .values()
            .filter_map(|c| (c as &dyn Any).downcast_ref::<RefCell<C>>())
            .collect()
    }

    /// Returns all live entities whose component signature is a superset of
    /// `signature`.
    pub fn get_entities_with_signature(&self, signature: &ComponentSignature) -> Vec<Entity> {
        self.entities
            .values()
            .filter(|e| {
                if let Some(s) = self.entity_component_signatures.get(&e.id()) {
                    signature.is_subset(s)
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }

    /// Returns the [`ComponentSignature`] of `entity`, or `None` if the entity
    /// is unknown.
    pub fn get_signature(&self, entity: Entity) -> Option<&ComponentSignature> {
        self.entity_component_signatures.get(&entity.id())
    }

    /// Returns a cloned handle to the `TagManager`.
    pub fn tag_manager(&self) -> TagManager {
        self.tag_manager.clone()
    }

    /// Returns a shared reference to the `GroupManager`.
    pub fn group_manager(&self) -> &GroupManager {
        &self.group_manager
    }

    /// Returns a mutable reference to the `GroupManager`.
    pub fn group_manager_mut(&mut self) -> &mut GroupManager {
        &mut self.group_manager
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}
