mod asset_manager;
mod entity;
pub mod events;
pub mod systems;

pub mod derive {
    pub use macros::Component;
}

use std::time::Duration;

pub use asset_manager::AssetManager;
pub use entity::{get_next_component_type_id, Component, Entity, EntityManager, Query};
use events::EventBus;

pub struct EntityComponentSystem {
    pub entity_manager: EntityManager,
    pub systems: Vec<systems::System>,
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

    pub fn add_system(&mut self, system: systems::System) {
        self.systems.push(system);
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.entity_manager.update();
        self.event_bus.clear();

        for system in &mut self.systems {
            system.setup_listeners(&mut self.event_bus);
        }

        for system in &mut self.systems {
            system.update(
                delta_time,
                &self.asset_manager,
                &mut self.entity_manager,
                &mut self.event_bus,
            );
        }

        self.event_bus.clear();
    }
}

impl Default for EntityComponentSystem {
    fn default() -> Self {
        Self::new()
    }
}
