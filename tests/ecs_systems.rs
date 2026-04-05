use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::Duration;

use rust_ecs::systems::System;
use rust_ecs::events::EventListener;
use rust_ecs::{
    Component, ComponentSignature, Entity, EntityComponentSystem, get_next_component_type_id,
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
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

struct CompB;

impl Component for CompB {
    fn get_type_id() -> usize {
        static TYPE_ID: OnceLock<usize> = OnceLock::new();
        *TYPE_ID.get_or_init(get_next_component_type_id)
    }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

// ---------------------------------------------------------------------------
// Helper: build a signature for the given component types
// ---------------------------------------------------------------------------

fn sig_a() -> ComponentSignature {
    let mut s = ComponentSignature::default();
    s.require_component::<CompA>();
    s
}

fn sig_ab() -> ComponentSignature {
    let mut s = ComponentSignature::default();
    s.require_component::<CompA>();
    s.require_component::<CompB>();
    s
}

fn sig_b() -> ComponentSignature {
    let mut s = ComponentSignature::default();
    s.require_component::<CompB>();
    s
}

// ---------------------------------------------------------------------------
// Tracking system
// ---------------------------------------------------------------------------

/// A system that records which entities have been added / removed, and counts
/// how many times `update` has been called.
struct TrackingSystem {
    required: ComponentSignature,
    entities: Rc<RefCell<Vec<Entity>>>,
    update_count: Rc<RefCell<u32>>,
}

impl TrackingSystem {
    fn new(required: ComponentSignature) -> Self {
        TrackingSystem {
            required,
            entities: Rc::new(RefCell::new(Vec::new())),
            update_count: Rc::new(RefCell::new(0)),
        }
    }
}

impl EventListener for TrackingSystem {}

impl System for TrackingSystem {
    fn signature(&self) -> &ComponentSignature {
        &self.required
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
        _asset_manager: &rust_ecs::AssetManager,
        _entity_manager: rust_ecs::EntityManager,
        _event_bus: Rc<RefCell<rust_ecs::events::EventBus>>,
        _resources: Rc<RefCell<rust_ecs::Resources>>,
    ) {
        *self.update_count.borrow_mut() += 1;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn system_receives_entity_when_signature_matches() {
    let mut ecs = EntityComponentSystem::new();
    let system = TrackingSystem::new(sig_a());
    let entities = system.entities.clone();
    ecs.add_system(system);

    let entity = ecs.create_entity();
    ecs.add_component(entity, CompA);

    assert!(entities.borrow().contains(&entity));
}

#[test]
fn system_does_not_receive_entity_when_signature_does_not_match() {
    let mut ecs = EntityComponentSystem::new();
    let system = TrackingSystem::new(sig_ab());
    let entities = system.entities.clone();
    ecs.add_system(system);

    let entity = ecs.create_entity();
    ecs.add_component(entity, CompA); // only A, but system requires {A, B}

    assert!(!entities.borrow().contains(&entity));
}

#[test]
fn system_receives_entity_after_all_required_components_added() {
    let mut ecs = EntityComponentSystem::new();
    let system = TrackingSystem::new(sig_ab());
    let entities = system.entities.clone();
    ecs.add_system(system);

    let entity = ecs.create_entity();
    ecs.add_component(entity, CompA);
    assert!(!entities.borrow().contains(&entity), "should not match yet");

    ecs.add_component(entity, CompB);
    assert!(entities.borrow().contains(&entity), "should match after B added");
}

#[test]
fn system_loses_entity_when_required_component_removed() {
    let mut ecs = EntityComponentSystem::new();
    let system = TrackingSystem::new(sig_ab());
    let entities = system.entities.clone();
    ecs.add_system(system);

    let entity = ecs.create_entity();
    ecs.add_component(entity, CompA);
    ecs.add_component(entity, CompB);
    assert!(entities.borrow().contains(&entity));

    ecs.remove_component::<CompB>(entity);
    assert!(!entities.borrow().contains(&entity));
}

#[test]
fn multiple_systems_each_get_correct_entities() {
    let mut ecs = EntityComponentSystem::new();

    let sys1 = TrackingSystem::new(sig_a());
    let entities1 = sys1.entities.clone();
    ecs.add_system(sys1);

    let sys2 = TrackingSystem::new(sig_b());
    let entities2 = sys2.entities.clone();
    ecs.add_system(sys2);

    // entity_ab has both A and B — should appear in both systems
    let entity_ab = ecs.create_entity();
    ecs.add_component(entity_ab, CompA);
    ecs.add_component(entity_ab, CompB);

    // entity_a has only A — should appear only in sys1
    let entity_a = ecs.create_entity();
    ecs.add_component(entity_a, CompA);

    assert!(entities1.borrow().contains(&entity_ab));
    assert!(entities1.borrow().contains(&entity_a));
    assert!(entities2.borrow().contains(&entity_ab));
    assert!(!entities2.borrow().contains(&entity_a));
}

#[test]
fn system_update_called_with_delta_time() {
    let mut ecs = EntityComponentSystem::new();
    let system = TrackingSystem::new(ComponentSignature::default());
    let update_count = system.update_count.clone();
    ecs.add_system(system);

    assert_eq!(*update_count.borrow(), 0);
    ecs.update(Duration::from_millis(16));
    assert_eq!(*update_count.borrow(), 1);
    ecs.update(Duration::from_millis(16));
    assert_eq!(*update_count.borrow(), 2);
}
