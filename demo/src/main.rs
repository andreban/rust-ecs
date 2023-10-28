mod components;
mod systems;

use std::time::Instant;

use components::{SpriteComponent, TransformComponent, VelocityComponent};
use macroquad::prelude::*;

use rust_ecs::EntityComponentSystem;
use systems::{collision_system, debug_system, movement_system, render_system};

struct CollisionEvent {
    pub entity_a: u32,
    pub entity_b: u32,
}

fn window_conf() -> Conf {
    Conf { window_title: "Demo".to_string(), ..Default::default() }
}

pub async fn setup(ecs: &mut EntityComponentSystem) {
    // Load assets.
    ecs.asset_manager
        .load_texture("tank", "assets/images/tank-panther-right.png")
        .await
        .unwrap();

    ecs.asset_manager
        .load_texture("truck", "assets/images/truck-ford-right.png")
        .await
        .unwrap();

    // Combining Component queries with system functions, we can add systems like this:
    ecs.add_system(|delta_time, _, em, _| movement_system(delta_time, em.into()));
    ecs.add_system(|_, am, em, _| render_system(am, em.into()));
    ecs.add_system(|_, _, em, eb| collision_system(eb, em.into()));
    ecs.add_system(|_, _, _, eb| debug_system(eb));

    // Create entities with components.
    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(glam::Vec2::ZERO))
        .add_component(VelocityComponent(Vec2::new(50.0, 0.0)))
        .add_component(SpriteComponent::new("tank".to_string(), 32, 32));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::new(100.0, 100.0)))
        .add_component(VelocityComponent(Vec2::new(0.0, 50.0)))
        .add_component(SpriteComponent::new("truck".to_string(), 32, 32));
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ecs = EntityComponentSystem::new();

    setup(&mut ecs).await;

    let mut time = Instant::now();
    loop {
        clear_background(BLACK);

        ecs.event_bus
            .emit(CollisionEvent { entity_a: 0, entity_b: 1 });

        ecs.update(time.elapsed());
        time = Instant::now();
        next_frame().await
    }
}
