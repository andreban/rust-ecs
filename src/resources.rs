use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// Type-keyed store for singleton resources shared across systems.
///
/// Insert any value with [`put`](Self::put) and retrieve it with
/// [`get`](Self::get) or [`get_mut`](Self::get_mut). At most one value per
/// type `T` can be stored at a time; inserting again overwrites the previous
/// value.
#[derive(Default)]
pub struct Resources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    /// Inserts (or replaces) the resource of type `T`.
    pub fn put<T: Any>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    /// Returns an immutable reference to the resource of type `T`, or `None`
    /// if it has not been inserted.
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|r| r.downcast_ref::<T>())
    }

    /// Returns a mutable reference to the resource of type `T`, or `None` if
    /// it has not been inserted.
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|r| r.downcast_mut::<T>())
    }
}
