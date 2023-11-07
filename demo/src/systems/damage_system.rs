use std::{any::TypeId, cell::RefCell, collections::HashSet, rc::Rc};

use rust_ecs::{events::EventListener, systems::System, Entity, EntityManager, Signature};

use crate::events::CollisionEvent;

pub struct DamageSystem {
    signature: Signature,
    entities: HashSet<Entity>,
    event_types: [TypeId; 1],
}

impl Default for DamageSystem {
    fn default() -> Self {
        let event_types = [std::any::TypeId::of::<CollisionEvent>()];
        Self { signature: Signature::with_capacity(32), entities: Default::default(), event_types }
    }
}

impl System for DamageSystem {
    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn get_event_type(&self) -> &[std::any::TypeId] {
        self.event_types.as_slice()
    }
}

impl EventListener for DamageSystem {
    fn on_event(&self, em: Rc<RefCell<EntityManager>>, event: &rust_ecs::events::Event) {
        let event = event.get_data::<CollisionEvent>().unwrap();
        let mut em = em.borrow_mut();
        println!(
            "Killing entities {:?} and {:?}",
            event.entity_a, event.entity_b
        );
        em.destroy_entity(event.entity_a);
        em.destroy_entity(event.entity_b);
    }
}
