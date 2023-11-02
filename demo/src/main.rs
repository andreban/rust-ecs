mod components;
mod systems;

use std::{
    thread,
    time::{Duration, Instant},
};

use components::{SpriteComponent, TransformComponent, VelocityComponent};
use macroquad::prelude::*;

use rust_ecs::{Entity, EntityComponentSystem};

struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
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
    ecs.add_system(systems::render_system());
    ecs.add_system(systems::collision_system());
    ecs.add_system(systems::movement_system());
    ecs.add_system(systems::debug_system());

    // Create entities with components.
    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(glam::Vec2::ZERO))
        .add_component(VelocityComponent(Vec2::new(50.0, 0.0)))
        .add_component(SpriteComponent::new("tank".to_string(), 32, 32));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::new(100.0, 0.0)))
        .add_component(VelocityComponent(Vec2::new(-50.0, 0.0)))
        .add_component(SpriteComponent::new("truck".to_string(), 32, 32));
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ecs = EntityComponentSystem::new();
    let rate = 1000 / 60;

    setup(&mut ecs).await;

    let mut time = Instant::now();
    loop {
        if time.elapsed().as_millis() < rate {
            thread::sleep(Duration::from_millis(
                (rate - time.elapsed().as_millis()) as u64,
            ));
            continue;
        }

        clear_background(BLACK);
        ecs.update(time.elapsed());
        time = Instant::now();
        next_frame().await
    }
}
