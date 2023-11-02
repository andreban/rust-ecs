use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::EntityManager;

type EventHandler = Box<dyn FnMut(&mut EntityManager, &Box<dyn Any>)>;

#[derive(Default)]
pub struct EventBus {
    listeners: HashMap<TypeId, Vec<EventHandler>>,
}

impl EventBus {
    pub fn add_listener<T: 'static>(
        &mut self,
        mut listener: impl FnMut(&mut EntityManager, &T) + 'static,
    ) {
        let type_id = std::any::TypeId::of::<T>();
        let listeners = self.listeners.entry(type_id).or_default();
        listeners.push(Box::new(move |em, e| {
            let data = e.downcast_ref::<T>().unwrap();
            listener(em, data);
        }));
    }

    pub fn emit<T: 'static>(&mut self, em: &mut EntityManager, event: T) {
        let type_id = std::any::TypeId::of::<T>();
        if let Some(listeners) = self.listeners.get_mut(&type_id) {
            let wrapped = Box::new(event) as Box<dyn Any + 'static>;
            for listener in listeners {
                listener(em, &wrapped);
            }
        }
    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}
