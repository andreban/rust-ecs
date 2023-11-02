use macroquad::{prelude::WHITE, texture::draw_texture};
use rust_ecs::systems::{System, SystemBuilder};

use crate::{
    components::{SpriteComponent, TransformComponent, VelocityComponent},
    CollisionEvent,
};

// The movement system uses a mutable TransformComponent and an immutable VelocityComponent,
// updating the entity position.
pub fn movement_system() -> System {
    SystemBuilder::new()
        .require_component::<TransformComponent>()
        .require_component::<VelocityComponent>()
        .with_update(|entities, delta_time, _, em, _| {
            for entity in entities {
                let mut transform = em.get_component_mut::<TransformComponent>(*entity).unwrap();
                let velocity = em.get_component::<VelocityComponent>(*entity).unwrap();
                transform.0 += velocity.0 * delta_time.as_secs_f32();
            }
        })
        .build()
}

// The render system prints the player position to the console.
pub fn render_system() -> System {
    SystemBuilder::new()
        .require_component::<TransformComponent>()
        .require_component::<SpriteComponent>()
        .with_update(|entities, _, am, em, _| {
            for entity in entities {
                let transform = em.get_component::<TransformComponent>(*entity).unwrap();
                let sprite = em.get_component::<SpriteComponent>(*entity).unwrap();
                let texture = am.get_texture(&sprite.sprite_name).unwrap();
                draw_texture(texture, transform.0.x, transform.0.y, WHITE);
            }
        })
        .build()
}

// Detects collision between entities.
pub fn collision_system() -> System {
    SystemBuilder::new()
        .require_component::<TransformComponent>()
        .require_component::<SpriteComponent>()
        .with_update(|entities, _, _, em, event_bus| {
            let entities = entities.to_vec();
            for (i, entity_a) in entities.iter().enumerate() {
                let transform_a = em.get_component::<TransformComponent>(*entity_a).unwrap();
                let sprite_a = em.get_component::<SpriteComponent>(*entity_a).unwrap();
                for entity_b in &entities[i + 1..] {
                    let transform_b = em.get_component::<TransformComponent>(*entity_b).unwrap();
                    let sprite_b = em.get_component::<SpriteComponent>(*entity_b).unwrap();

                    let a = transform_a.0;
                    let b = transform_b.0;
                    let a_width = sprite_a.width as f32;
                    let a_height = sprite_a.height as f32;
                    let b_width = sprite_b.width as f32;
                    let b_height = sprite_b.height as f32;

                    if a.x < b.x + b_width
                        && a.x + a_width > b.x
                        && a.y < b.y + b_height
                        && a.y + a_height > b.y
                    {
                        event_bus.emit(CollisionEvent { entity_a: *entity_a, entity_b: *entity_b });
                    }
                }
            }
        })
        .build()
}

pub fn debug_system() -> System {
    SystemBuilder::new()
        .with_setup_listeners(|event_bus| {
            event_bus.add_listener(|event: &CollisionEvent| {
                println!(
                    "Collision between entities {:?} and {:?}",
                    event.entity_a, event.entity_b
                );
            });
        })
        .build()
}
