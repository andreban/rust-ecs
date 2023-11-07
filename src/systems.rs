use std::{any::TypeId, cell::RefCell, rc::Rc, time::Duration};

use crate::{
    entity_manager::Signature,
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
    /// The update function is called for every frame.
    fn update(
        &self,
        _delta_time: Duration,
        _asset_manager: &AssetManager,
        _entity_manager: Rc<RefCell<EntityManager>>,
        _event_bus: Rc<RefCell<EventBus>>,
    ) {
    }
}
