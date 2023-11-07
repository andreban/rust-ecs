mod component;
mod entity;
mod query;

pub use component::{get_next_component_type_id, Component, ComponentTypeId};
pub use entity::{Entity, EntityId};
pub use query::Query;

use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
};

use crate::component_signature::ComponentSignature;

use self::entity::get_next_entity_id;

pub struct EntityManager {
    components: HashMap<ComponentTypeId, HashMap<EntityId, RefCell<Box<dyn Any>>>>,
    entities: HashMap<EntityId, Entity>,
    pub(crate) entities_to_spawn: HashSet<Entity>,
    pub(crate) entities_to_despawn: HashSet<Entity>,
    entity_component_signatures: HashMap<EntityId, ComponentSignature>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            components: HashMap::new(),
            entities: HashMap::new(),
            entities_to_spawn: HashSet::new(),
            entities_to_despawn: HashSet::new(),
            entity_component_signatures: HashMap::new(),
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
        let component = RefCell::new(Box::new(component));

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
    pub fn get_component<C: Component + 'static>(&self, entity: Entity) -> Option<Ref<C>> {
        let component_type_id = C::get_type_id();
        let entity_id = entity.id();
        let components = self.components.get(&component_type_id)?;
        let component = components.get(&entity_id)?;
        let component = Ref::map(component.borrow(), |f| f.downcast_ref::<C>().unwrap());
        Some(component)
    }

    /// Retrieves the component of type `C` from the entity, if available.
    pub fn get_component_mut<C: Component + 'static>(&self, entity: Entity) -> Option<RefMut<C>> {
        let component_type_id = C::get_type_id();
        let entity_id = entity.id();
        let components = self.components.get(&component_type_id)?;
        let component = components.get(&entity_id)?;
        let component = RefMut::map(component.borrow_mut(), |f| f.downcast_mut::<C>().unwrap());
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

    pub fn query<C: Component + 'static>(&self) -> Vec<Ref<C>> {
        let component_type_id = C::get_type_id();
        let components = self.components.get(&component_type_id).unwrap();
        components
            .iter()
            .map(|(_, c)| {
                (c as &dyn Any)
                    .downcast_ref::<RefCell<C>>()
                    .unwrap()
                    .borrow()
            })
            .collect()
    }

    pub fn query_mut<C: Component + 'static>(&mut self) -> Vec<&RefCell<C>> {
        let component_type_id = C::get_type_id();
        let components = self.components.get_mut(&component_type_id).unwrap();
        components
            .iter()
            .map(|(_, c)| (c as &dyn Any).downcast_ref::<RefCell<C>>().unwrap())
            .collect()
    }

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

    pub fn get_signature(&self, entity: Entity) -> Option<&ComponentSignature> {
        self.entity_component_signatures.get(&entity.id())
    }

    pub fn create_query<T>(&self) -> Query<T> {
        Query::new(self)
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}
