use macroquad::prelude::*;

// A velocity component, with the entity position.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct VelocityComponent(pub Vec2);
