pub mod component;
pub mod entity;
pub mod system;

use std::collections::{HashMap, HashSet};

use fixedbitset::FixedBitSet;

use self::{
    component::{Component, ComponentTypeId},
    entity::{get_next_entity_id, Entity, EntityId},
    system::System,
};

pub type Signature = FixedBitSet;

// Component storage
pub struct Registry {
    components: HashMap<ComponentTypeId, HashMap<EntityId, Box<dyn Component>>>,
    entities: Vec<Entity>,
    entities_to_spawn: HashSet<Entity>,
    entities_to_despawn: HashSet<Entity>,
    entity_component_signatures: HashMap<EntityId, Signature>,
    systems: HashMap<usize, Box<dyn System>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            components: HashMap::new(),
            entities: Vec::new(),
            entities_to_spawn: HashSet::new(),
            entities_to_despawn: HashSet::new(),
            entity_component_signatures: HashMap::new(),
            systems: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        // Add entities waiting to be created to systems.
        while let Some(entity) = self.entities.pop() {
            self.entities.push(entity);
            self.add_entity_to_system(entity);
        }

        // TODO: Remove entities waiting to be killed from systems.
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let id: EntityId = get_next_entity_id();
        let entity = Entity::new(id);
        self.entities_to_spawn.insert(entity);
        EntityBuilder {
            entity: entity,
            entity_manager: self,
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        // TODO: Remove components...
        self.entities.retain(|e| *e != entity);
    }

    pub fn add_component<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        let component_type_id = C::get_type_id();
        let component = Box::new(component);

        // Update the entity signature to indicate that the entity has the component.
        self.entity_component_signatures
            .entry(entity.id())
            .or_insert_with(|| FixedBitSet::with_capacity(32))
            .set(component_type_id, true);

        // Add the component to the component storage.
        self.components
            .entry(component_type_id)
            .or_insert_with(HashMap::new)
            .insert(entity.id(), component);

        self.add_entity_to_system(entity)
    }

    pub fn get_component<C: Component + 'static>(&self, entity: Entity) -> Option<&C> {
        let component_type_id = C::get_type_id();
        let components = self.components.get(&component_type_id)?;
        components
            .get(&entity.id())
            .map(|c| c.as_any().downcast_ref::<C>().unwrap())
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

    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        let system_type_id = S::get_type_id();
        let system = Box::new(system);
        self.systems.insert(system_type_id, system);
    }

    pub fn remove_system<S: System + 'static>(&mut self) {
        let system_type_id = S::get_type_id();
        self.systems.remove(&system_type_id);
    }

    pub fn has_system<S: System + 'static>(&self) -> bool {
        let system_type_id = S::get_type_id();
        self.systems.contains_key(&system_type_id)
    }

    pub fn get_system<S: System + 'static>(&self) -> Option<&S> {
        let system_type_id = S::get_type_id();
        self.systems
            .get(&system_type_id)
            .map(|s| s.as_any().downcast_ref::<S>().unwrap())
    }

    fn add_entity_to_system(&mut self, entity: Entity) {
        let Some(entity_signature) = self.entity_component_signatures.get(&entity.id()) else {
            return;
        };

        for system in self.systems.values_mut() {
            if system.signature() & entity_signature == *system.signature() {
                system.add_entity(entity);
            }
        }
    }

    pub fn query<C: Component + 'static>(&self) -> Vec<&C> {
        let component_type_id = C::get_type_id();
        let components = self.components.get(&component_type_id).unwrap();
        components
            .iter()
            .map(|(_, c)| c.as_any().downcast_ref::<C>().unwrap())
            .collect()
    }

    pub fn query_mut<C: Component + 'static>(&mut self) -> Vec<&mut C> {
        let component_type_id = C::get_type_id();
        let components = self.components.get_mut(&component_type_id).unwrap();
        components
            .iter_mut()
            .map(|(_, c)| c.as_any_mut().downcast_mut::<C>().unwrap())
            .collect()
    }
}

pub struct EntityBuilder<'a> {
    entity: Entity,
    entity_manager: &'a mut Registry,
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

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
