use macroquad::prelude::*;
use rust_ecs::Component;

#[derive(rust_ecs::derive::Component, Debug)]
pub struct SpriteComponent {
    pub sprite_name: String,
    pub src_rect: Option<Rect>,
    pub dst_size: Vec2,
    pub z_index: i32,
}

impl SpriteComponent {
    pub fn new(sprite_name: String, dst_size: Vec2) -> Self {
        Self { sprite_name, src_rect: None, dst_size, z_index: 0 }
    }

    pub fn with_src_rect(mut self, src_rect: Rect) -> Self {
        self.src_rect = Some(src_rect);
        self
    }

    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }
}
