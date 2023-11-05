use rust_ecs::Entity;

pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}
