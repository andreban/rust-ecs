use std::{any::TypeId, cell::RefCell, rc::Rc, time::Duration};

use crate::{
    entity::Signature,
    events::{EventBus, EventListener},
    AssetManager, Entity, EntityManager,
};

pub trait System: EventListener {
    fn signature(&self) -> &Signature;
    fn add_entity(&mut self, entity: Entity);
    fn remove_entity(&mut self, entity: Entity);
    fn get_event_type(&self) -> &[TypeId] {
        &[]
    }
    fn update(
        &self,
        delta_time: Duration,
        asset_manager: &AssetManager,
        entity_manager: Rc<RefCell<EntityManager>>,
        event_bus: Rc<RefCell<EventBus>>,
    );
}
