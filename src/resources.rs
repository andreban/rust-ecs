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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_then_get_returns_value() {
        let mut resources = Resources::default();
        resources.put(42u32);
        assert_eq!(resources.get::<u32>(), Some(&42));
    }

    #[test]
    fn get_missing_type_returns_none() {
        let resources = Resources::default();
        assert_eq!(resources.get::<String>(), None);
    }

    #[test]
    fn put_overwrites_existing_value() {
        let mut resources = Resources::default();
        resources.put(1u32);
        resources.put(2u32);
        assert_eq!(resources.get::<u32>(), Some(&2));
    }

    #[test]
    fn get_mut_allows_mutation() {
        let mut resources = Resources::default();
        resources.put(10u32);
        *resources.get_mut::<u32>().unwrap() = 99;
        assert_eq!(resources.get::<u32>(), Some(&99));
    }

    #[test]
    fn different_types_are_independent() {
        let mut resources = Resources::default();
        resources.put(7u32);
        resources.put(String::from("hello"));
        assert_eq!(resources.get::<u32>(), Some(&7));
        assert_eq!(resources.get::<String>(), Some(&String::from("hello")));
    }

    #[test]
    fn default_resources_is_empty() {
        let resources = Resources::default();
        assert_eq!(resources.get::<u32>(), None);
        assert_eq!(resources.get::<String>(), None);
    }
}
