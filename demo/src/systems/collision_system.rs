use rust_ecs::systems::{System, SystemBuilder};

use crate::{
    components::{SpriteComponent, TransformComponent, VelocityComponent},
    events::CollisionEvent,
};

// Detects collision between entities.
pub fn create_collision_system() -> System {
    SystemBuilder::new()
        .require_component::<TransformComponent>()
        .require_component::<VelocityComponent>()
        .require_component::<SpriteComponent>()
        .with_update(|entities, _, _, em, event_bus| {
            let entities = entities.to_vec();
            for (i, entity_a) in entities.iter().enumerate() {
                for entity_b in &entities[i + 1..] {
                    let collided = {
                        let transform_a =
                            em.get_component::<TransformComponent>(*entity_a).unwrap();
                        let sprite_a = em.get_component::<SpriteComponent>(*entity_a).unwrap();
                        let transform_b =
                            em.get_component::<TransformComponent>(*entity_b).unwrap();
                        let sprite_b = em.get_component::<SpriteComponent>(*entity_b).unwrap();

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
                        event_bus.emit(
                            em,
                            CollisionEvent {
                                entity_a: entity_a.clone(),
                                entity_b: entity_b.clone(),
                            },
                        );
                    }
                }
            }
        })
        .build()
}
