use rust_ecs::derive::Component;

#[derive(Component, Debug)]
pub struct HealthComponent {
    pub health: u32,
}
