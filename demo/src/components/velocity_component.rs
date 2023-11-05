use macroquad::prelude::*;
use rust_ecs::Component;

// A velocity component, with the entity position.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct VelocityComponent(pub Vec2);
