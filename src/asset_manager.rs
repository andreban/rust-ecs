use std::collections::HashMap;

use macroquad::{texture::Texture2D, Error};

#[derive(Default)]
pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
    // fonts: Vec<Font>,
    // sounds: Vec<Sound>,
}

impl AssetManager {
    pub async fn load_texture(&mut self, name: &str, path: &str) -> Result<(), Error> {
        let texture = macroquad::texture::load_texture(path).await?;
        self.textures.insert(name.to_string(), texture);
        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }
}
