use crate::components::{
    CameraFollowComponent, ProjectileComponent, ProjectileEmitterComponent, SpriteComponent,
    TransformComponent, VelocityComponent,
};
use crate::events::KeyboardEvent;
use macroquad::prelude::KeyCode::Space;
use macroquad::prelude::Vec2;
use rust_ecs::events::{Event, EventBus, EventListener};
use rust_ecs::systems::System;
use rust_ecs::{AssetManager, ComponentSignature, Entity, EntityManager, Resources};
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::{Duration, SystemTime};

pub struct ProjectileEmitterSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
    event_types: [TypeId; 1],
}

impl Default for ProjectileEmitterSystem {
    fn default() -> Self {
        let mut signature = ComponentSignature::default();
        signature.require_component::<ProjectileEmitterComponent>();
        signature.require_component::<TransformComponent>();
        signature.require_component::<SpriteComponent>();
        let event_types = [std::any::TypeId::of::<KeyboardEvent>()];
        Self { signature, entities: Default::default(), event_types }
    }
}

impl EventListener for ProjectileEmitterSystem {
    fn on_event(&self, entity_manager: Rc<RefCell<EntityManager>>, event: &Event) {
        if event.get_data::<KeyboardEvent>().unwrap().0 != Space {
            return;
        }
        for entity in &self.entities {
            let is_player = {
                let em = entity_manager.borrow();
                let is_player = match em.get_component::<CameraFollowComponent>(*entity) {
                    Some(_) => true,
                    None => false,
                };
                is_player
            };

            if !is_player {
                continue;
            }

            let mut em = entity_manager.borrow_mut();
            let projectile_emitter = em
                .get_component_mut::<ProjectileEmitterComponent>(*entity)
                .unwrap();
            let transform = em.get_component::<TransformComponent>(*entity).unwrap();
            let velocity = em.get_component::<VelocityComponent>(*entity).unwrap();
            let sprite = em.get_component::<SpriteComponent>(*entity).unwrap();

            let projectile_transform = TransformComponent(Vec2::new(
                transform.0.x + sprite.dst_size.x / 2.0,
                transform.0.y + sprite.dst_size.y / 2.0,
            ));
            let projectile_velocity = {
                if velocity.0.x > 0.0 {
                    VelocityComponent(Vec2::new(projectile_emitter.projectile_velocity.x, 0.0))
                } else if velocity.0.x < 0.0 {
                    VelocityComponent(Vec2::new(-projectile_emitter.projectile_velocity.x, 0.0))
                } else if velocity.0.y > 0.0 {
                    VelocityComponent(Vec2::new(0.0, projectile_emitter.projectile_velocity.x))
                } else {
                    VelocityComponent(Vec2::new(0.0, -projectile_emitter.projectile_velocity.y))
                }
            };
            let projectile_sprite =
                SpriteComponent::new("bullet", Vec2::new(4.0, 4.0)).with_z_index(4);
            let projectile_duration = ProjectileComponent {
                max_duration: projectile_emitter.projectile_duration,
                created: SystemTime::now(),
                damage: projectile_emitter.damage,
                friendly: projectile_emitter.friendly,
            };
            drop(transform);
            drop(projectile_emitter);
            drop(velocity);
            drop(sprite);

            let projectile = em.create_entity();
            em.group_manager_mut()
                .add_entity_to_group(&projectile, "projectile");
            em.add_component(projectile, projectile_transform);
            em.add_component(projectile, projectile_velocity);
            em.add_component(projectile, projectile_sprite);
            em.add_component(projectile, projectile_duration);
        }
    }
}

impl System for ProjectileEmitterSystem {
    fn signature(&self) -> &ComponentSignature {
        &self.signature
    }

    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn get_event_type(&self) -> &[std::any::TypeId] {
        self.event_types.as_slice()
    }

    fn update(
        &self,
        _delta_time: Duration,
        _asset_manager: &AssetManager,
        entity_manager: Rc<RefCell<EntityManager>>,
        _event_bus: Rc<RefCell<EventBus>>,
        _resources: Rc<RefCell<Resources>>,
    ) {
        let mut em = entity_manager.borrow_mut();
        for entity in &self.entities {
            let mut projectile_emitter = em
                .get_component_mut::<ProjectileEmitterComponent>(*entity)
                .unwrap();
            let transform = em.get_component::<TransformComponent>(*entity).unwrap();
            let sprite = em.get_component::<SpriteComponent>(*entity).unwrap();

            let Some(interval) = projectile_emitter.repeat_interval else {
                continue;
            };

            if projectile_emitter.last_emitted.elapsed().unwrap() < interval {
                continue;
            }

            let projectile_transform = TransformComponent(Vec2::new(
                transform.0.x + sprite.dst_size.x / 2.0,
                transform.0.y + sprite.dst_size.y / 2.0,
            ));
            let projectile_velocity = VelocityComponent(projectile_emitter.projectile_velocity);
            let projectile_sprite =
                SpriteComponent::new("bullet", Vec2::new(4.0, 4.0)).with_z_index(4);
            let projectile_duration = ProjectileComponent {
                max_duration: projectile_emitter.projectile_duration,
                created: SystemTime::now(),
                damage: projectile_emitter.damage,
                friendly: projectile_emitter.friendly,
            };
            projectile_emitter.last_emitted = SystemTime::now();

            drop(sprite);
            drop(transform);
            drop(projectile_emitter);

            let projectile = em.create_entity();
            em.group_manager_mut()
                .add_entity_to_group(&projectile, "projectile");
            em.add_component(projectile, projectile_transform);
            em.add_component(projectile, projectile_velocity);
            em.add_component(projectile, projectile_sprite);
            em.add_component(projectile, projectile_duration);
        }
    }
}
