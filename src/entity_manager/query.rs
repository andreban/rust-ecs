use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
};

use crate::component_signature::ComponentSignature;

use super::{component::Component, EntityId, EntityManager};

pub struct Query<'a, T> {
    pub(crate) phantom: PhantomData<T>,
    pub(crate) em: &'a EntityManager,
}

impl<'a, T> Query<'a, T> {
    pub fn new(em: &'a EntityManager) -> Self {
        Self { phantom: PhantomData, em }
    }
}

impl<'a, A: Component + 'static> Query<'a, Ref<'a, A>> {
    pub fn values(&'a self) -> Vec<Ref<'a, A>> {
        // Get the component ID.
        let component_id = &A::get_type_id();

        // Get the components from the list. Return an empty vector if the list is empty.
        let Some(components) = self.em.inner.borrow().components.get(component_id) else {
            return vec![];
        };

        // Get the components values, borrow the RefCell and cast to type A.
        components
            .values()
            .map(|c| Ref::map(c.borrow(), |f| f.downcast_ref::<A>().unwrap()))
            .collect::<Vec<_>>()
    }
}

impl<'a, A: Component + 'static> From<&'a EntityManager> for Query<'a, Ref<'a, A>> {
    fn from(em: &'a EntityManager) -> Self {
        Self::new(em)
    }
}

impl<'a, A: Component + 'static, B: Component + 'static> Query<'a, (RefMut<'a, A>, Ref<'a, B>)> {
    pub fn values(&'a self) -> Vec<(RefMut<'a, A>, Ref<'a, B>)> {
        // Create a signature for the query, using the IDs for the Component types.
        let mut query_signature = ComponentSignature::default();
        query_signature.require_component::<A>();
        query_signature.require_component::<B>();

        // Filter the entities by the query signature, then map to the component pairs.
        let entities = self
            .em
            .entities
            .values()
            .filter(|e| {
                let Some(entity_signature) = self.em.entity_component_signatures.get(&e.id())
                else {
                    return false;
                };
                query_signature.is_subset(entity_signature)
            })
            .map(|a| get_component::<A, B>(&self.em.components, a.id()))
            .collect::<Vec<_>>();

        entities
    }
}

impl<'a, A: Component + 'static, B: Component + 'static> Query<'a, (Ref<'a, A>, Ref<'a, B>)> {
    pub fn values(&'a self) -> Vec<(Ref<'a, A>, Ref<'a, B>)> {
        // Get the IDs for the Component types.
        let a_id = A::get_type_id();
        let b_id = B::get_type_id();

        // Create a signature for the query, using the IDs for the Component types.
        let mut query_signature = ComponentSignature::default();
        query_signature.require_component::<A>();
        query_signature.require_component::<B>();

        // Filter the entities by the query signature, then map to the component pairs.
        let entities = self
            .em
            .entities
            .values()
            .filter(|e| {
                let Some(entity_signature) = self.em.entity_component_signatures.get(&e.id())
                else {
                    return false;
                };
                query_signature.is_subset(entity_signature)
            })
            .map(|entity| {
                let a = self
                    .em
                    .components
                    .get(&a_id)
                    .unwrap()
                    .get(&entity.id())
                    .unwrap();
                let a = Ref::map(a.borrow(), |f| f.downcast_ref::<A>().unwrap());

                // Get component A from the component storage, then typecast to the correct type.
                let b = self
                    .em
                    .components
                    .get(&b_id)
                    .unwrap()
                    .get(&entity.id())
                    .unwrap();
                let b = Ref::map(b.borrow(), |f| f.downcast_ref::<B>().unwrap());
                (a, b)
            })
            .collect::<Vec<_>>();

        entities
    }
}

impl<'a, A: Component + 'static, B: Component + 'static> From<&'a EntityManager>
    for Query<'a, (RefMut<'a, A>, Ref<'a, B>)>
{
    fn from(em: &'a EntityManager) -> Self {
        Self::new(em)
    }
}

impl<'a, A: Component + 'static, B: Component + 'static> From<&'a EntityManager>
    for Query<'a, (Ref<'a, A>, Ref<'a, B>)>
{
    fn from(em: &'a EntityManager) -> Self {
        Self::new(em)
    }
}

type AnyRefCell = RefCell<Box<dyn Any>>;

// Builds a component pair from the components HashMap.
fn get_component<A: Component + 'static, B: Component + 'static>(
    components: &HashMap<usize, HashMap<usize, AnyRefCell>>,
    entity_id: EntityId,
) -> (RefMut<A>, Ref<B>) {
    let a_id = A::get_type_id();
    let b_id = B::get_type_id();

    // Get component A from the component storage, then typecast to the correct type.
    let a = components.get(&a_id).unwrap().get(&entity_id).unwrap();
    let a = RefMut::map(a.borrow_mut(), |f| f.downcast_mut::<A>().unwrap());

    // Get component A from the component storage, then typecast to the correct type.
    let b = components.get(&b_id).unwrap().get(&entity_id).unwrap();
    let b = Ref::map(b.borrow(), |f| f.downcast_ref::<B>().unwrap());
    (a, b)
}
