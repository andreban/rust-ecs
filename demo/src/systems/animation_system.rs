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
        entity_manager: std::rc::Rc<std::cell::RefCell<rust_ecs::EntityManager>>,
        _event_bus: std::rc::Rc<std::cell::RefCell<rust_ecs::events::EventBus>>,
    ) {
        for entity in &self.entities {
            let em = entity_manager.borrow_mut();
            let mut sprite = em.get_component_mut::<SpriteComponent>(*entity).unwrap();
            let mut animation = em.get_component_mut::<AnimationComponent>(*entity).unwrap();
            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as usize;
            animation.current_frame =
                ((time - animation.start_time) * animation.framerate / 1000) % animation.num_frames;

            sprite.src_rect = Some(Rect::new(
                animation.current_frame as f32 * sprite.dst_size.x,
                0.0,
                sprite.dst_size.x,
                sprite.dst_size.y,
            ));
        }
    }
}

impl EventListener for AnimationSystem {}
