use rust_ecs::systems::{System, SystemBuilder};

use crate::components::{TransformComponent, VelocityComponent};

// The movement system uses a mutable TransformComponent and an immutable VelocityComponent,
// updating the entity position.
pub fn create_movement_system() -> System {
    SystemBuilder::new()
        .require_component::<TransformComponent>()
        .require_component::<VelocityComponent>()
        .with_update(|entities, delta_time, _, em, _| {
            for entity in entities {
                let mut transform = em.get_component_mut::<TransformComponent>(*entity).unwrap();
                let velocity = em.get_component::<VelocityComponent>(*entity).unwrap();
                transform.0 += velocity.0 * delta_time.as_secs_f32();
            }
        })
        .build()
}
