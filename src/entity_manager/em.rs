use std::{
    any::Any,
    cell::{RefCell, RefMut},
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
        self.inner.borrow().tag_manager()
    }

    /// Returns a cloned handle to the `GroupManager`.
    pub fn group_manager(&self) -> GroupManager {
        self.inner.borrow().group_manager().clone()
    }

    /// Returns owned handles to every component of type `C`.
    ///
    /// Each handle is an `Rc<RefCell<Box<C>>>` giving both shared access and
    /// interior mutability. Returns an empty `Vec` if no entity has that component.
    pub fn query<C: Component + 'static>(&self) -> Vec<Rc<RefCell<Box<C>>>> {
        self.inner.borrow().query::<C>()
    }

    /// Returns owned mutable handles to every component of type `C`.
    ///
    /// Equivalent to [`query`] — each `Rc<RefCell<Box<C>>>` handle supports
    /// interior mutability via `borrow_mut()`.
    pub fn query_mut<C: Component + 'static>(&self) -> Vec<Rc<RefCell<Box<C>>>> {
        self.inner.borrow_mut().query_mut::<C>()
    }

    /// Returns all live entities whose component signature is a superset of
    /// `signature`.
    pub fn get_entities_with_signature(&self, signature: &ComponentSignature) -> Vec<Entity> {
        self.inner.borrow().get_entities_with_signature(signature)
    }

    /// Returns a `RefMut` guard providing mutable access to the `GroupManager`.
    ///
    /// The guard releases the borrow when dropped. Do not hold it across `.await`
    /// points or while calling other `EntityManager` methods.
    pub fn group_manager_mut(&self) -> RefMut<'_, GroupManager> {
        RefMut::map(self.inner.borrow_mut(), |inner| inner.group_manager_mut())
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

    /// Returns owned handles to every component of type `C`.
    ///
    /// Each handle is an `Rc<RefCell<Box<C>>>` giving both shared access and
    /// interior mutability. Returns an empty `Vec` if no entity has that component.
    pub fn query<C: Component + 'static>(&self) -> Vec<Rc<RefCell<Box<C>>>> {
        let component_type_id = C::get_type_id();
        let Some(components) = self.components.get(&component_type_id) else {
            return vec![];
        };
        components
            .values()
            .filter_map(|c| c.clone().downcast::<RefCell<Box<C>>>().ok())
            .collect()
    }

    /// Returns owned mutable handles to every component of type `C`.
    ///
    /// Equivalent to [`query`] — each `Rc<RefCell<Box<C>>>` handle supports
    /// interior mutability via `borrow_mut()`.
    pub fn query_mut<C: Component + 'static>(&mut self) -> Vec<Rc<RefCell<Box<C>>>> {
        self.query::<C>()
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

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::collections::HashSet;
    use std::sync::OnceLock;
    use super::EntityManagerInner;
    use crate::{Component, ComponentSignature, get_next_component_type_id};

    struct TestHealth(u32);

    impl Component for TestHealth {
        fn get_type_id() -> usize {
            static TYPE_ID: OnceLock<usize> = OnceLock::new();
            *TYPE_ID.get_or_init(get_next_component_type_id)
        }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }

    #[allow(dead_code)]
    struct TestVelocity(f32, f32);

    impl Component for TestVelocity {
        fn get_type_id() -> usize {
            static TYPE_ID: OnceLock<usize> = OnceLock::new();
            *TYPE_ID.get_or_init(get_next_component_type_id)
        }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }

    #[test]
    fn create_entity_increments_id() {
        let mut em = EntityManagerInner::new();
        let e1 = em.create_entity();
        let e2 = em.create_entity();
        assert_ne!(e1.id(), e2.id());
    }

    #[test]
    fn create_multiple_entities() {
        let mut em = EntityManagerInner::new();
        let ids: HashSet<_> = (0..10).map(|_| em.create_entity().id()).collect();
        assert_eq!(ids.len(), 10);
    }

    #[test]
    fn destroy_entity_removes_components() {
        let mut em = EntityManagerInner::new();
        let entity = em.create_entity();
        em.update(); // spawn entity
        em.add_component(entity, TestHealth(100));
        assert!(em.get_component::<TestHealth>(&entity).is_some());
        em.destroy_entity(entity);
        em.update(); // process despawn
        assert!(em.get_component::<TestHealth>(&entity).is_none());
    }

    #[test]
    fn add_component_then_get() {
        let mut em = EntityManagerInner::new();
        let entity = em.create_entity();
        em.add_component(entity, TestHealth(42));
        let comp = em.get_component::<TestHealth>(&entity).expect("component should be present");
        assert_eq!(comp.borrow().0, 42);
    }

    #[test]
    fn add_component_updates_signature() {
        let mut em = EntityManagerInner::new();
        let entity = em.create_entity();
        em.add_component(entity, TestHealth(0));
        let sig = em.get_signature(entity).expect("signature should exist");
        let mut expected = ComponentSignature::default();
        expected.require_component::<TestHealth>();
        assert_eq!(sig, &expected);
    }

    #[test]
    fn remove_component_updates_signature() {
        let mut em = EntityManagerInner::new();
        let entity = em.create_entity();
        em.add_component(entity, TestHealth(0));
        em.remove_component::<TestHealth>(entity);
        let sig = em.get_signature(entity).expect("signature entry should still exist");
        assert_eq!(sig, &ComponentSignature::default());
    }

    #[test]
    fn get_component_wrong_type_returns_none() {
        let mut em = EntityManagerInner::new();
        let entity = em.create_entity();
        em.add_component(entity, TestHealth(0));
        assert!(em.get_component::<TestVelocity>(&entity).is_none());
    }

    #[test]
    fn update_commits_pending_destroys() {
        let mut em = EntityManagerInner::new();
        let entity = em.create_entity();
        em.update(); // spawn
        assert!(em.entities.contains_key(&entity.id()));
        em.destroy_entity(entity);
        em.update(); // despawn
        assert!(!em.entities.contains_key(&entity.id()));
    }

    #[test]
    fn query_returns_all_matching_components() {
        let mut em = EntityManagerInner::new();
        let e1 = em.create_entity();
        let e2 = em.create_entity();
        let _e3 = em.create_entity();
        em.add_component(e1, TestHealth(1));
        em.add_component(e2, TestHealth(2));
        let results = em.query::<TestHealth>();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn get_entities_with_signature_filters_correctly() {
        let mut em = EntityManagerInner::new();
        let e1 = em.create_entity();
        let e2 = em.create_entity();
        em.add_component(e1, TestHealth(10));
        em.add_component(e2, TestVelocity(1.0, 2.0));
        em.update(); // commit spawns so entities appear in live set

        let mut sig = ComponentSignature::default();
        sig.require_component::<TestHealth>();

        let matches = em.get_entities_with_signature(&sig);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], e1);
    }
}
