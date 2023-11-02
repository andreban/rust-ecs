use macroquad::prelude::{Rect, Vec2};
use rust_ecs::Component;

// A transform component, with the entity position.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct TransformComponent(pub Vec2);

// A velocity component, with the entity position.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct VelocityComponent(pub Vec2);

#[derive(rust_ecs::derive::Component, Debug)]
pub struct SpriteComponent {
    pub sprite_name: String,
    pub src_rect: Option<Rect>,
    pub dst_size: Vec2,
    pub z_index: i32,
}

impl SpriteComponent {
    pub fn new(sprite_name: String, src_rect: Option<Rect>, dst_size: Vec2, z_index: i32) -> Self {
        Self { sprite_name, src_rect, dst_size, z_index }
    }
}
