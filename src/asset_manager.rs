use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// Stores named game assets for use by systems.
///
/// Assets are keyed by both name and type, so two assets of different types can
/// share the same name without conflict. The caller is responsible for loading
/// assets and inserting them before the game loop starts.
///
/// # Example
///
/// ```rust,ignore
/// // At startup (renderer-specific loading is the caller's responsibility):
/// let texture = my_renderer::load_texture("player.png");
/// ecs.asset_manager_mut().insert("player", texture);
///
/// // Inside a system:
/// let texture = asset_manager.get_asset::<MyTexture>("player").unwrap();
/// ```
#[derive(Default)]
pub struct AssetManager {
    assets: HashMap<(String, TypeId), Box<dyn Any>>,
}

impl AssetManager {
    /// Stores `asset` under `name`, replacing any previously stored value of the same type.
    pub fn insert<T: 'static>(&mut self, name: &str, asset: T) {
        self.assets.insert((name.to_string(), TypeId::of::<T>()), Box::new(asset));
    }

    /// Returns a reference to the asset stored under `name` with type `T`.
    ///
    /// Returns `None` if no asset of type `T` is registered under `name`.
    pub fn get_asset<T: 'static>(&self, name: &str) -> Option<&T> {
        self.assets
            .get(&(name.to_string(), TypeId::of::<T>()))
            .and_then(|a| a.downcast_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_then_get_returns_value() {
        let mut am = AssetManager::default();
        am.insert("player", 42u32);
        assert_eq!(am.get_asset::<u32>("player"), Some(&42));
    }

    #[test]
    fn get_unknown_name_returns_none() {
        let am = AssetManager::default();
        assert_eq!(am.get_asset::<u32>("missing"), None);
    }

    #[test]
    fn get_wrong_type_returns_none() {
        let mut am = AssetManager::default();
        am.insert("player", 42u32);
        assert_eq!(am.get_asset::<f32>("player"), None);
    }

    #[test]
    fn same_name_different_types_coexist() {
        let mut am = AssetManager::default();
        am.insert("player", 42u32);
        am.insert("player", "texture_data");
        assert_eq!(am.get_asset::<u32>("player"), Some(&42));
        assert_eq!(am.get_asset::<&str>("player"), Some(&"texture_data"));
    }

    #[test]
    fn insert_same_name_and_type_overwrites() {
        let mut am = AssetManager::default();
        am.insert("player", 42u32);
        am.insert("player", 99u32);
        assert_eq!(am.get_asset::<u32>("player"), Some(&99));
    }
}
