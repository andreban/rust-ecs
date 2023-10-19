use macroquad::prelude::Vec2;
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
    pub src_x: usize,
    pub src_y: usize,
    pub width: usize,
    pub height: usize,
}

impl SpriteComponent {
    pub fn new(sprite_name: String, width: usize, height: usize) -> Self {
        Self { sprite_name, src_x: 0, src_y: 0, width, height }
    }
}
