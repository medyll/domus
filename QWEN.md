# Domus — Project Context

## Project Overview

**Domus** is a multi-platform Rust UI framework featuring **fine-grained reactivity** via signals and **direct DOM manipulation** (no Virtual DOM). It achieves O(1) updates by coupling reactive primitives directly to real DOM nodes with automatic dependency tracking.

### Key Characteristics
- **Reactive Model**: Signal/Effect architecture with TLS-based automatic dependency tracking
- **Platform Support**: Web (WASM via `domius-web`) and Desktop (Tauri via `domius-desktop`)
- **Core Design**: Platform-agnostic `domius-core` — 100% Rust std, no external dependencies
- **Bundle Size**: Minimal — only platform-specific glue code
- **Performance**: Predictable O(1) updates regardless of component tree size

---

## Workspace Structure

```
domus/
├── domius-core/    # Reactive runtime: Signal, Effect, Scope, batch (100% Rust std)
├── domius-macro/   # RSX proc-macro: parses domus!{} syntax → platform code
├── domius-web/     # Web backend: component system, router, web_sys DOM
├── domius-desktop/ # Desktop backend: Tauri integration, native windows
├── domius-cli/     # CLI scaffolding: domus new/add commands
├── examples/
│   ├── hello-world-web/    # Web/WASM counter demo
│   └── hello-world-tauri/  # Desktop/Tauri counter demo
└── .cargo/config.toml  # Build configuration (opt-level = 3)
```

### Crate Dependencies

```
domius-cli      → (standalone)
domius-core     → (no dependencies - 100% Rust std!)
domius-macro    → (proc-macro, standalone)
domius-web      → domius-core + wasm-bindgen + web-sys
domius-desktop  → domius-core + tauri + serde
hello-world     → domius-web (or) domius-desktop
```

---

## Building and Running

### Prerequisites
```bash
cargo install wasm-pack
```

### Build All Crates
```bash
cargo build --workspace
```

### Run Tests
```bash
cargo test --workspace --exclude hello-world
```
Note: Tests run on native (no WASM runtime). WASM-specific code is gated behind `#[cfg(target_arch = "wasm32")]`.

### Build Example for WASM
```bash
cd examples/hello-world
wasm-pack build --target web
npx serve .
```

### Release Build
```bash
cargo build --release --workspace
```

---

## Core Concepts

### Signal<T>
Reactive cell that tracks readers and notifies subscribers on write.
```rust
let (count, set_count) = signal(0i32);
let current = count.get();   // Tracks if inside effect
set_count.set(current + 1);  // Notifies subscribers
```

### Effect
Auto-tracking closure that re-runs when dependent signals change.
```rust
create_effect(move || {
    web_sys::console::log_1(&count.get().into());
});
```

### Batching
Group multiple writes into single notification cycle.
```rust
batch(|| {
    set_x.set(1);
    set_y.set(2);
    // Effects fire once after closure returns
});
```

### Scopes
Group effects for bulk disposal (prevents memory leaks).
```rust
let scope = create_scope(None);
create_effect_in_scope(scope, move || { /* ... */ });
dispose_scope(scope);  // Unsubscribes all effects in scope
```

---

## Architecture Highlights

### Dependency Tracking (TLS)
`RUNNING_EFFECT` thread-local tracks which effect is executing. When `signal.get()` is called, it registers the current effect as a subscriber.

### Two-Queue Flush System
- **Primary Queue**: Effects ready to execute
- **Secondary Queue**: Effects scheduled during flush (next generation)
- **Generation-based execution**: Prevents diamond-dependency duplicate executions

### Re-entrancy Prevention
`EXECUTED_THIS_GENERATION` tracks effects already run in current generation to prevent infinite loops.

### Disposal (WASM)
`MutationObserver` watches for DOM node removal. Nodes with `data-domus-scope` attribute trigger automatic scope disposal.

---

## Key Files

| File | Purpose |
|------|---------|
| `domius-core/src/signal.rs` | Signal<T> implementation with subscriber tracking |
| `domius-core/src/effect.rs` | Effect struct with TLS-based auto-tracking |
| `domius-core/src/runtime.rs` | Batch system, two-queue flush, re-entrancy prevention |
| `domius-core/src/scope.rs` | Scope system for grouped disposal |
| `domius-web/src/component.rs` | DomiusComponent trait (setup/render separation) |
| `domius-web/src/router.rs` | URL pattern matching (exact/param/wildcard) |
| `domius-web/src/context.rs` | TypeId-based context API |
| `domius-web/src/list.rs` | O(N+M) keyed list reconciliation |
| `domius-macro/src/lib.rs` | RSX parser and codegen |
| `domius-cli/src/main.rs` | CLI commands (new/add) |

---

## Development Conventions

### Code Style
- Rust edition 2021
- `#![warn(missing_docs)]` on library crates
- `Rc<RefCell<T>>` for shared mutable state (single-threaded WASM)
- `Cell<Option<FnMut>>` for Effect closures (avoids RefCell borrow conflicts)

### Testing Practices
- All tests run on native target
- WASM-specific code uses no-op stubs for testing
- Tests cover: signal tracking, effect re-runs, batch deduplication, diamond convergence, re-entrancy, glitch-freedom

### Naming Conventions
- `snake_case` for modules/files
- `PascalCase` for types/traits
- Generated CSS scopes use FNV-1a hash of file path + content

---

## Current Status

**Version**: Alpha (0.1.0)
**Tests**: 154 passing

### Implemented Features
- ✅ Signal/Effect reactive core
- ✅ Batch system with nested batch support
- ✅ Scope-based disposal
- ✅ Diamond convergence (single execution)
- ✅ Re-entrancy prevention
- ✅ Glitch-free updates
- ✅ RSX macro (Rust-style + HTML-style syntax)
- ✅ Component system (DomiusComponent trait)
- ✅ Page system with routing (DomusPage trait)
- ✅ Context API
- ✅ Keyed list reconciliation
- ✅ Automatic CSS scoping
- ✅ CLI scaffolding
- ✅ MutationObserver-based automatic disposal

### Roadmap
- [ ] `cargo clippy` clean pass
- [ ] `cargo doc --no-deps` public API docs
- [ ] crates.io publish
- [ ] Dev server with file watch
- [ ] Error boundaries
- [ ] SSR/hydration support

---

## Common Commands

```bash
# Check code
cargo check --workspace

# Run all tests
cargo test --workspace --exclude hello-world

# Format code
cargo fmt --all

# Build WASM example
cd examples/hello-world && wasm-pack build --target web

# Install CLI (if needed)
cargo install --path domius-cli
```
