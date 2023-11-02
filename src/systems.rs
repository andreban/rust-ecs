use std::time::Duration;

use crate::{entity::Signature, events::EventBus, AssetManager, Component, Entity, EntityManager};

pub struct SystemBuilder {
    signature: Signature,
    update_func:
        Option<Box<dyn Fn(&[Entity], Duration, &AssetManager, &mut EntityManager, &mut EventBus)>>,
    setup_listeners_func: Option<Box<dyn Fn(&mut EventBus)>>,
}

impl SystemBuilder {
    pub fn new() -> Self {
        SystemBuilder {
            signature: Signature::with_capacity(32),
            update_func: None,
            setup_listeners_func: None,
        }
    }

    pub fn with_update(
        mut self,
        func: impl Fn(&[Entity], Duration, &AssetManager, &mut EntityManager, &mut EventBus) + 'static,
    ) -> Self {
        self.update_func = Some(Box::new(func));
        self
    }

    pub fn with_setup_listeners(mut self, func: impl Fn(&mut EventBus) + 'static) -> Self {
        self.setup_listeners_func = Some(Box::new(func));
        self
    }

    pub fn require_component<T: Component>(mut self) -> Self {
        self.signature.set(T::get_type_id(), true);
        self
    }

    pub fn build(self) -> System {
        System {
            signature: self.signature,
            update_func: self.update_func,
            setup_listeners_func: self.setup_listeners_func,
            entities: Vec::new(),
        }
    }
}

pub struct System {
    pub signature: Signature,
    pub update_func:
        Option<Box<dyn Fn(&[Entity], Duration, &AssetManager, &mut EntityManager, &mut EventBus)>>,
    pub setup_listeners_func: Option<Box<dyn Fn(&mut EventBus)>>,
    pub entities: Vec<Entity>,
}

impl System {
    pub fn setup_listeners(&mut self, event_bus: &mut EventBus) {
        if let Some(func) = &mut self.setup_listeners_func {
            func(event_bus);
        }
    }

    pub fn update(
        &mut self,
        delta_time: Duration,
        asset_manager: &AssetManager,
        entity_manager: &mut EntityManager,
        event_bus: &mut EventBus,
    ) {
        if let Some(func) = &self.update_func {
            let entities = entity_manager.get_entities_with_signature(&self.signature);
            func(
                &entities,
                delta_time,
                asset_manager,
                entity_manager,
                event_bus,
            );
        }
    }
}
