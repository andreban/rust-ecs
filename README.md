# rust-ecs [![Rust](https://github.com/andreban/rust-ecs/actions/workflows/rust.yml/badge.svg)](https://github.com/andreban/rust-ecs/actions/workflows/rust.yml)

A simple [Entity Component System][1] (ECS) library written in Rust.

## Goals

The goal of this project is to be a learning exercise for the author, to understand the [Entity Component System][1] architecture and how to implement it with Rust.

The architecture is heavily inspired by the [C++ Game Engine Programming][2] course, but adapted to (try) being more idiomatic to Rust.

## Features

- Entities with arbitrary component combinations
- Signature-based system dispatch (systems are automatically notified when entities match their required components)
- Type-safe component access via `Rc<RefCell<…>>`
- Pub/sub event bus for inter-system communication
- Shared resources accessible by all systems
- Asset manager for loading and caching textures
- Tag and group managers for entity lookup

## Concepts

### Entity

An `Entity` is a lightweight, copyable handle (wrapping a `usize`) that identifies a game object. Entities have no data of their own — data lives in components.

### Component

A `Component` is a plain data struct. Derive the `Component` trait with the provided macro:

```rust
use rust_ecs::derive::Component;

#[derive(Component, Debug)]
pub struct TransformComponent {
    pub position: (f32, f32),
}

#[derive(Component, Debug)]
pub struct VelocityComponent {
    pub velocity: (f32, f32),
}
```

> **Limit:** Up to 32 distinct component types can be registered per ECS instance.

### System

A `System` declares which components it needs via a `ComponentSignature`. The ECS automatically adds or removes entities from a system as components are attached or detached.

Systems must implement both `System` and `EventListener` traits:

```rust
use rust_ecs::{
    events::{Event, EventListener},
    System, ComponentSignature, Entity, EntityManager,
    asset_manager::AssetManager,
    events::EventBus,
    resources::Resources,
};
use std::{cell::RefCell, collections::HashSet, rc::Rc, time::Duration};

pub struct MovementSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
}

impl Default for MovementSystem {
    fn default() -> Self {
        let mut signature = ComponentSignature::default();
        signature.require_component::<TransformComponent>();
        signature.require_component::<VelocityComponent>();
        Self { signature, entities: HashSet::new() }
    }
}

// Required even if the system handles no events.
impl EventListener for MovementSystem {}

impl System for MovementSystem {
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
        delta_time: Duration,
        _asset_manager: &AssetManager,
        em: EntityManager,
        _event_bus: Rc<RefCell<EventBus>>,
        _resources: Rc<RefCell<Resources>>,
    ) {
        for entity in &self.entities {
            let transform = em.get_component::<TransformComponent>(entity).unwrap();
            let velocity = em.get_component::<VelocityComponent>(entity).unwrap();

            let dt = delta_time.as_secs_f32();
            let mut t = transform.borrow_mut();
            let v = velocity.borrow();
            t.position.0 += v.velocity.0 * dt;
            t.position.1 += v.velocity.1 * dt;
        }
    }
}
```

### Events

Systems can emit and receive typed events through the `EventBus`. A system subscribes to event types by returning them from `get_event_type()` and handles them in `on_event()`:

```rust
use std::any::TypeId;

#[derive(Clone)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

pub struct DamageSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
    event_types: [TypeId; 1],
}

impl Default for DamageSystem {
    fn default() -> Self {
        let mut signature = ComponentSignature::default();
        signature.require_component::<HealthComponent>();
        Self {
            signature,
            entities: HashSet::new(),
            event_types: [TypeId::of::<CollisionEvent>()],
        }
    }
}

impl System for DamageSystem {
    fn signature(&self) -> &ComponentSignature { &self.signature }
    fn add_entity(&mut self, entity: Entity) { self.entities.insert(entity); }
    fn remove_entity(&mut self, entity: Entity) { self.entities.remove(&entity); }

    fn get_event_type(&self) -> &[TypeId] {
        &self.event_types
    }
}

impl EventListener for DamageSystem {
    fn on_event(&self, em: EntityManager, event: &Event) {
        if let Some(collision) = event.get_data::<CollisionEvent>() {
            if let Some(health) = em.get_component::<HealthComponent>(&collision.entity_a) {
                health.borrow_mut().hp -= 10;
            }
        }
    }
}
```

To emit an event from a system's `update()`:

```rust
event_bus.borrow().emit(em.clone(), CollisionEvent { entity_a, entity_b });
```

### Resources

`Resources` is a type-keyed store for singleton data shared across all systems:

```rust
// Setup
ecs.resources_mut().put::<GameConfig>(GameConfig::default());

// Inside a system's update()
let config = resources.borrow();
let game_config = config.get::<GameConfig>().unwrap();
```

### Tags and Groups

Entities can be labelled for easy lookup:

```rust
// Tags: one tag per entity (one-to-one)
em.set_tag(player_entity, "player");
let player = em.tag_manager().get_entity("player");

// Groups: many entities per group, many groups per entity (many-to-many)
em.add_entity_to_group(&enemy_entity, "enemies");
```

## Usage

### 1. Create the ECS

```rust
use rust_ecs::EntityComponentSystem;

let mut ecs = EntityComponentSystem::new();
```

### 2. Register Systems

Systems must be registered before entities are created so that signature matching works correctly.

```rust
ecs.add_system(MovementSystem::default());
ecs.add_system(RenderSystem::default());
ecs.add_system(DamageSystem::default());
```

### 3. Spawn Entities

```rust
let player = ecs.create_entity();
ecs.add_component(player, TransformComponent { position: (100.0, 200.0) });
ecs.add_component(player, VelocityComponent { velocity: (0.0, 0.0) });

// Tag for easy retrieval later
ecs.entity_manager().set_tag(player, "player");
```

> **Note:** `create_entity()` queues the entity; it is committed on the next `update()` call.

### 4. Load Assets

```rust
ecs.asset_manager_mut()
    .load_texture("player", "assets/player.png")
    .await
    .unwrap();
```

### 5. Run the Game Loop

```rust
let mut last_frame = std::time::Instant::now();

loop {
    let delta = last_frame.elapsed();
    last_frame = std::time::Instant::now();

    // Emit input events
    ecs.event_bus_cloned()
        .borrow()
        .emit(ecs.entity_manager(), KeyboardEvent { key: KeyCode::Up });

    // Advance the ECS (commits pending entities, updates all systems)
    ecs.update(delta);
}
```

## Running the Demo

The demo project uses assets from the [C++ Game Engine Programming][2] course. Download the assets from the original course and place the `assets` folder inside the `demo/` directory.

```sh
cd demo
cargo run
```

[1]: https://en.wikipedia.org/wiki/Entity_component_system
[2]: https://pikuma.com/courses/cpp-2d-game-engine-development
