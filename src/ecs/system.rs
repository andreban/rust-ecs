use std::any::Any;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::ecs::Registry;

use super::entity::Entity;
use super::{component::Component, Signature};

/// A unique identifier for a system type. This is used when deriving a system to
/// generate a unique ID for the component type. Check the `system_macro_derive` function in
/// `macros/src/lib.rs` for more information.
pub fn get_next_system_type_id() -> usize {
    static NEXT_TYPE_ID: AtomicUsize = AtomicUsize::new(0);
    NEXT_TYPE_ID.fetch_add(1, Ordering::SeqCst)
}

pub trait BaseSystem {
    fn get_type_id() -> usize
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait System: BaseSystem {
    fn update(&self);
    fn signature(&self) -> &Signature;
    fn add_entity(&mut self, entity: Entity);
}

trait Query<T> {
    fn queryz(&self) -> Vec<T>;
}

impl<'a, A: Component, B: Component> Query<(&'a A, &'a B)> for Registry {
    fn queryz(&self) -> Vec<(&'a A, &'a B)> {
        let a_id = A::get_type_id();
        let b_id = B::get_type_id();
        todo!()
    }
}

impl<'a, A: Component, B: Component> Query<(&'a mut A, &'a B)> for Registry {
    fn queryz(&self) -> Vec<(&'a mut A, &'a B)> {
        let a_id = A::get_type_id();
        let b_id = B::get_type_id();
        todo!()
    }
}

impl<'a, A: Component, B: Component, C: Component> Query<(&'a mut A, &'a B, &'a C)> for Registry {
    fn queryz(&self) -> Vec<(&'a mut A, &'a B, &'a C)> {
        let a_id = A::get_type_id();
        let b_id = B::get_type_id();
        let c_id = C::get_type_id();
        todo!()
    }
}
