pub mod entity;

pub mod derive {
    pub use macros::Component;
}

pub struct EntityComponentSystem {
    pub entity_manager: entity::EntityManager,
    systems: Vec<Box<dyn FnMut(&mut entity::EntityManager)>>,
}

impl EntityComponentSystem {
    pub fn new() -> Self {
        EntityComponentSystem { entity_manager: entity::EntityManager::new(), systems: Vec::new() }
    }

    pub fn add_system<F: FnMut(&mut entity::EntityManager) + 'static>(&mut self, system: F) {
        self.systems.push(Box::new(system));
    }

    pub fn update(&mut self) {
        self.entity_manager.update();
        for system in &mut self.systems {
            println!("Running system... ");
            system(&mut self.entity_manager);
        }
    }
}

impl Default for EntityComponentSystem {
    fn default() -> Self {
        Self::new()
    }
}
