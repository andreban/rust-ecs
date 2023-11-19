use rust_ecs::derive::Component;
use std::time::{Duration, SystemTime};

#[derive(Component)]
pub struct ProjectileComponent {
    pub max_duration: Duration,
    pub created: SystemTime,
}
