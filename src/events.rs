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
