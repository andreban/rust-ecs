---
name: rust-ecs
description: Guide an AI coding agent to use the rust-ecs ECS library correctly, covering components, systems, events, resources, and the game loop.
---

You are helping the user write code using the `rust_ecs` crate — a minimal, single-threaded Entity Component System (ECS) library for Rust. Apply the patterns and constraints below precisely.

{{args}}

---

## Dependency Setup

```toml
[dependencies]
rust_ecs = { git = "https://github.com/andreban/rust-ecs.git" }
```

The crate re-exports the `Component` derive macro. No separate macro crate import is needed.

---

## Core Concepts

| Concept | Type | Purpose |
|---|---|---|
| `EntityComponentSystem` | Struct | Top-level ECS owner; entry point |
| `Entity` | `Copy` struct wrapping `usize` | Lightweight game-object handle |
| `Component` | Trait (via `#[derive(Component)]`) | Plain data struct attached to entities |
| `System` | Trait | Game logic that runs on matching entities |
| `EventBus` | Struct | Pub/sub inter-system communication |
| `Resources` | Struct | Typed singleton store accessible by all systems |
| `AssetManager` | Struct | Named asset store keyed by (name, type) |

**Hard limit**: Maximum **32 distinct component types** per ECS instance. Exceeding this panics.

---

## Defining Components

Use the derive macro — it assigns a unique type ID automatically:

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

---

## Defining Systems

Every system must implement both `System` and `EventListener`. If the system doesn't handle events, use the default `EventListener` impl.

```rust
use rust_ecs::{
    asset_manager::AssetManager,
    events::{Event, EventBus, EventListener},
    systems::System,
    ComponentSignature, Entity, EntityManager,
};
use std::{
    collections::HashSet,
    rc::Rc,
    cell::RefCell,
    time::Duration,
};

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

// Required even if no events are handled
impl EventListener for MovementSystem {}

impl System for MovementSystem {
    fn signature(&self) -> &ComponentSignature { &self.signature }
    fn add_entity(&mut self, entity: Entity) { self.entities.insert(entity); }
    fn remove_entity(&mut self, entity: Entity) { self.entities.remove(&entity); }

    fn update(
        &self,
        delta_time: Duration,
        _assets: &AssetManager,
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

**Key rules for systems:**
- Declare required components in `Default::default()` via `signature.require_component::<C>()`.
- Store matched entities in a `HashSet<Entity>`.
- Access components via `em.get_component::<C>(entity)` → `Option<Rc<RefCell<Box<C>>>>`.
- Always call `.borrow()` or `.borrow_mut()` before reading/writing component data.
- Never borrow the same component mutably twice in the same scope — this will panic at runtime.

---

## Events

```rust
use std::any::TypeId;
use rust_ecs::events::{Event, EventListener};

// Define event (must be Clone)
#[derive(Clone)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: u32,
}

// System that subscribes to events
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
            event_types: [TypeId::of::<DamageEvent>()],
        }
    }
}

impl System for DamageSystem {
    fn signature(&self) -> &ComponentSignature { &self.signature }
    fn add_entity(&mut self, entity: Entity) { self.entities.insert(entity); }
    fn remove_entity(&mut self, entity: Entity) { self.entities.remove(&entity); }
    fn get_event_type(&self) -> &[TypeId] { &self.event_types }
}

impl EventListener for DamageSystem {
    fn on_event(&self, em: EntityManager, event: &Event) {
        if let Some(dmg) = event.get_data::<DamageEvent>() {
            if let Some(health) = em.get_component::<HealthComponent>(&dmg.target) {
                health.borrow_mut().hp = health.borrow().hp.saturating_sub(dmg.amount);
            }
        }
    }
}

// Emitting events (e.g., from another system's update, or main loop)
event_bus.borrow().emit(em.clone(), DamageEvent { target: enemy, amount: 10 });
```

---

## Setup and Game Loop

```rust
use rust_ecs::EntityComponentSystem;

let mut ecs = EntityComponentSystem::new();

// Register systems BEFORE creating entities
ecs.add_system(MovementSystem::default());
ecs.add_system(DamageSystem::default());

// Spawn entities
let player = ecs.create_entity();
ecs.add_component(player, TransformComponent { position: (100.0, 100.0) });
ecs.add_component(player, VelocityComponent { velocity: (50.0, 0.0) });
ecs.entity_manager().set_tag(player, "player");

// Load assets
ecs.asset_manager_mut().insert("player_sprite", some_texture);

// Shared resources
ecs.resources_mut().put::<GameConfig>(GameConfig { difficulty: 1 });

// Game loop
let mut last = std::time::Instant::now();
loop {
    let delta = last.elapsed();
    last = std::time::Instant::now();

    // Emit events before update
    ecs.event_bus_cloned()
        .borrow()
        .emit(ecs.entity_manager(), SomeEvent { ... });

    ecs.update(delta);
}
```

**Lifecycle notes:**
- `create_entity()` and `destroy_entity()` are queued — they take effect on the next `update()`.
- `add_component()` / `remove_component()` trigger immediate signature re-evaluation.
- Register all systems before spawning entities; existing entities are **not** retroactively matched to newly added systems.
- Systems update in the order they were registered.

---

## Resources

```rust
// Store
ecs.resources_mut().put::<Score>(Score { value: 0 });

// Inside system update()
let res = resources.borrow();
let score = res.get::<Score>().unwrap();

// Mutate
drop(res); // release borrow first
resources.borrow_mut().get_mut::<Score>().unwrap().value += 100;
```

---

## Tags and Groups

```rust
// Tags: one-to-one entity ↔ string label
em.set_tag(player, "player");
let player_entity = em.tag_manager().get_entity("player");
let has_tag = em.tag_manager().has_tag(player, "player");

// Groups: many-to-many entity ↔ string label
em.add_entity_to_group(&enemy, "enemies");
let in_group = em.group_manager().entity_in_group(&entity, "enemies");
```

---

## Querying Components

```rust
// All components of type C (no entity context needed)
let healths = em.query::<HealthComponent>();
for h in healths {
    println!("{}", h.borrow().hp);
}

// Entities matching a signature
let mut sig = ComponentSignature::default();
sig.require_component::<TransformComponent>();
sig.require_component::<VelocityComponent>();
let entities = em.get_entities_with_signature(&sig);
```

---

## Common Pitfalls

- **Double borrow panic**: Never hold a `borrow_mut()` and a `borrow()` on the same `Rc<RefCell<...>>` simultaneously.
- **Component limit**: If you define more than 32 component types, the program panics.
- **System registration order**: Systems must be registered before `update()` is called for the first time; entities spawned before a system is registered are not matched.
- **Single-threaded only**: The library uses `Rc<RefCell<...>>` and is not `Send`/`Sync`. Do not move ECS data across threads.
- **Event emission timing**: Emit events before calling `ecs.update()` in the game loop, or emit from within a system's `update()` method.
