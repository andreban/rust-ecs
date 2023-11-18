use std::collections::HashSet;

use macroquad::math::Rect;
use rust_ecs::{events::EventListener, systems::System, ComponentSignature, Entity};

use crate::{
    components::{CameraFollowComponent, TransformComponent},
    resources::{Camera, MapDimensions},
};

pub struct CameraFollowSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
}

impl Default for CameraFollowSystem {
    fn default() -> Self {
        let mut signature = ComponentSignature::default();
        signature.require_component::<CameraFollowComponent>();
        signature.require_component::<TransformComponent>();
        Self { signature, entities: Default::default() }
    }
}

impl EventListener for CameraFollowSystem {}

impl System for CameraFollowSystem {
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
        resources: std::rc::Rc<std::cell::RefCell<rust_ecs::Resources>>,
    ) {
        let em = entity_manager.borrow();
        let mut res = resources.borrow_mut();

        let map_dimensions = res.get::<MapDimensions>().unwrap().0;
        let camera = res.get_mut::<Camera>().unwrap();

        let entity = *self.entities.iter().next().unwrap();

        let transform = em.get_component::<TransformComponent>(entity).unwrap();

        camera.0 = {
            let camera_left = (transform.0.x - camera.0.w / 2.0)
                .max(0.0)
                .min(map_dimensions.x);
            let camera_top = (transform.0.y - camera.0.h / 2.0)
                .max(0.0)
                .min(map_dimensions.y);
            Rect::new(camera_left, camera_top, camera.0.w, camera.0.h)
        };
    }
}
