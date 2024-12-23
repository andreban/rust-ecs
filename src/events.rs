use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::{systems::System, EntityManager};

type SystemRef = Rc<RefCell<Box<(dyn System + 'static)>>>;

pub struct Event {
    data: Box<dyn Any + 'static>,
}

impl Event {
    pub fn new<T: Clone + 'static>(data: T) -> Self {
        Self { data: Box::new(data) }
    }
    pub fn get_data<T: Clone + 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }
}

pub trait EventListener {
    fn on_event(&self, _em: EntityManager, _event: &Event) {}
}

#[derive(Default)]
pub struct EventBus {
    listeners: HashMap<TypeId, Vec<SystemRef>>,
}

impl EventBus {
    pub fn subscribe_type(&mut self, type_id: TypeId, listener: SystemRef) {
        let listeners = self.listeners.entry(type_id).or_default();
        listeners.push(listener);
    }

    pub fn emit<T: Clone + 'static>(&self, em: EntityManager, data: T) {
        let type_id = TypeId::of::<T>();
        if let Some(listeners) = self.listeners.get(&type_id) {
            let event = Event::new(data);
            for listener in listeners {
                listener.borrow().on_event(em.clone(), &event);
            }
        }
    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}
