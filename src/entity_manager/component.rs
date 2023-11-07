use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
};

pub type ComponentTypeId = usize;

/// A unique identifier for a component type. This is used when deriving the a component to
/// generate a unique ID for the component type. Check the `component_macro_derive` function in
/// `macros/src/lib.rs` for more information.
pub fn get_next_component_type_id() -> usize {
    static NEXT_TYPE_ID: AtomicUsize = AtomicUsize::new(0);
    NEXT_TYPE_ID.fetch_add(1, Ordering::SeqCst)
}

pub trait Component {
    /// Gets the component type ID. This is used to uniquely identify a component type.
    fn get_type_id() -> usize
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
