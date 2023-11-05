use macroquad::prelude::*;
use rust_ecs::Component;

// A transform component, with the entity position.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct TransformComponent(pub Vec2);
