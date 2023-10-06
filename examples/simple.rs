use std::collections::HashSet;

use glam::Vec2;
use rust_ecs::ecs::{component::Component, entity::Entity, system::System, Registry, Signature};

#[derive(rust_ecs::derive::Component, Debug)]
pub struct TransformComponent {
    position: glam::Vec2,
}

#[derive(macros::Component, Debug)]
pub struct RigidBodyComponent {
    velocity: glam::Vec2,
}

#[derive(Debug, rust_ecs::derive::BaseSystem)]
pub struct MovementSystem {
    entities: HashSet<Entity>,
    signature: Signature,
}

impl MovementSystem {
    pub fn new() -> Self {
        let mut signature = Signature::with_capacity(32);
        signature.set(TransformComponent::get_type_id(), true);
        signature.set(RigidBodyComponent::get_type_id(), true);
        MovementSystem {
            entities: HashSet::new(),
            signature,
        }
    }
}

impl System for MovementSystem {
    fn update(&self) {
        // let manager = self.get_entity_nanager();

        // let entities = Query::<(&mut PositionComponent, &VelocityComponent)>::queryz(manager);
        // for (position, velocity) in entities {
        //     position.x += velocity.x;
        //     position.y += velocity.y;
        // }
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }
}

fn main() {
    // $Env:RUST_BACKTRACE = 1
    let mut entity_manager = Registry::new();

    entity_manager.add_system(MovementSystem::new());

    let entity = entity_manager.create_entity().entity();
    entity_manager.add_component(
        entity,
        TransformComponent {
            position: Vec2::ZERO,
        },
    );
    entity_manager.add_component(
        entity,
        RigidBodyComponent {
            velocity: Vec2::ONE,
        },
    );

    let entity = entity_manager.create_entity().entity();
    entity_manager.add_component(
        entity,
        TransformComponent {
            position: Vec2::ZERO,
        },
    );
    entity_manager.add_component(
        entity,
        RigidBodyComponent {
            velocity: Vec2::ONE,
        },
    );

    let position_components = entity_manager.query_mut::<TransformComponent>();
    for position_component in position_components {
        println!("Position: {:?}", position_component);
    }

    let system = entity_manager.get_system::<MovementSystem>();
    println!("System: {:?}", system);
}
