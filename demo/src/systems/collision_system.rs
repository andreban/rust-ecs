use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    rc::Rc,
};

use rust_ecs::{
    events::{EventBus, EventListener},
    systems::System,
    ComponentSignature, Entity, EntityManager,
};

use crate::{
    components::{SpriteComponent, TransformComponent, VelocityComponent},
    events::CollisionEvent,
};

pub struct CollisionSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
}

impl Default for CollisionSystem {
    fn default() -> Self {
        let mut signature = ComponentSignature::default();
        signature.require_component::<SpriteComponent>();
        signature.require_component::<TransformComponent>();
        signature.require_component::<VelocityComponent>();
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
        entity_manager: Rc<RefCell<EntityManager>>,
        event_bus: Rc<RefCell<EventBus>>,
    ) {
        let entities = self.entities.iter().collect::<Vec<_>>();
        for (i, entity_a) in entities.iter().enumerate() {
            for entity_b in &entities[i + 1..] {
                let collided = {
                    let em: Ref<'_, EntityManager> = entity_manager.borrow();
                    let transform_a = em.get_component::<TransformComponent>(**entity_a).unwrap();
                    let sprite_a = em.get_component::<SpriteComponent>(**entity_a).unwrap();
                    let transform_b = em.get_component::<TransformComponent>(**entity_b).unwrap();
                    let sprite_b = em.get_component::<SpriteComponent>(**entity_b).unwrap();

                    let a = transform_a.0;
                    let b = transform_b.0;
                    let a_width = sprite_a.dst_size.x as f32;
                    let a_height = sprite_a.dst_size.y as f32;
                    let b_width = sprite_b.dst_size.x as f32;
                    let b_height = sprite_b.dst_size.y as f32;

                    a.x < b.x + b_width
                        && a.x + a_width > b.x
                        && a.y < b.y + b_height
                        && a.y + a_height > b.y
                };

                if collided {
                    event_bus.borrow_mut().emit(
                        entity_manager.clone(),
                        CollisionEvent { entity_a: **entity_a, entity_b: **entity_b },
                    );
                }
            }
        }
    }
}

impl EventListener for CollisionSystem {}
