mod components;

use std::{
    cell::{Ref, RefMut},
    time::Instant,
};

use components::{SpriteComponent, TransformComponent, VelocityComponent};
use macroquad::prelude::*;

use rust_ecs::{AssetManager, EntityComponentSystem, Query};

// The movement system uses a mutable TransformComponent and an immutable VelocityComponent,
// updating the entity position.
fn movement_system(
    delta_time: f32,
    query: Query<(RefMut<TransformComponent>, Ref<VelocityComponent>)>,
) {
    for (mut transform, velocity) in query.values() {
        transform.0 += velocity.0 * delta_time;
    }
}

// The render system prints the player position to the console.
fn render_system(
    asset_manager: &AssetManager,
    query: Query<(Ref<TransformComponent>, Ref<SpriteComponent>)>,
) {
    for (transform, sprite) in query.values() {
        let texture = asset_manager.get_texture(&sprite.sprite_name).unwrap();
        draw_texture(texture, transform.0.x, transform.0.y, WHITE);
    }
}

fn window_conf() -> Conf {
    Conf { window_title: "Demo".to_string(), ..Default::default() }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ecs = EntityComponentSystem::new();

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
    ecs.add_system(|delta_time, _, em| movement_system(delta_time, em.into()));
    ecs.add_system(|_, am, em| render_system(am, em.into()));

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

    let mut time = Instant::now();
    loop {
        clear_background(BLACK);
        ecs.update(time.elapsed().as_secs_f32());
        time = Instant::now();
        next_frame().await
    }
}
