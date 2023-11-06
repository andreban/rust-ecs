use std::{cell::RefCell, collections::HashSet, rc::Rc};

use rust_ecs::{
    events::{EventBus, EventListener},
    systems::System,
    Component, Entity, EntityManager, Signature,
};

use crate::components::{TransformComponent, VelocityComponent};

// The movement system uses a mutable TransformComponent and an immutable VelocityComponent,
// updating the entity position.
pub struct MovementSystem {
    signature: Signature,
    entities: HashSet<Entity>,
}

impl Default for MovementSystem {
    fn default() -> Self {
        let mut signature = Signature::with_capacity(32);
        signature.set(TransformComponent::get_type_id(), true);
        signature.set(VelocityComponent::get_type_id(), true);
        Self { signature, entities: Default::default() }
    }
}

impl EventListener for MovementSystem {
    fn on_event(&self, _em: Rc<RefCell<EntityManager>>, _event: &rust_ecs::events::Event) {
        todo!()
    }
}

impl System for MovementSystem {
    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    // fn subscribe_events(&self, _: Rc<RefCell<EventBus>>) {}

    fn update(
        &self,
        delta_time: std::time::Duration,
        _asset_manager: &rust_ecs::AssetManager,
        entity_manager: Rc<RefCell<EntityManager>>,
        _event_bus: Rc<RefCell<EventBus>>,
    ) {
        for entity in &self.entities {
            let em = entity_manager.borrow_mut();
            let mut transform = em.get_component_mut::<TransformComponent>(*entity).unwrap();
            let velocity = em.get_component::<VelocityComponent>(*entity).unwrap();
            transform.0 += velocity.0 * delta_time.as_secs_f32();
        }
    }
}
