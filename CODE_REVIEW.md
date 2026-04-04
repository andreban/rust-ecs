# Code Review: rust-ecs

## Critical — Panics in hot paths

1. ~~**`src/entity_manager/em.rs:157`** — `component.downcast().unwrap()` will panic if the stored component type doesn't match `C`. Similarly at lines 175, 189 where `components.get(...).unwrap()` panics if no component of that type has ever been added.~~ — **Fixed.** `downcast().ok()?` in `get_component`; early `return vec![]` in `query`/`query_mut`; inner downcasts use `filter_map`.

2. ~~**`src/resources.rs:19,25`** — `downcast_ref::<T>().unwrap()` / `downcast_mut::<T>().unwrap()` panic on type mismatch with no user-friendly error.~~ — **Fixed.** Replaced `.map(|r| r.downcast_X().unwrap())` with `.and_then(|r| r.downcast_X())`.

3. ~~**`src/entity_manager/query.rs:36,101-114`** — Multiple `.unwrap()` calls on downcasts and map lookups assume type invariants that aren't enforced.~~ — **Fixed.** Added `QueryError` enum; `values()` now returns `Result<Vec<...>, QueryError>`; map lookups use `.and_then(...).ok_or(QueryError::ComponentNotFound)?`; downcasts use `Ref::filter_map`/`RefMut::filter_map` mapped to `QueryError::TypeMismatch`.

4. ~~**Keyboard input not working**~~ — **Fixed.** Root cause was an upstream miniquad bug: `SetWindowLongPtrA` was never called on `aarch64`, so `win32_wndproc` always read a null display pointer and dropped every window message. Bug filed at not-fl3/miniquad; workaround applied via `[patch.crates-io]` pointing to a local patched copy in `../miniquad-patched`.

**Recommendation:** Return `Option<T>` or a custom error type from all component/resource access methods instead of unwrapping.

---

## High

4. ~~**`src/lib.rs:75-80`** — Systems re-subscribe to the event bus on every frame (after `clear()` wipes listeners). This is wasteful. Subscribe once when a system is added via `add_system()`.~~ — **Fixed.** Moved subscription into `add_system()`; removed `clear()` + re-subscription loop from `update()`.

5. **`src/entity_manager/query.rs:36,104,114`** — `RefCell::borrow()` panics at runtime if a mutable borrow is already held. No protection against double-borrow in system iteration. *(Note: `query.rs` is currently excluded from compilation via `mod.rs`.)*

6. ~~**`demo/src/systems/projectile_lifecycle_system.rs:52`** — `SystemTime::elapsed().unwrap()` panics if the system clock goes backward.~~ — **Fixed.** Replaced `.unwrap()` on `elapsed()` with `.map_or(false, ...)` (clock regression → treat as not expired); replaced `.get_component(...).unwrap()` with `let Some(...) else { continue }`.

---

## Medium

7. ~~**`src/entity_manager/em.rs:176-177, 190-191`** — `.iter().map(|(_, c)| ...)` ignores map keys; use `.values()` instead (Clippy flag).~~ — **Fixed** (as part of `query`/`query_mut` refactor).

8. ~~**`src/events.rs:10`** — Unnecessary parentheses in `Box<(dyn System + 'static)>` — should be `Box<dyn System + 'static>`.~~ — **Fixed.**

9. **Component/entity ID counters** (`src/entity_manager/component.rs`, `entity.rs`) — Sequential `AtomicUsize` IDs are fine for this use case but there's no safeguard if IDs ever overflow 32 (the `FixedBitSet` is capped at 32 components). No assertion or doc warning when exceeding the limit.

---

## Low

10. **`macros/src/lib.rs:20`** — `TYPE_ID.get().unwrap()` — the unwrap is safe since `get_or_init` guarantees initialization, but `get_or_init` + direct use is cleaner.

11. **`src/lib.rs:100,112`** — `em.get_signature(*entity).unwrap()` — called on entities in `entities_to_spawn`/`entities_to_despawn` which are managed by the ECS, so this should be safe, but an `expect()` with a diagnostic message would be more informative.

12. **No public API docs** — None of the public types/methods have `///` doc comments.

---

## Top priorities to fix

1. ~~Replace `.unwrap()` on component/resource access with `Option` returns — these are the most likely to bite users.~~ — **Done.**
2. ~~Move event bus subscription out of the update loop into `add_system()`.~~ — **Done.**
3. Add a compile-time or runtime assertion when component count exceeds 32.
