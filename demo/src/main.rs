use std::cell::{RefMut, Ref};

use macroquad::prelude::*;

use rust_ecs::{Component, EntityComponentSystem, Query};

// A transform component, with the entity position.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct TransformComponent(glam::Vec2);

// A velocity component, with the entity position.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct VelocityComponent(glam::Vec2);

// The movement system uses a mutable TransformComponent and an immutable VelocityComponent,
// updating the entity position.
fn movement_system(query: Query<(RefMut<TransformComponent>, Ref<VelocityComponent>)>) {
    for (mut transform, velocity) in query.values() {
        transform.0 += velocity.0;
    }
}

// The render system prints the player position to the console.
fn render_system(query: Query<Ref<TransformComponent>>) {
    for transform in query.values() {
        draw_rectangle(transform.0.x, transform.0.y, 32.0, 32.0, BLUE);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Demo".to_string(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ecs = EntityComponentSystem::new();

    // Combining Component queries with system functions, we can add systems like this:
    ecs.add_system(|em| movement_system(em.into()));
    ecs.add_system(|em| render_system(em.into()));

    // Or manually create a system function:
    // ecs.add_system(|_| println!("Hello world!"));

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

    loop {
        clear_background(BLACK);
        ecs.update();
        next_frame().await
    }
}
