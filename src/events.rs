use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type EventHandler = Box<dyn FnMut(&Box<dyn Any>)>;

#[derive(Default)]
pub struct EventBus {
    listeners: HashMap<TypeId, Vec<EventHandler>>,
}

impl EventBus {
    pub fn add_listener<T: 'static>(&mut self, mut listener: impl FnMut(&T) + 'static) {
        let type_id = std::any::TypeId::of::<T>();
        let listeners = self.listeners.entry(type_id).or_default();
        listeners.push(Box::new(move |e| {
            let data = e.downcast_ref::<T>().unwrap();
            listener(data);
        }));
    }

    pub fn emit<T: 'static>(&mut self, event: T) {
        let type_id = std::any::TypeId::of::<T>();
        if let Some(listeners) = self.listeners.get_mut(&type_id) {
            let wrapped = Box::new(event) as Box<dyn Any + 'static>;
            for listener in listeners {
                listener(&wrapped);
            }
        }
    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}
