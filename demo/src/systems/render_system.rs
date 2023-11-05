use macroquad::{
    prelude::*,
    texture::{draw_texture_ex, DrawTextureParams},
};
use rust_ecs::systems::{System, SystemBuilder};

use crate::components::{SpriteComponent, TransformComponent};

// The render system prints the player position to the console.
pub fn create_render_system() -> System {
    SystemBuilder::new()
        .require_component::<TransformComponent>()
        .require_component::<SpriteComponent>()
        .with_update(|entities, _, am, em, _| {
            let mut entities = entities
                .iter()
                .map(|entity| {
                    let transform = em.get_component::<TransformComponent>(*entity).unwrap();
                    let sprite = em.get_component::<SpriteComponent>(*entity).unwrap();
                    (transform, sprite)
                })
                .collect::<Vec<_>>();

            entities.sort_by_key(|(_, sprite)| sprite.z_index);

            for (transform, sprite) in entities {
                let texture = am.get_texture(&sprite.sprite_name).unwrap();

                draw_texture_ex(
                    texture,
                    transform.0.x,
                    transform.0.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(sprite.dst_size),
                        source: sprite.src_rect,
                        ..Default::default()
                    },
                );
            }
        })
        .build()
}
