use macroquad::prelude::{KeyCode, Vec2};
use rust_ecs::events::EventListener;
use rust_ecs::systems::System;
use rust_ecs::{Component, Entity, EntityManager, Signature};
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::{
    components::{KeyboardControlComponent, VelocityComponent},
    events::KeyboardEvent,
};

pub struct KeyboardMovementSystem {
    signature: Signature,
    entities: HashSet<Entity>,
    event_types: [TypeId; 1],
}

impl Default for KeyboardMovementSystem {
    fn default() -> Self {
        let event_types = [std::any::TypeId::of::<KeyboardEvent>()];
        let mut signature = Signature::with_capacity(32);
        signature.set(VelocityComponent::get_type_id(), true);
        signature.set(KeyboardControlComponent::get_type_id(), true);
        Self { signature, entities: Default::default(), event_types }
    }
}

impl System for KeyboardMovementSystem {
    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn get_event_type(&self) -> &[std::any::TypeId] {
        self.event_types.as_slice()
    }
}

impl EventListener for KeyboardMovementSystem {
    fn on_event(&self, em: Rc<RefCell<EntityManager>>, event: &rust_ecs::events::Event) {
        for entity in &self.entities {
            let em = em.borrow();
            let mut velocity = em.get_component_mut::<VelocityComponent>(*entity).unwrap();
            let keyboard_control = em
                .get_component::<KeyboardControlComponent>(*entity)
                .unwrap();
            match event.get_data::<KeyboardEvent>().unwrap().0 {
                KeyCode::Up => velocity.0 = Vec2::new(0.0, -keyboard_control.0),
                KeyCode::Right => velocity.0 = Vec2::new(keyboard_control.0, 0.0),
                KeyCode::Down => velocity.0 = Vec2::new(0.0, keyboard_control.0),
                KeyCode::Left => velocity.0 = Vec2::new(-keyboard_control.0, 0.0),
                _ => {}
            }
        }
    }
}
