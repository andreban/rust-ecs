use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt,
    marker::PhantomData,
};

use crate::component_signature::ComponentSignature;

use super::{component::Component, EntityId, EntityManager};

#[derive(Debug)]
pub enum QueryError {
    ComponentNotFound,
    TypeMismatch,
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::ComponentNotFound => write!(f, "component not found in storage"),
            QueryError::TypeMismatch => write!(f, "stored value could not be downcast to requested type"),
        }
    }
}

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
    pub fn values(&'a self) -> Result<Vec<Ref<'a, A>>, QueryError> {
        let component_id = &A::get_type_id();

        let Some(components) = self.em.inner.borrow().components.get(component_id) else {
            return Ok(vec![]);
        };

        components
            .values()
            .map(|c| {
                Ref::filter_map(c.borrow(), |f| f.downcast_ref::<A>())
                    .map_err(|_| QueryError::TypeMismatch)
            })
            .collect()
    }
}

impl<'a, A: Component + 'static> From<&'a EntityManager> for Query<'a, Ref<'a, A>> {
    fn from(em: &'a EntityManager) -> Self {
        Self::new(em)
    }
}

impl<'a, A: Component + 'static, B: Component + 'static> Query<'a, (RefMut<'a, A>, Ref<'a, B>)> {
    pub fn values(&'a self) -> Result<Vec<(RefMut<'a, A>, Ref<'a, B>)>, QueryError> {
        let mut query_signature = ComponentSignature::default();
        query_signature.require_component::<A>();
        query_signature.require_component::<B>();

        self.em
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
            .collect()
    }
}

impl<'a, A: Component + 'static, B: Component + 'static> Query<'a, (Ref<'a, A>, Ref<'a, B>)> {
    pub fn values(&'a self) -> Result<Vec<(Ref<'a, A>, Ref<'a, B>)>, QueryError> {
        let a_id = A::get_type_id();
        let b_id = B::get_type_id();

        let mut query_signature = ComponentSignature::default();
        query_signature.require_component::<A>();
        query_signature.require_component::<B>();

        self.em
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
                let a_cell = self
                    .em
                    .components
                    .get(&a_id)
                    .and_then(|m| m.get(&entity.id()))
                    .ok_or(QueryError::ComponentNotFound)?;
                let a = Ref::filter_map(a_cell.borrow(), |f| f.downcast_ref::<A>())
                    .map_err(|_| QueryError::TypeMismatch)?;

                let b_cell = self
                    .em
                    .components
                    .get(&b_id)
                    .and_then(|m| m.get(&entity.id()))
                    .ok_or(QueryError::ComponentNotFound)?;
                let b = Ref::filter_map(b_cell.borrow(), |f| f.downcast_ref::<B>())
                    .map_err(|_| QueryError::TypeMismatch)?;

                Ok((a, b))
            })
            .collect()
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

fn get_component<A: Component + 'static, B: Component + 'static>(
    components: &HashMap<usize, HashMap<usize, AnyRefCell>>,
    entity_id: EntityId,
) -> Result<(RefMut<A>, Ref<B>), QueryError> {
    let a_id = A::get_type_id();
    let b_id = B::get_type_id();

    let a_cell = components
        .get(&a_id)
        .and_then(|m| m.get(&entity_id))
        .ok_or(QueryError::ComponentNotFound)?;
    let a = RefMut::filter_map(a_cell.borrow_mut(), |f| f.downcast_mut::<A>())
        .map_err(|_| QueryError::TypeMismatch)?;

    let b_cell = components
        .get(&b_id)
        .and_then(|m| m.get(&entity_id))
        .ok_or(QueryError::ComponentNotFound)?;
    let b = Ref::filter_map(b_cell.borrow(), |f| f.downcast_ref::<B>())
        .map_err(|_| QueryError::TypeMismatch)?;

    Ok((a, b))
}
