use std::{any::TypeId, cell::RefCell, rc::Rc, time::Duration};

use crate::{
    component_signature::ComponentSignature,
    events::{EventBus, EventListener},
    AssetManager, Entity, EntityManager, Resources,
};

/// Core trait that all ECS systems must implement.
///
/// A system declares which components it cares about via [`signature`](Self::signature)
/// and receives per-frame callbacks via [`update`](Self::update). To handle
/// events, also implement [`EventListener`] and return the relevant
/// [`TypeId`]s from [`get_event_type`](Self::get_event_type).
pub trait System: EventListener {
    /// Returns the set of component types this system requires. Entities are
    /// added to the system only when their signature is a superset of this.
    fn signature(&self) -> &ComponentSignature;

    /// Called when an entity matching this system's signature is spawned.
    fn add_entity(&mut self, entity: Entity);

    /// Called when an entity is removed or no longer matches the signature.
    fn remove_entity(&mut self, entity: Entity);

    /// Returns the event types this system wants to receive via
    /// [`EventListener::on_event`]. Defaults to an empty slice (no events).
    fn get_event_type(&self) -> &[TypeId] {
        &[]
    }

    /// Called once per frame for every registered system.
    fn update(
        &self,
        _delta_time: Duration,
        _asset_manager: &AssetManager,
        _entity_manager: EntityManager,
        _event_bus: Rc<RefCell<EventBus>>,
        _resources: Rc<RefCell<Resources>>,
    ) {
    }
}
