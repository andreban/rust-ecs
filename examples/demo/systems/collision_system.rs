use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
};

use rust_ecs::{
    events::{EventBus, EventListener},
    systems::System,
    ComponentSignature, Entity, EntityManager,
};

use crate::{
    components::{Box2dColliderComponent, TransformComponent},
    events::CollisionEvent,
};

pub struct CollisionSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
}

impl Default for CollisionSystem {
    fn default() -> Self {
        let mut signature = ComponentSignature::default();
        signature.require_component::<TransformComponent>();
        signature.require_component::<Box2dColliderComponent>();
        Self { signature, entities: Default::default() }
    }
}

impl System for CollisionSystem {
    fn signature(&self) -> &ComponentSignature {
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
        _delta_time: std::time::Duration,
        _asset_manager: &rust_ecs::AssetManager,
        em: EntityManager,
        event_bus: Rc<RefCell<EventBus>>,
        _resources: std::rc::Rc<std::cell::RefCell<rust_ecs::Resources>>,
    ) {
        let entities = self.entities.iter().collect::<Vec<_>>();
        for (i, entity_a) in entities.iter().enumerate() {
            for entity_b in &entities[i + 1..] {
                let collided = {
                    let transform_a = em
                        .get_component::<TransformComponent>(entity_a)
                        .unwrap();
                    let box_a = em
                        .get_component::<Box2dColliderComponent>(entity_a)
                        .unwrap();
                    let transform_b = em
                        .get_component::<TransformComponent>(entity_b)
                        .unwrap();
                    let box_b = em
                        .get_component::<Box2dColliderComponent>(entity_b)
                        .unwrap();

                    let transform_a = transform_a.borrow();
                    let box_a = box_a.borrow();
                    let transform_b = transform_b.borrow();
                    let box_b = box_b.borrow();

                    let a = transform_a.0;
                    let b = transform_b.0;
                    let a_width = box_a.size.x;
                    let a_height = box_a.size.y;
                    let b_width = box_b.size.x;
                    let b_height = box_b.size.y;

                    a.x < b.x + b_width
                        && a.x + a_width > b.x
                        && a.y < b.y + b_height
                        && a.y + a_height > b.y
                };

                if collided {
                    event_bus.borrow_mut().emit(
                        em.clone(),
                        CollisionEvent { entity_a: **entity_a, entity_b: **entity_b },
                    );
                }
            }
        }
    }
}

impl EventListener for CollisionSystem {}
