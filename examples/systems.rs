use std::cell::Ref;
use std::marker::PhantomData;

use glam::Vec2;
use rust_ecs::entity::{Component, EntityManager};
use rust_ecs::EntityComponentSystem;

#[derive(rust_ecs::derive::Component, Debug)]
pub struct TransformComponent(glam::Vec2);

#[derive(rust_ecs::derive::Component, Debug)]
pub struct RigidBodyComponent(glam::Vec2);

struct Strubbles<'a, T> {
    phantom: PhantomData<T>,
    em: &'a EntityManager,
}

impl <'a, A: Component + 'static> Strubbles<'a, Ref<'a, A>> {
    pub fn new(em: &'a EntityManager) -> Self {
        Self { phantom: PhantomData, em  }
    }

    pub fn values(&'a self) -> Vec<Ref<'a, A>> {
        // Get the component ID.
        let component_id = &A::get_type_id();

        // Get the components from the list. Return an empty vector if the list is empty.
        let Some(components) = self.em.components.get(component_id) else {
            return vec![];
        };

        // Get the components values, borrow the RefCell and cast to type A.
        components
            .values()
            .map(|c| Ref::map(c.borrow(), |f| f.downcast_ref::<A>().unwrap()))
            .collect::<Vec<_>>()
    }
}

impl <'a, A:Component + 'static> From<&'a EntityManager> for Strubbles<'a, Ref<'a, A>> {
    fn from(em: &'a EntityManager) -> Self {
        Self::new(em)
    }
}

fn ble(objects: Strubbles<Ref<TransformComponent>>) {
    println!("ble");
    let objs = objects.values();
    for o in objs {
        println!("{:?}", o);
    }
}

fn ble2(query: Strubbles<Ref<RigidBodyComponent>>) {
    println!("ble2");
    let objs = query.values();
    for o in objs {
        println!("{:?}", o);
    }
}

fn main() {
    let mut ecs = EntityComponentSystem::new();

    let mut systems: Vec<Box<dyn Fn(&EntityManager)>> = vec![];
    systems.push(Box::new(|em: &EntityManager| ble(em.into())));
    systems.push(Box::new(|em: &EntityManager| ble2(em.into())));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::ZERO))
        .add_component(RigidBodyComponent(Vec2::ONE));

    ecs.entity_manager
        .create_entity()
        .add_component(TransformComponent(Vec2::new(100.0, 100.0)))
        .add_component(RigidBodyComponent(Vec2::ONE));
        
    ecs.update();
    
    for system in systems {
        system(&ecs.entity_manager);
    }
}
