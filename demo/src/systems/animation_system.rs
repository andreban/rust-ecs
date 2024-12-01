use std::collections::HashSet;

use macroquad::math::Rect;
use rust_ecs::{events::EventListener, systems::System, ComponentSignature, Entity};

use crate::components::{AnimationComponent, SpriteComponent};

pub struct AnimationSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
}

impl Default for AnimationSystem {
    fn default() -> Self {
        let mut signature = ComponentSignature::default();
        signature.require_component::<AnimationComponent>();
        signature.require_component::<SpriteComponent>();
        Self { signature, entities: Default::default() }
    }
}

impl System for AnimationSystem {
    fn signature(&self) -> &ComponentSignature {
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
        _asset_manager: &rust_ecs::AssetManager,
        entity_manager: rust_ecs::EntityManager,
        _event_bus: std::rc::Rc<std::cell::RefCell<rust_ecs::events::EventBus>>,
        _resources: std::rc::Rc<std::cell::RefCell<rust_ecs::Resources>>,
    ) {
        for entity in &self.entities {
            let sprite = entity_manager
                .get_component::<SpriteComponent>(entity)
                .unwrap();
            let animation = entity_manager
                .get_component::<AnimationComponent>(entity)
                .unwrap();
            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as usize;

            let mut sprite = sprite.borrow_mut();
            let mut animation = animation.borrow_mut();

            animation.current_frame =
                ((time - animation.start_time) * animation.framerate / 1000) % animation.num_frames;

            let src_y = if let Some(src_rect) = sprite.src_rect {
                src_rect.y
            } else {
                0.0
            };

            sprite.src_rect = Some(Rect::new(
                animation.current_frame as f32 * sprite.dst_size.x,
                src_y,
                sprite.dst_size.x,
                sprite.dst_size.y,
            ));
        }
    }
}

impl EventListener for AnimationSystem {}
