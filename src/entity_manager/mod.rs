mod component;
mod em;
mod entity;
mod group_manager;
mod tag_manager;
// mod query;

pub use component::{get_next_component_type_id, Component, ComponentTypeId};
pub use entity::{Entity, EntityId};
pub use tag_manager::TagManager;
// pub use query::Query;
pub use em::EntityManager;
pub use group_manager::GroupManager;
