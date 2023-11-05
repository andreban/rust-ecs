use macroquad::prelude::{KeyCode, Vec2};
use rust_ecs::systems::{System, SystemBuilder};
use std::cell::Ref;
use std::cell::RefMut;

use crate::{
    components::{KeyboardControlComponent, VelocityComponent},
    events::KeyboardEvent,
};

pub fn create_keyboard_movement_system() -> System {
    SystemBuilder::new()
        .with_setup_listeners(|event_bus| {
            event_bus.add_listener(|em, event: &KeyboardEvent| {
                let query =
                    em.create_query::<(RefMut<VelocityComponent>, Ref<KeyboardControlComponent>)>();
                for (mut velocity, keyboard_control) in query.values() {
                    match event.0 {
                        KeyCode::Up => velocity.0 = Vec2::new(0.0, -keyboard_control.0),
                        KeyCode::Right => velocity.0 = Vec2::new(keyboard_control.0, 0.0),
                        KeyCode::Down => velocity.0 = Vec2::new(0.0, keyboard_control.0),
                        KeyCode::Left => velocity.0 = Vec2::new(-keyboard_control.0, 0.0),
                        _ => {}
                    }
                }
            });
        })
        .build()
}
