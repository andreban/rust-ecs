pub mod ecs;

pub mod derive {
    pub use macros::{BaseSystem, Component};
}

// use ecs::{
//     component::{PositionComponent, VelocityComponent},
//     Registry,
// };

// fn main() {
//     let mut entity_manager = Registry::new();

//     entity_manager
//         .create_entity()
//         .add_component(PositionComponent { x: 0.0, y: 0.0 })
//         .add_component(VelocityComponent { x: 1.0, y: 1.0 })
//         .entity();

//     entity_manager
//         .create_entity()
//         .add_component(PositionComponent { x: 0.0, y: 0.0 })
//         .add_component(VelocityComponent { x: 1.0, y: 1.0 })
//         .entity();

//     let position_components = entity_manager.query_mut::<PositionComponent>();
//     for position_component in position_components {
//         position_component.x += 1.0;
//         println!("Position: {:?}", position_component);
//     }
// }
