use rust_ecs::Component;

// A keyboard control component, with the entity speed.
#[derive(rust_ecs::derive::Component, Debug)]
pub struct KeyboardControlComponent(pub f32);
