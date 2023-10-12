use std::cell::Ref;

use glam::Vec2;
use rust_ecs::entity::{Component, Query};
use rust_ecs::EntityComponentSystem;

#[derive(rust_ecs::derive::Component, Debug)]
pub struct TransformComponent(glam::Vec2);

#[derive(rust_ecs::derive::Component, Debug)]
pub struct RigidBodyComponent(glam::Vec2);

fn ble(query: Query<Ref<TransformComponent>>) {
    println!("ble");
    let objs = query.values();
    for o in objs {
        println!("{:?}", o);
    }
}

fn ble2(query: Query<Ref<RigidBodyComponent>>) {
    println!("ble2");
    let objs = query.values();
    for o in objs {
        println!("{:?}", o);
    }
}

fn main() {
    let mut ecs = EntityComponentSystem::new();

    ecs.add_system(|em|ble(em.into()));
    ecs.add_system(|em|ble2(em.into()));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::ZERO))
        .add_component(RigidBodyComponent(Vec2::ONE));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::new(100.0, 100.0)))
        .add_component(RigidBodyComponent(Vec2::ONE));
        
    ecs.update();
}
