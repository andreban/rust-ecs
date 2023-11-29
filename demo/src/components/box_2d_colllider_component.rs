use macroquad::math::Vec2;
use rust_ecs::derive::Component;

#[derive(Component, Debug)]
pub struct Box2dColliderComponent {
    pub offset: Vec2,
    pub size: Vec2,
}
