mod asset_manager;
mod entity;
pub mod events;

pub mod derive {
    pub use macros::Component;
}

use std::time::Duration;

pub use asset_manager::AssetManager;
pub use entity::{get_next_component_type_id, Component, Entity, EntityManager, Query};
use events::EventBus;

pub struct EntityComponentSystem {
    pub entity_manager: EntityManager,
    pub systems: Vec<Box<dyn Fn(Duration, &AssetManager, &EntityManager, &mut EventBus)>>,
    pub asset_manager: AssetManager,
    pub event_bus: EventBus,
}

impl EntityComponentSystem {
    pub fn new() -> Self {
        EntityComponentSystem {
            entity_manager: entity::EntityManager::new(),
            systems: Vec::new(),
            asset_manager: AssetManager::default(),
            event_bus: EventBus::default(),
        }
    }

    pub fn add_system<T: Fn(Duration, &AssetManager, &EntityManager, &mut EventBus) + 'static>(
        &mut self,
        system: T,
    ) {
        let boxed = Box::new(system);
        self.systems.push(boxed);
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.entity_manager.update();
        for system in &mut self.systems {
            system(
                delta_time,
                &self.asset_manager,
                &self.entity_manager,
                &mut self.event_bus,
            );
        }
    }
}

impl Default for EntityComponentSystem {
    fn default() -> Self {
        Self::new()
    }
}
