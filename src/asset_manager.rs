use std::collections::HashMap;

use macroquad::{texture::Texture2D, Error};

/// Stores named game assets (currently textures) for use by systems.
///
/// Load assets once at startup with [`load_texture`](Self::load_texture), then
/// retrieve them by name inside system update loops with
/// [`get_texture`](Self::get_texture).
#[derive(Default)]
pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
    // fonts: Vec<Font>,
    // sounds: Vec<Sound>,
}

impl AssetManager {
    /// Loads a texture from `path` and stores it under `name`.
    ///
    /// Returns an error if the texture cannot be loaded (e.g. file not found).
    pub async fn load_texture(&mut self, name: &str, path: &str) -> Result<(), Error> {
        let texture = macroquad::texture::load_texture(path).await?;
        self.textures.insert(name.to_string(), texture);
        Ok(())
    }

    /// Returns the texture registered under `name`, or `None` if not loaded.
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }
}
