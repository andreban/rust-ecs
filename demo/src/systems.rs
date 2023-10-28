use std::{
    cell::{Ref, RefMut},
    time::Duration,
};

use macroquad::{prelude::WHITE, texture::draw_texture};
use rust_ecs::{events::EventBus, AssetManager, Query};

use crate::{
    components::{SpriteComponent, TransformComponent, VelocityComponent},
    CollisionEvent,
};

// The movement system uses a mutable TransformComponent and an immutable VelocityComponent,
// updating the entity position.
pub fn movement_system(
    delta_time: Duration,
    query: Query<(RefMut<TransformComponent>, Ref<VelocityComponent>)>,
) {
    for (mut transform, velocity) in query.values() {
        transform.0 += velocity.0 * delta_time.as_secs_f32();
    }
}

// The render system prints the player position to the console.
pub fn render_system(
    asset_manager: &AssetManager,
    query: Query<(Ref<TransformComponent>, Ref<SpriteComponent>)>,
) {
    for (transform, sprite) in query.values() {
        let texture = asset_manager.get_texture(&sprite.sprite_name).unwrap();
        draw_texture(texture, transform.0.x, transform.0.y, WHITE);
    }
}

// Detects collision between entities.
pub fn collision_system(
    event_bus: &mut EventBus,
    _query: Query<(Ref<TransformComponent>, Ref<SpriteComponent>)>,
) {
    event_bus.emit(CollisionEvent { entity_a: 0, entity_b: 1 });
}

pub fn debug_system(event_bus: &mut EventBus) {
    event_bus.add_listener(|event: &CollisionEvent| {
        println!(
            "Collision between entities {} and {}",
            event.entity_a, event.entity_b
        );
    });
}
