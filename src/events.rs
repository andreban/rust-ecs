use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::{systems::System, EntityManager};

type SystemRef = Rc<RefCell<Box<dyn System + 'static>>>;

/// A type-erased event payload.
///
/// Wrap any `Clone + 'static` value with [`Event::new`] and retrieve it with
/// [`Event::get_data`].
pub struct Event {
    data: Box<dyn Any + 'static>,
}

impl Event {
    /// Wraps `data` in a new event.
    pub fn new<T: Clone + 'static>(data: T) -> Self {
        Self { data: Box::new(data) }
    }

    /// Attempts to downcast the event payload to `T`, returning `None` on type
    /// mismatch.
    pub fn get_data<T: Clone + 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }
}

/// Trait implemented by systems that wish to receive events.
///
/// Override [`on_event`](Self::on_event) to handle events. The default
/// implementation is a no-op.
pub trait EventListener {
    /// Called by the [`EventBus`] when an event matching one of the system's
    /// registered types is emitted.
    fn on_event(&self, _em: EntityManager, _event: &Event) {}
}

/// Pub/sub event dispatcher.
///
/// Systems register their interest in specific event types via
/// [`subscribe_type`](Self::subscribe_type). Call [`emit`](Self::emit) to
/// dispatch an event to all matching listeners.
#[derive(Default)]
pub struct EventBus {
    listeners: HashMap<TypeId, Vec<SystemRef>>,
}

impl EventBus {
    /// Subscribes `listener` to events identified by `type_id`.
    pub fn subscribe_type(&mut self, type_id: TypeId, listener: SystemRef) {
        let listeners = self.listeners.entry(type_id).or_default();
        listeners.push(listener);
    }

    /// Emits an event carrying `data` to all listeners registered for type `T`.
    pub fn emit<T: Clone + 'static>(&self, em: EntityManager, data: T) {
        let type_id = TypeId::of::<T>();
        if let Some(listeners) = self.listeners.get(&type_id) {
            let event = Event::new(data);
            for listener in listeners {
                listener.borrow().on_event(em.clone(), &event);
            }
        }
    }

    /// Removes all listener registrations.
    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{any::TypeId, cell::Cell, rc::Rc};

    use crate::{
        component_signature::ComponentSignature,
        systems::System,
        Entity, EntityManager,
    };

    // --- helpers ---

    #[derive(Clone)]
    struct TestData {
        value: u32,
    }

    struct CountingSystem {
        signature: ComponentSignature,
        count: Rc<Cell<u32>>,
    }

    impl CountingSystem {
        fn new(count: Rc<Cell<u32>>) -> Self {
            Self { signature: ComponentSignature::default(), count }
        }
    }

    impl EventListener for CountingSystem {
        fn on_event(&self, _em: EntityManager, _event: &Event) {
            self.count.set(self.count.get() + 1);
        }
    }

    impl System for CountingSystem {
        fn signature(&self) -> &ComponentSignature {
            &self.signature
        }
        fn add_entity(&mut self, _entity: Entity) {}
        fn remove_entity(&mut self, _entity: Entity) {}
    }

    struct OtherSystem {
        signature: ComponentSignature,
        count: Rc<Cell<u32>>,
    }

    impl OtherSystem {
        fn new(count: Rc<Cell<u32>>) -> Self {
            Self { signature: ComponentSignature::default(), count }
        }
    }

    impl EventListener for OtherSystem {
        fn on_event(&self, _em: EntityManager, _event: &Event) {
            self.count.set(self.count.get() + 1);
        }
    }

    impl System for OtherSystem {
        fn signature(&self) -> &ComponentSignature {
            &self.signature
        }
        fn add_entity(&mut self, _entity: Entity) {}
        fn remove_entity(&mut self, _entity: Entity) {}
    }

    fn make_em() -> EntityManager {
        EntityManager::new()
    }

    // --- Event tests ---

    #[test]
    fn event_get_data_returns_stored_value() {
        let event = Event::new(42u32);
        assert_eq!(event.get_data::<u32>(), Some(&42u32));
    }

    #[test]
    fn event_get_data_wrong_type_returns_none() {
        let event = Event::new(42u32);
        assert_eq!(event.get_data::<String>(), None);
    }

    #[test]
    fn event_works_with_clone_types() {
        let data = TestData { value: 7 };
        let event = Event::new(data);
        let retrieved = event.get_data::<TestData>();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, 7);
    }

    // --- EventBus tests ---

    #[test]
    fn emit_calls_subscribed_listener() {
        let count = Rc::new(Cell::new(0u32));
        let system = CountingSystem::new(Rc::clone(&count));
        let system_ref: SystemRef = Rc::new(RefCell::new(Box::new(system)));

        let mut bus = EventBus::default();
        bus.subscribe_type(TypeId::of::<u32>(), system_ref);
        bus.emit(make_em(), 1u32);

        assert_eq!(count.get(), 1);
    }

    #[test]
    fn emit_does_not_call_unsubscribed_listener() {
        let count = Rc::new(Cell::new(0u32));
        let system = CountingSystem::new(Rc::clone(&count));
        let system_ref: SystemRef = Rc::new(RefCell::new(Box::new(system)));

        let mut bus = EventBus::default();
        // subscribed to u32, but we emit String
        bus.subscribe_type(TypeId::of::<u32>(), system_ref);
        bus.emit(make_em(), String::from("hello"));

        assert_eq!(count.get(), 0);
    }

    #[test]
    fn multiple_listeners_all_called() {
        let count_a = Rc::new(Cell::new(0u32));
        let count_b = Rc::new(Cell::new(0u32));

        let sys_a = CountingSystem::new(Rc::clone(&count_a));
        let sys_b = OtherSystem::new(Rc::clone(&count_b));

        let ref_a: SystemRef = Rc::new(RefCell::new(Box::new(sys_a)));
        let ref_b: SystemRef = Rc::new(RefCell::new(Box::new(sys_b)));

        let mut bus = EventBus::default();
        bus.subscribe_type(TypeId::of::<u32>(), ref_a);
        bus.subscribe_type(TypeId::of::<u32>(), ref_b);
        bus.emit(make_em(), 99u32);

        assert_eq!(count_a.get(), 1);
        assert_eq!(count_b.get(), 1);
    }

    #[test]
    fn clear_removes_all_subscriptions() {
        let count = Rc::new(Cell::new(0u32));
        let system = CountingSystem::new(Rc::clone(&count));
        let system_ref: SystemRef = Rc::new(RefCell::new(Box::new(system)));

        let mut bus = EventBus::default();
        bus.subscribe_type(TypeId::of::<u32>(), system_ref);
        bus.clear();
        bus.emit(make_em(), 1u32);

        assert_eq!(count.get(), 0);
    }

    #[test]
    fn emit_with_no_listeners_does_not_panic() {
        let bus = EventBus::default();
        bus.emit(make_em(), 42u32); // should not panic
    }
}
