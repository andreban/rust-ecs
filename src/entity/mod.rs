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

pub type Signature = FixedBitSet;

// Component storage
pub struct EntityManager {
    components: HashMap<ComponentTypeId, HashMap<EntityId, RefCell<Box<dyn Any>>>>,
    entities: Vec<Entity>,
    entities_to_spawn: HashSet<Entity>,
    _entities_to_despawn: HashSet<Entity>,
    entity_component_signatures: HashMap<EntityId, Signature>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            components: HashMap::new(),
            entities: Vec::new(),
            entities_to_spawn: HashSet::new(),
            _entities_to_despawn: HashSet::new(),
            entity_component_signatures: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        // Add entities waiting to be created to systems.
        for entity in &mut self.entities_to_spawn.drain() {
            self.entities.push(entity);
        }

        // TODO: Remove entities waiting to be killed from systems.
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

    pub fn get_component<C: Component + 'static>(&self, entity: Entity) -> Option<&C> {
        let component_type_id = C::get_type_id();
        let components = self.components.get(&component_type_id)?;
        components
            .get(&entity.id())
            .map(|c| (c as &dyn Any).downcast_ref::<C>().unwrap())
    }

    pub fn get_component_2<C: Component + 'static>(&self, id: EntityId) -> Option<Ref<C>> {
        let component_type_id = C::get_type_id();
        let components = self.components.get(&component_type_id)?;
        let component = components.get(&id)?;
        let component = Ref::map(component.borrow(), |f| f.downcast_ref::<C>().unwrap());
        Some(component)
    }

    pub fn get_component_mut<C: Component + 'static>(&self, id: EntityId) -> Option<RefMut<C>> {
        let component_type_id = C::get_type_id();
        let components = self.components.get(&component_type_id)?;
        let component = components.get(&id)?;
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
