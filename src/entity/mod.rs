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

use fixedbitset::FixedBitSet;

use self::entity::get_next_entity_id;

// The signature of an entity is a bitset that indicates which components the entity has,
// using the ComponentTypeId.
pub type Signature = FixedBitSet;

pub struct EntityManager {
    components: HashMap<ComponentTypeId, HashMap<EntityId, RefCell<Box<dyn Any>>>>,
    // TODO: use a Map<EntityId, Entity> instead, to improve removal performance.
    entities: Vec<Entity>,
    entities_to_spawn: HashSet<Entity>,
    entities_to_despawn: HashSet<Entity>,
    entity_component_signatures: HashMap<EntityId, Signature>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            components: HashMap::new(),
            entities: Vec::new(),
            entities_to_spawn: HashSet::new(),
            entities_to_despawn: HashSet::new(),
            entity_component_signatures: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        // Add entities waiting to be created to systems.
        for entity in &mut self.entities_to_spawn.drain() {
            self.entities.push(entity);
        }

        // Despawn entities waiting to be killed from systems.
        for entity in &mut self.entities_to_despawn.drain() {
            // Remove the components...
            for v in &mut self.components.values_mut() {
                v.remove(&entity.id());
            }

            // Remove entity.
            let pos = self
                .entities
                .iter()
                .position(|e| e.id() == entity.id())
                .unwrap();
            self.entities.remove(pos);
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let id: EntityId = get_next_entity_id();
        let entity = Entity::new(id);
        self.entities_to_spawn.insert(entity);
        EntityBuilder { entity, entity_manager: self }
    }

    pub fn edit_entity(&mut self, id: EntityId) -> EntityBuilder {
        EntityBuilder { entity: Entity::new(id), entity_manager: self }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        // TODO: Remove components...
        self.entities.retain(|e| *e != entity);
    }

    pub fn add_component<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        let component_type_id = C::get_type_id();
        let component = RefCell::new(Box::new(component));

        // Update the entity signature to indicate that the entity has the component.
        self.entity_component_signatures
            .entry(entity.id())
            .or_insert_with(|| FixedBitSet::with_capacity(32))
            .set(component_type_id, true);

        // Add the component to the component storage.
        self.components
            .entry(component_type_id)
            .or_default()
            .insert(entity.id(), component);
    }

    pub fn get_component<C: Component + 'static>(&self, entity: Entity) -> Option<Ref<C>> {
        let component_type_id = C::get_type_id();
        let entity_id = entity.id();
        let components = self.components.get(&component_type_id)?;
        let component = components.get(&entity_id)?;
        let component = Ref::map(component.borrow(), |f| f.downcast_ref::<C>().unwrap());
        Some(component)
    }

    pub fn get_component_mut<C: Component + 'static>(&self, entity: Entity) -> Option<RefMut<C>> {
        let component_type_id = C::get_type_id();
        let entity_id = entity.id();
        let components = self.components.get(&component_type_id)?;
        let component = components.get(&entity_id)?;
        let component = RefMut::map(component.borrow_mut(), |f| f.downcast_mut::<C>().unwrap());
        Some(component)
    }

    pub fn remove_component<C: Component + 'static>(&mut self, entity: Entity) {
        let component_type_id = C::get_type_id();
        if let Some(component) = self.components.get_mut(&component_type_id) {
            component.remove(&entity.id());
        }

        if let Some(signature) = self.entity_component_signatures.get_mut(&entity.id()) {
            signature.set(component_type_id, false);
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

    pub fn get_entities_with_signature(&self, signature: &Signature) -> Vec<Entity> {
        self.entities
            .iter()
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
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EntityBuilder<'a> {
    entity: Entity,
    entity_manager: &'a mut EntityManager,
}

impl<'a> EntityBuilder<'a> {
    pub fn add_component<C: Component + 'static>(&mut self, component: C) -> &mut Self {
        let component_type_id = C::get_type_id();

        println!(
            "Added Component: Entity {}, Component name {}, Component ID: {}",
            self.entity.id(),
            std::any::type_name::<C>(),
            component_type_id
        );

        self.entity_manager.add_component(self.entity, component);
        self
    }

    pub fn remove_component<C: Component + 'static>(&mut self) -> &mut Self {
        self.entity_manager.remove_component::<C>(self.entity);
        self
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
