use crate::components::ProjectileComponent;
use rust_ecs::events::{EventBus, EventListener};
use rust_ecs::systems::System;
use rust_ecs::{AssetManager, ComponentSignature, Entity, EntityManager, Resources};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;

pub struct ProjectileLifecycleSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
}

impl Default for ProjectileLifecycleSystem {
    fn default() -> Self {
        let mut signature = ComponentSignature::default();
        signature.require_component::<ProjectileComponent>();

        Self { signature, entities: Default::default() }
    }
}

impl EventListener for ProjectileLifecycleSystem {}

impl System for ProjectileLifecycleSystem {
    fn signature(&self) -> &ComponentSignature {
        &self.signature
    }

    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn update(
        &self,
        _delta_time: Duration,
        _asset_manager: &AssetManager,
        entity_manager: EntityManager,
        _event_bus: Rc<RefCell<EventBus>>,
        _resources: Rc<RefCell<Resources>>,
    ) {
        for entity in &self.entities {
            let expired = {
                let projectile_component =
                    entity_manager.get_component::<ProjectileComponent>(entity).unwrap();
                let projectile_component = projectile_component.borrow();
                if projectile_component.created.elapsed().unwrap()
                    >= projectile_component.max_duration
                {
                    true
                } else {
                    false
                }
            };
            if expired {
                entity_manager.destroy_entity(*entity);
            }
        }
    }
}
