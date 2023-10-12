use std::cell::{Ref, RefMut};

use glam::Vec2;
use rust_ecs::entity::{Component, Query};
use rust_ecs::EntityComponentSystem;

#[derive(rust_ecs::derive::Component, Debug)]
pub struct TransformComponent(glam::Vec2);

#[derive(rust_ecs::derive::Component, Debug)]
pub struct SpeedComponent(glam::Vec2);

fn movement_system(query: Query<(RefMut<TransformComponent>, Ref<SpeedComponent>)>) {
    for (mut transform, velocity) in query.values() {
        transform.0 += velocity.0;
    }
}

fn render_system(query: Query<Ref<TransformComponent>>) {
    for transform in query.values() {
        println!("Position: {:?}", transform.0);
    }
}

fn main() {
    let mut ecs = EntityComponentSystem::new();

    ecs.add_system(|em|movement_system(em.into()));
    ecs.add_system(|em|render_system(em.into()));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::ZERO))
        .add_component(SpeedComponent(Vec2::ONE));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::new(100.0, 100.0)))
        .add_component(SpeedComponent(Vec2::ONE))
        .entity();

    ecs.update();
}
