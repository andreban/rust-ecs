mod asset_manager;
mod component_signature;
mod entity_manager;
pub mod events;
mod resources;
pub mod systems;

pub mod derive {
    pub use macros::Component;
}

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    time::Duration,
};

pub use asset_manager::AssetManager;
pub use component_signature::ComponentSignature;
pub use entity_manager::{get_next_component_type_id, Component, Entity, EntityManager};
use events::EventBus;
pub use resources::Resources;
use systems::System;

pub struct EntityComponentSystem {
    entity_manager: EntityManager,
    systems: Vec<Rc<RefCell<Box<dyn System>>>>,
    asset_manager: AssetManager,
    event_bus: Rc<RefCell<EventBus>>,
    resources: Rc<RefCell<Resources>>,
}

impl EntityComponentSystem {
    pub fn new() -> Self {
        EntityComponentSystem {
            entity_manager: entity_manager::EntityManager::new(),
            systems: Vec::new(),
            asset_manager: AssetManager::default(),
            event_bus: Rc::new(RefCell::new(EventBus::default())),
            resources: Rc::new(RefCell::new(Resources::default())),
        }
    }

    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        let boxed: Rc<RefCell<Box<dyn System>>> = Rc::new(RefCell::new(Box::new(system)));
        self.systems.push(boxed);
    }

    pub fn update(&self, delta_time: Duration) {
        {
            let mut em = self.entity_manager.inner.borrow_mut();
            {
                for entity in em.entities_to_spawn.iter() {
                    let entity_signature = em.get_signature(*entity).unwrap();
                    for system in &self.systems {
                        let mut system = system.borrow_mut();
                        if system.signature().is_subset(entity_signature) {
                            system.add_entity(*entity);
                        }
                    }
                }
            }

            for entity in em.entities_to_despawn.iter() {
                for system in &self.systems {
                    let mut system = system.borrow_mut();
                    system.remove_entity(*entity);
                }
            }

            em.update();
        }
        self.event_bus.borrow_mut().clear();

        for system in &self.systems {
            for type_id in system.borrow().get_event_type() {
                let mut eb = self.event_bus.borrow_mut();
                eb.subscribe_type(*type_id, system.clone());
            }
        }

        for system in &self.systems {
            system.borrow().update(
                delta_time,
                &self.asset_manager,
                self.entity_manager.clone(),
                self.event_bus.clone(),
                self.resources.clone(),
            );
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.inner.borrow_mut().create_entity()
    }

    pub fn add_component<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        let mut em = self.entity_manager.inner.borrow_mut();
        em.add_component(entity, component);
        let signature = em.get_signature(entity).unwrap();

        for system in &mut self.systems {
            if system.borrow().signature().is_subset(signature) {
                system.borrow_mut().add_entity(entity);
            }
        }
    }

    pub fn remove_component<C: Component + 'static>(&mut self, entity: Entity) {
        let mut em = self.entity_manager.inner.borrow_mut();
        em.remove_component::<C>(entity);
        let signature = em.get_signature(entity).unwrap();
        for system in &mut self.systems {
            if !system.borrow().signature().is_subset(signature) {
                system.borrow_mut().remove_entity(entity);
            }
        }
    }

    pub fn asset_manager(&self) -> &AssetManager {
        &self.asset_manager
    }

    pub fn asset_manager_mut(&mut self) -> &mut AssetManager {
        &mut self.asset_manager
    }

    pub fn resources(&self) -> Ref<Resources> {
        self.resources.borrow()
    }

    pub fn resources_mut(&self) -> RefMut<Resources> {
        self.resources.borrow_mut()
    }

    pub fn event_bus_cloned(&self) -> Rc<RefCell<EventBus>> {
        self.event_bus.clone()
    }

    pub fn entity_manager(&self) -> EntityManager {
        self.entity_manager.clone()
    }
}

impl Default for EntityComponentSystem {
    fn default() -> Self {
        Self::new()
    }
}
