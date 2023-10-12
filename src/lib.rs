mod entity;

pub mod derive {
    pub use macros::Component;
}

pub use entity::{get_next_component_type_id, Component, Entity, EntityManager, Query};

pub struct EntityComponentSystem {
    pub entity_manager: EntityManager,
    pub systems: Vec<Box<dyn Fn(&EntityManager)>>,
}

impl EntityComponentSystem {
    pub fn new() -> Self {
        EntityComponentSystem { entity_manager: entity::EntityManager::new(), systems: Vec::new() }
    }

    pub fn add_system<T: Fn(&EntityManager) + 'static>(&mut self, system: T) {
        let boxed = Box::new(system);
        self.systems.push(boxed);
    }

    pub fn update(&mut self) {
        self.entity_manager.update();
        for system in &mut self.systems {
            system(&self.entity_manager);
        }
    }
}

impl Default for EntityComponentSystem {
    fn default() -> Self {
        Self::new()
    }
}
