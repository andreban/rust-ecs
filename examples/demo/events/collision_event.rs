use rust_ecs::Entity;

#[derive(Clone)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}
