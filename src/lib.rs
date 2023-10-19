mod asset_manager;
mod entity;

pub mod derive {
    pub use macros::Component;
}

pub use asset_manager::AssetManager;
pub use entity::{get_next_component_type_id, Component, Entity, EntityManager, Query};

pub struct EntityComponentSystem {
    pub entity_manager: EntityManager,
    pub systems: Vec<Box<dyn Fn(f32, &AssetManager, &EntityManager)>>,
    pub asset_manager: AssetManager,
}

impl EntityComponentSystem {
    pub fn new() -> Self {
        EntityComponentSystem {
            entity_manager: entity::EntityManager::new(),
            systems: Vec::new(),
            asset_manager: AssetManager::default(),
        }
    }

    pub fn add_system<T: Fn(f32, &AssetManager, &EntityManager) + 'static>(&mut self, system: T) {
        let boxed = Box::new(system);
        self.systems.push(boxed);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.entity_manager.update();
        for system in &mut self.systems {
            system(delta_time, &self.asset_manager, &self.entity_manager);
        }
    }
}

impl Default for EntityComponentSystem {
    fn default() -> Self {
        Self::new()
    }
}
