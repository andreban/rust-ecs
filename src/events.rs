use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::{systems::System, EntityManager};

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
    fn on_event(&self, em: Rc<RefCell<EntityManager>>, event: &Event);
}

#[derive(Default)]
pub struct EventBus<'a> {
    listeners: HashMap<TypeId, Vec<&'a Box<(dyn System + 'static)>>>,
}

impl<'a> EventBus<'a> {
    pub fn subscribe_type(&mut self, type_id: TypeId, listener: &'a Box<dyn System + 'static>) {
        let listeners = self.listeners.entry(type_id).or_default();
        // let boxed: Box<&(dyn EventListener + 'static)> = Box::new(listener);
        listeners.push(listener);
    }

    // pub fn subscribe<T: Clone + 'static>(&mut self, listener: &'a (impl EventListener + 'static)) {
    //     let type_id = TypeId::of::<T>();
    //     self.subscribe_type(type_id, listener);
    // }

    pub fn emit<T: Clone + 'static>(&self, em: Rc<RefCell<EntityManager>>, data: T) {
        let type_id = TypeId::of::<T>();
        if let Some(listeners) = self.listeners.get(&type_id) {
            let event = Event::new(data);
            for listener in listeners {
                listener.on_event(em.clone(), &event);
            }
        }
    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}
