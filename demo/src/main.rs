mod components;
mod events;
mod resources;
mod systems;
mod tilemap;

use std::time::SystemTime;
use std::{
    cell::RefCell,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use components::{
    AnimationComponent, CameraFollowComponent, HealthComponent, KeyboardControlComponent,
    SpriteComponent, TransformComponent, VelocityComponent,
};
use events::KeyboardEvent;
use macroquad::prelude::*;

use crate::components::ProjectileEmitterComponent;
use resources::{Camera, MapDimensions};
use rust_ecs::{events::EventBus, EntityComponentSystem, EntityManager};
use tilemap::load_map;

fn window_conf() -> Conf {
    Conf { window_title: "Demo".to_string(), ..Default::default() }
}

pub async fn setup(ecs: &mut EntityComponentSystem) {
    // Load assets.
    ecs.asset_manager_mut()
        .load_texture("tank", "assets/images/tank-panther-right.png")
        .await
        .unwrap();

    ecs.asset_manager_mut()
        .load_texture("truck", "assets/images/truck-ford-right.png")
        .await
        .unwrap();

    ecs.asset_manager_mut()
        .load_texture("jungle", "assets/tilemaps/jungle.png")
        .await
        .unwrap();

    ecs.asset_manager_mut()
        .load_texture("chopper", "assets/images/chopper-spritesheet.png")
        .await
        .unwrap();

    ecs.asset_manager_mut()
        .load_texture("bullet", "assets/images/bullet.png")
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
    ecs.add_system(systems::ProjectileEmitterSystem::default());
    ecs.add_system(systems::ProjectileLifecycleSystem::default());

    let tiles = load_map("assets/tilemaps/jungle.map").unwrap();
    let tile_scale = 2;
    for tile in tiles {
        let tile_x = (tile.x * 32 * tile_scale) as f32;
        let tile_y = (tile.y * 32 * tile_scale) as f32;

        let tile_src_y = (tile.sprite_id / 10 * 32) as f32;
        let tile_src_x = (tile.sprite_id % 10 * 32) as f32;

        let entity = ecs.create_entity();
        ecs.entity_manager_mut()
            .group_manager_mut()
            .add_entity_to_group(&entity, "tile");
        ecs.add_component(entity, TransformComponent(Vec2::new(tile_x, tile_y)));
        ecs.add_component(
            entity,
            SpriteComponent::new(
                "jungle",
                Vec2::new(32.0 * tile_scale as f32, 32.0 * tile_scale as f32),
            )
            .with_src_rect(Rect::new(tile_src_x + 0.5, tile_src_y + 0.5, 31.0, 31.0)),
        )
    }

    // Create entities with components.
    let tank = ecs.create_entity();
    ecs.entity_manager_mut()
        .group_manager_mut()
        .add_entity_to_group(&tank, "enemy");
    ecs.add_component(tank, TransformComponent(glam::Vec2::ZERO));
    ecs.add_component(tank, VelocityComponent(Vec2::new(0.0, 0.0)));
    ecs.add_component(
        tank,
        SpriteComponent::new("tank", Vec2::new(32.0, 32.0)).with_z_index(1),
    );
    ecs.add_component(
        tank,
        ProjectileEmitterComponent {
            repeat_interval: Some(Duration::from_secs(1)),
            projectile_velocity: Vec2::new(150.0, 0.0),
            last_emitted: SystemTime::now(),
            projectile_duration: Duration::from_secs(5),
            damage: 10,
            friendly: false,
        },
    );
    ecs.add_component(tank, HealthComponent { health: 100 });

    let truck = ecs.create_entity();
    ecs.entity_manager_mut()
        .group_manager_mut()
        .add_entity_to_group(&truck, "enemy");
    ecs.add_component(truck, TransformComponent(Vec2::new(100.0, 0.0)));
    ecs.add_component(truck, VelocityComponent(Vec2::new(-0.0, 0.0)));
    ecs.add_component(
        truck,
        SpriteComponent::new("truck", Vec2::new(32.0, 32.0)).with_z_index(1),
    );
    ecs.add_component(
        truck,
        ProjectileEmitterComponent {
            repeat_interval: Some(Duration::from_secs(3)),
            projectile_velocity: Vec2::new(0.0, 150.0),
            last_emitted: SystemTime::now(),
            projectile_duration: Duration::from_secs(5),
            damage: 10,
            friendly: false,
        },
    );
    ecs.add_component(truck, HealthComponent { health: 100 });

    let chopper = ecs.create_entity();
    ecs.entity_manager_mut()
        .tag_manager_mut()
        .set_tag(chopper, "player");
    ecs.add_component(chopper, TransformComponent(Vec2::new(0.0, 100.0)));
    ecs.add_component(chopper, VelocityComponent(Vec2::new(0.0, 0.0)));
    ecs.add_component(chopper, KeyboardControlComponent(100.0));
    ecs.add_component(
        chopper,
        SpriteComponent::new("chopper", Vec2::new(32.0, 32.0))
            .with_z_index(1)
            .with_src_rect(Rect::new(0.0, 0.0, 32.0, 32.0)),
    );
    ecs.add_component(
        chopper,
        AnimationComponent::new()
            .num_frames(2)
            .framerate(15)
            .is_loop(true),
    );
    ecs.add_component(chopper, CameraFollowComponent);
    ecs.add_component(
        chopper,
        ProjectileEmitterComponent {
            repeat_interval: None,
            projectile_velocity: Vec2::new(150.0, 150.0),
            last_emitted: SystemTime::now(),
            projectile_duration: Duration::from_secs(5),
            damage: 10,
            friendly: true,
        },
    );
    ecs.add_component(chopper, HealthComponent { health: 100 });

    let window_conf = window_conf();
    let camera = Camera(Rect::new(
        0.0,
        0.0,
        window_conf.window_width as f32,
        window_conf.window_height as f32,
    ));

    let map_dimensions = MapDimensions(Vec2::new(640.0, 640.0));
    ecs.resources_mut().put::<Camera>(camera);
    ecs.resources_mut().put::<MapDimensions>(map_dimensions);
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

    if is_key_pressed(KeyCode::Space) {
        event_bus
            .borrow()
            .emit(entity_manager.clone(), KeyboardEvent(KeyCode::Space));
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

        handle_keyboard_events(ecs.entity_manager_cloned(), ecs.event_bus_cloned());

        clear_background(BLACK);
        ecs.update(time.elapsed());
        time = Instant::now();
        next_frame().await
    }
}
