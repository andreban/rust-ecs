mod components;
mod events;
mod resources;
mod systems;
mod tilemap;

use std::{
    cell::RefCell,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use components::{
    AnimationComponent, CameraFollowComponent, KeyboardControlComponent, SpriteComponent,
    TransformComponent, VelocityComponent,
};
use events::KeyboardEvent;
use macroquad::prelude::*;

use resources::{Camera, MapDimensions};
use rust_ecs::{events::EventBus, EntityComponentSystem, EntityManager};
use tilemap::load_map;

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

    ecs.asset_manager
        .load_texture("jungle", "assets/tilemaps/jungle.png")
        .await
        .unwrap();

    ecs.asset_manager
        .load_texture("chopper", "assets/images/chopper-spritesheet.png")
        .await
        .unwrap();

    // Combining Component queries with system functions, we can add systems like this:
    ecs.add_system(systems::RenderSystem::default());
    ecs.add_system(systems::CollisionSystem::default());
    ecs.add_system(systems::MovementSystem::default());
    ecs.add_system(systems::DamageSystem::default());
    ecs.add_system(systems::KeyboardMovementSystem::default());
    ecs.add_system(systems::AnimationSystem::default());
    ecs.add_system(systems::CameraFollowSystem::default());

    let tiles = load_map("assets/tilemaps/jungle.map").unwrap();
    let tile_scale = 2;
    for tile in tiles {
        let tile_x = (tile.x * 32 * tile_scale) as f32;
        let tile_y = (tile.y * 32 * tile_scale) as f32;

        let tile_src_y = (tile.sprite_id / 10 * 32) as f32;
        let tile_src_x = (tile.sprite_id % 10 * 32) as f32;

        let entity = ecs.create_entity();
        ecs.add_component(entity, TransformComponent(Vec2::new(tile_x, tile_y)));
        ecs.add_component(
            entity,
            SpriteComponent::new(
                "jungle".to_string(),
                Vec2::new(32.0 * tile_scale as f32, 32.0 * tile_scale as f32),
            )
            .with_src_rect(Rect::new(tile_src_x + 0.5, tile_src_y + 0.5, 31.0, 31.0)),
        )
    }

    // Create entities with components.
    let entity = ecs.create_entity();
    ecs.add_component(entity, TransformComponent(glam::Vec2::ZERO));
    ecs.add_component(entity, VelocityComponent(Vec2::new(50.0, 0.0)));
    ecs.add_component(
        entity,
        SpriteComponent::new("tank".to_string(), Vec2::new(32.0, 32.0)).with_z_index(1),
    );

    let entity = ecs.create_entity();
    ecs.add_component(entity, TransformComponent(Vec2::new(100.0, 0.0)));
    ecs.add_component(entity, VelocityComponent(Vec2::new(-50.0, 0.0)));
    ecs.add_component(
        entity,
        SpriteComponent::new("truck".to_string(), Vec2::new(32.0, 32.0)).with_z_index(1),
    );

    let entity = ecs.create_entity();
    ecs.add_component(entity, TransformComponent(Vec2::new(0.0, 100.0)));
    ecs.add_component(entity, VelocityComponent(Vec2::new(0.0, 0.0)));
    ecs.add_component(entity, KeyboardControlComponent(100.0));
    ecs.add_component(
        entity,
        SpriteComponent::new("chopper".to_string(), Vec2::new(32.0, 32.0))
            .with_z_index(1)
            .with_src_rect(Rect::new(0.0, 0.0, 32.0, 32.0)),
    );
    ecs.add_component(
        entity,
        AnimationComponent::new()
            .num_frames(2)
            .framerate(15)
            .is_loop(true),
    );
    ecs.add_component(entity, CameraFollowComponent);

    let window_conf = window_conf();
    let camera = Camera(Rect::new(
        0.0,
        0.0,
        window_conf.window_width as f32,
        window_conf.window_height as f32,
    ));

    let map_dimensions = MapDimensions(Vec2::new(640.0, 640.0));
    ecs.resources.borrow_mut().put::<Camera>(camera);
    ecs.resources
        .borrow_mut()
        .put::<MapDimensions>(map_dimensions);
}

fn handle_keyboard_events(
    entity_manager: Rc<RefCell<EntityManager>>,
    event_bus: Rc<RefCell<EventBus>>,
) {
    if is_key_pressed(KeyCode::Up) {
        event_bus
            .borrow()
            .emit(entity_manager.clone(), KeyboardEvent(KeyCode::Up));
    }

    if is_key_pressed(KeyCode::Right) {
        event_bus
            .borrow()
            .emit(entity_manager.clone(), KeyboardEvent(KeyCode::Right));
    }

    if is_key_pressed(KeyCode::Down) {
        event_bus
            .borrow()
            .emit(entity_manager.clone(), KeyboardEvent(KeyCode::Down));
    }

    if is_key_pressed(KeyCode::Left) {
        event_bus
            .borrow()
            .emit(entity_manager.clone(), KeyboardEvent(KeyCode::Left));
    }
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

        handle_keyboard_events(ecs.entity_manager.clone(), ecs.event_bus.clone());

        clear_background(BLACK);
        ecs.update(time.elapsed());
        time = Instant::now();
        next_frame().await
    }
}
