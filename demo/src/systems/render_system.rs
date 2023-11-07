use std::{cell::RefCell, collections::HashSet, rc::Rc};

use macroquad::{
    prelude::*,
    texture::{draw_texture_ex, DrawTextureParams},
};
use rust_ecs::{
    events::{EventBus, EventListener},
    systems::System,
    Component, Entity, EntityManager, Signature,
};

use crate::components::{SpriteComponent, TransformComponent};

pub struct RenderSystem {
    signature: Signature,
    entities: HashSet<Entity>,
}

impl Default for RenderSystem {
    fn default() -> Self {
        let mut signature = Signature::with_capacity(32);
        signature.set(TransformComponent::get_type_id(), true);
        signature.set(SpriteComponent::get_type_id(), true);
        Self { signature, entities: Default::default() }
    }
}

impl System for RenderSystem {
    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn update(
        &self,
        _delta_time: std::time::Duration,
        asset_manager: &rust_ecs::AssetManager,
        entity_manager: Rc<RefCell<EntityManager>>,
        _event_bus: Rc<RefCell<EventBus>>,
    ) {
        let em = entity_manager.borrow();
        let mut entities = self
            .entities
            .iter()
            .map(|entity| {
                let transform = em.get_component::<TransformComponent>(*entity).unwrap();
                let sprite = em.get_component::<SpriteComponent>(*entity).unwrap();
                (transform, sprite)
            })
            .collect::<Vec<_>>();
        entities.sort_by_key(|(_, sprite)| sprite.z_index);

        for (transform, sprite) in entities {
            let texture = asset_manager.get_texture(&sprite.sprite_name).unwrap();

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
    }
}

impl EventListener for RenderSystem {}
