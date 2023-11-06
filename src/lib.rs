mod asset_manager;
mod entity;
pub mod events;
pub mod systems;

pub mod derive {
    pub use macros::Component;
}

use std::{cell::RefCell, rc::Rc, time::Duration};

pub use asset_manager::AssetManager;
pub use entity::{get_next_component_type_id, Component, Entity, EntityManager, Query, Signature};
use events::EventBus;
use systems::System;

pub struct EntityComponentSystem<'a> {
    pub entity_manager: Rc<RefCell<EntityManager>>,
    pub systems: Vec<Box<dyn System>>,
    pub asset_manager: AssetManager,
    pub event_bus: Rc<RefCell<EventBus<'a>>>,
}

impl<'a> EntityComponentSystem<'a> {
    pub fn new() -> Self {
        EntityComponentSystem {
            entity_manager: Rc::new(RefCell::new(entity::EntityManager::new())),
            systems: Vec::new(),
            asset_manager: AssetManager::default(),
            event_bus: Rc::new(RefCell::new(EventBus::default())),
        }
    }

    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        let boxed: Box<dyn System> = Box::new(system);
        self.systems.push(boxed);
    }

    pub fn update<'b>(&'a self, delta_time: Duration) {
        // for entity in self.entity_manager.borrow().entities_to_despawn.iter() {
        //     for system in &self.systems {
        //         system.remove_entity(*entity);
        //     }
        // }

        self.entity_manager.borrow_mut().update();
        self.event_bus.borrow_mut().clear();

        for system in &self.systems {
            for type_id in system.get_event_type() {
                self.event_bus.borrow_mut().subscribe_type(*type_id, system);
            }
        }

        for system in &self.systems {
            system.update(
                delta_time,
                &self.asset_manager,
                self.entity_manager.clone(),
                self.event_bus.clone(),
            );
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.borrow_mut().create_entity()
    }

    pub fn add_component<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        let mut em = self.entity_manager.borrow_mut();
        em.add_component(entity, component);
        let signature = em.get_signature(entity).unwrap();

        for system in &mut self.systems {
            if system.signature().is_subset(signature) {
                system.add_entity(entity);
            }
        }
    }

    pub fn remove_component<C: Component + 'static>(&mut self, entity: Entity) {
        let mut em = self.entity_manager.borrow_mut();
        em.remove_component::<C>(entity);
        let signature = em.get_signature(entity).unwrap();
        for system in &mut self.systems {
            if !system.signature().is_subset(signature) {
                system.remove_entity(entity);
            }
        }
    }
}

impl<'a> Default for EntityComponentSystem<'a> {
    fn default() -> Self {
        Self::new()
    }
}
