use std::cell::{Ref, RefMut};

use glam::Vec2;
use rust_ecs::systems::{System, SystemBuilder};
use rust_ecs::{Component, EntityComponentSystem, Query};
use std::time::Duration;

// A transform component, with the entity position.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct TransformComponent(glam::Vec2);

// A velocity component, with the entity position.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct VelocityComponent(glam::Vec2);

// The movement system uses a mutable TransformComponent and an immutable VelocityComponent,
// updating the entity position.
fn movement_system() -> System {
    SystemBuilder::new()
        .require_component::<TransformComponent>()
        .require_component::<VelocityComponent>()
        .with_update(|entities, _, _, em, _| {
            for entity in entities {
                let mut transform = em.get_component_mut::<TransformComponent>(*entity).unwrap();
                let velocity = em.get_component::<VelocityComponent>(*entity).unwrap();
                transform.0 += velocity.0;
            }
        })
        .build()
}

// The render system prints the player position to the console.
fn render_system() -> System {
    SystemBuilder::new()
        .require_component::<TransformComponent>()
        .with_update(|entities, _, _, em, _| {
            for entity in entities {
                let transform = em.get_component::<TransformComponent>(*entity).unwrap();
                println!("Position: {:?}", transform.0);
            }
        })
        .build()
}

fn main() {
    let mut ecs = EntityComponentSystem::new();

    // Combining Component queries with system functions, we can add systems like this:
    ecs.add_system(movement_system());
    ecs.add_system(render_system());

    // Or manually create a system function:
    ecs.add_system(
        SystemBuilder::new()
            .with_update(|_, _, _, _, _| {
                println!("Hello from a system function!");
            })
            .build(),
    );

    // Create entities with components.
    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::ZERO))
        .add_component(VelocityComponent(Vec2::ONE));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::new(100.0, 100.0)))
        .add_component(VelocityComponent(Vec2::ONE))
        .entity();

    // Update the ECS - will run all systems (usually in a loop, but calling once as an example)
    ecs.update(Duration::from_secs_f32(1.0));
}
