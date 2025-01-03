use std::{cell::RefCell, collections::HashSet, rc::Rc};

use rust_ecs::{
    events::{EventBus, EventListener},
    systems::System,
    ComponentSignature, Entity, EntityManager,
};

use crate::components::{TransformComponent, VelocityComponent};

// The movement system uses a mutable TransformComponent and an immutable VelocityComponent,
// updating the entity position.
pub struct MovementSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
}

impl Default for MovementSystem {
    fn default() -> Self {
        let mut signature = ComponentSignature::default();
        signature.require_component::<TransformComponent>();
        signature.require_component::<VelocityComponent>();
        Self { signature, entities: Default::default() }
    }
}

impl EventListener for MovementSystem {}

impl System for MovementSystem {
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
        delta_time: std::time::Duration,
        _asset_manager: &rust_ecs::AssetManager,
        entity_manager: EntityManager,
        _event_bus: Rc<RefCell<EventBus>>,
        _resources: std::rc::Rc<std::cell::RefCell<rust_ecs::Resources>>,
    ) {
        for entity in &self.entities {
            let transform = entity_manager.get_component::<TransformComponent>(entity).unwrap();
            let velocity = entity_manager.get_component::<VelocityComponent>(entity).unwrap();

            let mut transform = transform.borrow_mut();
            let velocity = velocity.borrow();
            transform.0 += velocity.0 * delta_time.as_secs_f32();
        }
    }
}
