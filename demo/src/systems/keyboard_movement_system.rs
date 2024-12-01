use macroquad::prelude::{KeyCode, Vec2};
use rust_ecs::events::EventListener;
use rust_ecs::systems::System;
use rust_ecs::{ComponentSignature, Entity, EntityManager};
use std::any::TypeId;
use std::collections::HashSet;

use crate::components::SpriteComponent;
use crate::{
    components::{KeyboardControlComponent, VelocityComponent},
    events::KeyboardEvent,
};

pub struct KeyboardMovementSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
    event_types: [TypeId; 1],
}

impl Default for KeyboardMovementSystem {
    fn default() -> Self {
        let event_types = [std::any::TypeId::of::<KeyboardEvent>()];
        let mut signature = ComponentSignature::default();
        signature.require_component::<VelocityComponent>();
        signature.require_component::<KeyboardControlComponent>();
        signature.require_component::<SpriteComponent>();
        Self { signature, entities: Default::default(), event_types }
    }
}

impl System for KeyboardMovementSystem {
    fn signature(&self) -> &ComponentSignature {
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
    fn on_event(&self, em: EntityManager, event: &rust_ecs::events::Event) {
        for entity in &self.entities {
            let velocity = em.get_component::<VelocityComponent>(entity).unwrap();
            let sprite = em.get_component::<SpriteComponent>(entity).unwrap();
            let keyboard_control = em
                .get_component::<KeyboardControlComponent>(entity)
                .unwrap();

            let mut velocity = velocity.borrow_mut();
            let mut sprite = sprite.borrow_mut();
            let keyboard_control = keyboard_control.borrow();
            
            match event.get_data::<KeyboardEvent>().unwrap().0 {
                KeyCode::Up => {
                    velocity.0 = Vec2::new(0.0, -keyboard_control.0);
                    if let Some(rect) = sprite.src_rect.as_mut() {
                        rect.y = 0.0;
                    }
                }
                KeyCode::Right => {
                    velocity.0 = Vec2::new(keyboard_control.0, 0.0);
                    if let Some(rect) = sprite.src_rect.as_mut() {
                        rect.y = 32.0;
                    }
                }
                KeyCode::Down => {
                    velocity.0 = Vec2::new(0.0, keyboard_control.0);
                    if let Some(rect) = sprite.src_rect.as_mut() {
                        rect.y = 64.0;
                    }
                }
                KeyCode::Left => {
                    velocity.0 = Vec2::new(-keyboard_control.0, 0.0);
                    if let Some(rect) = sprite.src_rect.as_mut() {
                        rect.y = 96.0;
                    }
                }
                _ => {}
            }
        }
    }
}
