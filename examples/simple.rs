use std::cell::{Ref, RefMut};

use glam::Vec2;
use rust_ecs::entity::{Component, EntityManager, Query};
use rust_ecs::EntityComponentSystem;

#[derive(rust_ecs::derive::Component, Debug)]
pub struct TransformComponent(glam::Vec2);

#[derive(rust_ecs::derive::Component, Debug)]
pub struct RigidBodyComponent(glam::Vec2);

fn movement_system(registry: &mut EntityManager) {
    let entities = Query::<(RefMut<TransformComponent>, Ref<RigidBodyComponent>)>::query(registry);
    for (mut transform, velocity) in entities {
        transform.0 += velocity.0;
    }
}

fn print_position_system(registry: &mut EntityManager) {
    let query = Query::<Ref<TransformComponent>>::query(registry);
    for transform in query {
        println!("Position: {:?}", transform.0);
    }
}

fn main() {
    let mut ecs = EntityComponentSystem::new();

    ecs.add_system(movement_system);
    ecs.add_system(print_position_system);

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::ZERO))
        .add_component(RigidBodyComponent(Vec2::ONE));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::new(100.0, 100.0)))
        .add_component(RigidBodyComponent(Vec2::ONE))
        .entity();

    ecs.update();
}
