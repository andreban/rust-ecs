use std::any::{Any, TypeId};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::Duration;

use rust_ecs::events::{Event, EventBus, EventListener};
use rust_ecs::systems::System;
use rust_ecs::{
    get_next_component_type_id, AssetManager, Component, ComponentSignature, Entity,
    EntityComponentSystem, EntityManager, Resources,
};

// ---------------------------------------------------------------------------
// Minimal test components
// ---------------------------------------------------------------------------

struct CompA;

impl Component for CompA {
    fn get_type_id() -> usize {
        static TYPE_ID: OnceLock<usize> = OnceLock::new();
        *TYPE_ID.get_or_init(get_next_component_type_id)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct CompB(u32);

impl Component for CompB {
    fn get_type_id() -> usize {
        static TYPE_ID: OnceLock<usize> = OnceLock::new();
        *TYPE_ID.get_or_init(get_next_component_type_id)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// ---------------------------------------------------------------------------
// Event type for testing
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct TestEvent;

// ---------------------------------------------------------------------------
// Tracking system
// ---------------------------------------------------------------------------

struct TrackingSystem {
    required: ComponentSignature,
    subscriptions: Vec<TypeId>,
    entities: Rc<RefCell<Vec<Entity>>>,
    update_count: Rc<RefCell<u32>>,
    event_count: Rc<Cell<u32>>,
}

impl TrackingSystem {
    fn new(required: ComponentSignature) -> Self {
        TrackingSystem {
            required,
            subscriptions: vec![],
            entities: Rc::new(RefCell::new(Vec::new())),
            update_count: Rc::new(RefCell::new(0)),
            event_count: Rc::new(Cell::new(0)),
        }
    }

    fn subscribing_to_test_event(mut self) -> Self {
        self.subscriptions.push(TypeId::of::<TestEvent>());
        self
    }
}

impl EventListener for TrackingSystem {
    fn on_event(&self, _em: EntityManager, _event: &Event) {
        self.event_count.set(self.event_count.get() + 1);
    }
}

impl System for TrackingSystem {
    fn signature(&self) -> &ComponentSignature {
        &self.required
    }

    fn get_event_type(&self) -> &[TypeId] {
        &self.subscriptions
    }

    fn add_entity(&mut self, entity: Entity) {
        self.entities.borrow_mut().push(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.borrow_mut().retain(|e| *e != entity);
    }

    fn update(
        &self,
        _delta_time: Duration,
        _asset_manager: &AssetManager,
        _entity_manager: EntityManager,
        _event_bus: Rc<RefCell<EventBus>>,
        _resources: Rc<RefCell<Resources>>,
    ) {
        *self.update_count.borrow_mut() += 1;
    }
}

// ---------------------------------------------------------------------------
// Entity and component lifecycle
// ---------------------------------------------------------------------------

#[test]
fn create_entity_returns_unique_entities() {
    let mut ecs = EntityComponentSystem::new();
    let e1 = ecs.create_entity();
    let e2 = ecs.create_entity();
    assert_ne!(e1, e2);
}

#[test]
fn add_and_retrieve_component_via_entity_manager() {
    let mut ecs = EntityComponentSystem::new();
    let entity = ecs.create_entity();
    ecs.add_component(entity, CompB(42));
    let comp = ecs
        .entity_manager()
        .get_component::<CompB>(&entity)
        .expect("component should be present");
    assert_eq!(comp.borrow().0, 42);
}

#[test]
fn remove_component_then_get_returns_none() {
    let mut ecs = EntityComponentSystem::new();
    let entity = ecs.create_entity();
    ecs.add_component(entity, CompA);
    ecs.remove_component::<CompA>(entity);
    assert!(ecs.entity_manager().get_component::<CompA>(&entity).is_none());
}

// ---------------------------------------------------------------------------
// System + entity interaction
// ---------------------------------------------------------------------------

#[test]
fn ecs_receives_entity_after_component_added() {
    let mut ecs = EntityComponentSystem::new();
    let mut sig = ComponentSignature::default();
    sig.require_component::<CompA>();
    let system = TrackingSystem::new(sig);
    let entities = system.entities.clone();
    ecs.add_system(system);

    let entity = ecs.create_entity();
    ecs.add_component(entity, CompA);

    assert!(entities.borrow().contains(&entity));
}

#[test]
fn ecs_update_invokes_system_update() {
    let mut ecs = EntityComponentSystem::new();
    let system = TrackingSystem::new(ComponentSignature::default());
    let update_count = system.update_count.clone();
    ecs.add_system(system);

    assert_eq!(*update_count.borrow(), 0);
    ecs.update(Duration::from_millis(16));
    assert_eq!(*update_count.borrow(), 1);
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

#[test]
fn resources_accessible_and_mutable() {
    let ecs = EntityComponentSystem::new();
    ecs.resources_mut().put(42u32);
    assert_eq!(ecs.resources().get::<u32>(), Some(&42));
}

// ---------------------------------------------------------------------------
// Event bus
// ---------------------------------------------------------------------------

#[test]
fn emit_through_ecs_event_bus_reaches_system() {
    let mut ecs = EntityComponentSystem::new();
    let system = TrackingSystem::new(ComponentSignature::default()).subscribing_to_test_event();
    let event_count = system.event_count.clone();
    ecs.add_system(system);

    ecs.event_bus_cloned()
        .borrow()
        .emit(ecs.entity_manager(), TestEvent);

    assert_eq!(event_count.get(), 1);
}

// ---------------------------------------------------------------------------
// Default / constructors
// ---------------------------------------------------------------------------

#[test]
fn default_ecs_has_no_entities() {
    let ecs = EntityComponentSystem::default();
    let live = ecs
        .entity_manager()
        .get_entities_with_signature(&ComponentSignature::default());
    assert!(live.is_empty());
}

#[test]
fn default_ecs_has_empty_resources() {
    let ecs = EntityComponentSystem::default();
    assert!(ecs.resources().get::<u32>().is_none());
}
