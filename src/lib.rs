//! A minimal Entity-Component-System (ECS) library built on top of macroquad.
//!
//! # Overview
//!
//! The central entry point is [`EntityComponentSystem`], which owns the entity
//! manager, all registered systems, the asset manager, the event bus, and the
//! shared resource store.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use rust_ecs::EntityComponentSystem;
//!
//! let mut ecs = EntityComponentSystem::new();
//! let entity = ecs.create_entity();
//! ecs.add_component(entity, my_component);
//! ecs.add_system(my_system);
//!
//! // inside your game loop:
//! ecs.update(delta_time);
//! ```

mod asset_manager;
mod component_signature;
mod entity_manager;
pub mod events;
mod resources;
pub mod systems;

pub mod derive {
    pub use macros::Component;
}

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    time::Duration,
};

pub use asset_manager::AssetManager;
pub use component_signature::ComponentSignature;
pub use entity_manager::{get_next_component_type_id, Component, Entity, EntityManager, MAX_COMPONENTS};
use events::EventBus;
pub use resources::Resources;
use systems::System;

/// The top-level ECS container.
///
/// Owns all entities, components, systems, assets, events, and resources.
/// Create one instance per game/application and call [`update`](Self::update)
/// every frame.
pub struct EntityComponentSystem {
    entity_manager: EntityManager,
    systems: Vec<Rc<RefCell<Box<dyn System>>>>,
    asset_manager: AssetManager,
    event_bus: Rc<RefCell<EventBus>>,
    resources: Rc<RefCell<Resources>>,
}

impl EntityComponentSystem {
    /// Creates a new, empty [`EntityComponentSystem`].
    pub fn new() -> Self {
        EntityComponentSystem {
            entity_manager: entity_manager::EntityManager::new(),
            systems: Vec::new(),
            asset_manager: AssetManager::default(),
            event_bus: Rc::new(RefCell::new(EventBus::default())),
            resources: Rc::new(RefCell::new(Resources::default())),
        }
    }

    /// Registers a system with the ECS.
    ///
    /// The system is subscribed to any events it declares via
    /// [`System::get_event_type`] at registration time. Entities that already
    /// match the system's [`ComponentSignature`] are **not** retroactively
    /// added; they will be picked up on the next [`update`](Self::update) call
    /// after they are spawned.
    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        let boxed: Rc<RefCell<Box<dyn System>>> = Rc::new(RefCell::new(Box::new(system)));
        {
            let mut eb = self.event_bus.borrow_mut();
            for type_id in boxed.borrow().get_event_type() {
                eb.subscribe_type(*type_id, boxed.clone());
            }
        }
        self.systems.push(boxed);
    }

    /// Advances the ECS by one frame.
    ///
    /// In order, this call:
    /// 1. Commits pending entity spawns and assigns them to matching systems.
    /// 2. Removes entities that were queued for destruction.
    /// 3. Calls [`System::update`] on every registered system.
    pub fn update(&self, delta_time: Duration) {
        {
            let mut em = self.entity_manager.inner.borrow_mut();
            {
                for entity in em.entities_to_spawn.iter() {
                    let entity_signature = em.get_signature(*entity).expect("entity in entities_to_spawn must have a signature");
                    for system in &self.systems {
                        let mut system = system.borrow_mut();
                        if system.signature().is_subset(entity_signature) {
                            system.add_entity(*entity);
                        }
                    }
                }
            }

            for entity in em.entities_to_despawn.iter() {
                for system in &self.systems {
                    let mut system = system.borrow_mut();
                    system.remove_entity(*entity);
                }
            }

            em.update();
        }
        for system in &self.systems {
            system.borrow().update(
                delta_time,
                &self.asset_manager,
                self.entity_manager.clone(),
                self.event_bus.clone(),
                self.resources.clone(),
            );
        }
    }

    /// Creates a new entity and enqueues it for spawning on the next [`update`](Self::update).
    ///
    /// Returns the [`Entity`] handle immediately so components can be attached
    /// before the first update.
    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.inner.borrow_mut().create_entity()
    }

    /// Attaches a component to an entity and updates all system memberships.
    ///
    /// Systems whose [`ComponentSignature`] is now satisfied by the entity's
    /// updated signature will begin receiving the entity.
    pub fn add_component<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        let mut em = self.entity_manager.inner.borrow_mut();
        em.add_component(entity, component);
        let signature = em.get_signature(entity).expect("entity must have a signature after add_component");

        for system in &mut self.systems {
            if system.borrow().signature().is_subset(signature) {
                system.borrow_mut().add_entity(entity);
            }
        }
    }

    /// Detaches a component from an entity and updates all system memberships.
    ///
    /// Systems whose [`ComponentSignature`] is no longer satisfied will stop
    /// receiving the entity.
    pub fn remove_component<C: Component + 'static>(&mut self, entity: Entity) {
        let mut em = self.entity_manager.inner.borrow_mut();
        em.remove_component::<C>(entity);
        let signature = em.get_signature(entity).expect("entity must have a signature after remove_component");
        for system in &mut self.systems {
            if !system.borrow().signature().is_subset(signature) {
                system.borrow_mut().remove_entity(entity);
            }
        }
    }

    /// Returns a shared reference to the [`AssetManager`].
    pub fn asset_manager(&self) -> &AssetManager {
        &self.asset_manager
    }

    /// Returns a mutable reference to the [`AssetManager`].
    pub fn asset_manager_mut(&mut self) -> &mut AssetManager {
        &mut self.asset_manager
    }

    /// Borrows the shared [`Resources`] store immutably.
    pub fn resources(&self) -> Ref<'_, Resources> {
        self.resources.borrow()
    }

    /// Borrows the shared [`Resources`] store mutably.
    pub fn resources_mut(&self) -> RefMut<'_, Resources> {
        self.resources.borrow_mut()
    }

    /// Returns a cloned [`Rc`] handle to the shared [`EventBus`].
    ///
    /// Use this to emit events from outside the system update loop (e.g. from
    /// game-setup code or input handlers).
    pub fn event_bus_cloned(&self) -> Rc<RefCell<EventBus>> {
        self.event_bus.clone()
    }

    /// Returns a cloned [`EntityManager`] handle.
    ///
    /// The returned handle shares the same underlying data via `Rc`, so
    /// mutations are reflected everywhere.
    pub fn entity_manager(&self) -> EntityManager {
        self.entity_manager.clone()
    }
}

impl Default for EntityComponentSystem {
    fn default() -> Self {
        Self::new()
    }
}
