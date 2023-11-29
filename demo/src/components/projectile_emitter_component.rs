use macroquad::math::Vec2;
use rust_ecs::derive::Component;
use std::time::{Duration, SystemTime};

#[derive(Component, Debug)]
pub struct ProjectileEmitterComponent {
    pub projectile_velocity: Vec2,
    pub repeat_interval: Option<Duration>,
    pub last_emitted: SystemTime,
    pub projectile_duration: Duration,
    pub damage: u32,
    pub friendly: bool,
}
