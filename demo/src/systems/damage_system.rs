use rust_ecs::systems::{System, SystemBuilder};

use crate::events::CollisionEvent;

pub fn create_damage_system() -> System {
    SystemBuilder::new()
        .with_setup_listeners(|event_bus| {
            event_bus.add_listener(|em, event: &CollisionEvent| {
                println!(
                    "Collision between entities {:?} and {:?}",
                    event.entity_a, event.entity_b
                );
                em.destroy_entity(event.entity_a);
                em.destroy_entity(event.entity_b);
            });
        })
        .build()
}
