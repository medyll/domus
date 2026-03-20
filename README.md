# Domus

A reactive UI framework for Rust/WASM — fine-grained reactivity, no Virtual DOM.

> **Status: Alpha** — all 6 MVP sprints complete, 135 tests passing.

```
┌─────────────┐   RSX macros    ┌──────────────┐   signals/effects   ┌──────────────┐
│ domus-macro │ ─────────────►  │  domus-web   │ ──────────────────► │  domus-core  │
│  (proc-mac) │                 │  (DOM + comp)│                     │  (reactive)  │
└─────────────┘                 └──────────────┘                     └──────────────┘
                                       │
                                  domus-cli
                               (scaffold + CSS)
```

## Features

| Feature | Crate | Status |
|---------|-------|--------|
| Fine-grained signals + effects | `domus-core` | ✅ |
| Scope-based effect disposal | `domus-core` | ✅ |
| Batched updates | `domus-core` | ✅ |
| RSX parser (Rust-style + HTML-style) | `domus-macro` | ✅ |
| Reactive code generation via `quote!` | `domus-macro` | ✅ |
| `DomusComponent` trait | `domus-web` | ✅ |
| URL pattern router with params | `domus-web` | ✅ |
| Keyed list reconciliation (O(N)) | `domus-web` | ✅ |
| TypeId-based Context API | `domus-web` | ✅ |
| MutationObserver scope disposal | `domus-web` | ✅ |
| Scoped CSS (FNV-1a hash) | `domus-cli` | ✅ |
| Project scaffolding CLI (`clap`) | `domus-cli` | ✅ |

## Quick Start

```bash
cargo install domus-cli

domus new project my-app
cd my-app
wasm-pack build --target web
npx serve .
```

```bash
domus add component NavBar
domus add page Dashboard
```

## Workspace

```
domus/
├── domus-core/          # Signals, effects, scopes, batching
├── domus-macro/         # RSX proc-macro + code generator
├── domus-web/           # Component, router, context, list, disposal
├── domus-cli/           # scaffold + CSS scoper
└── examples/
    └── hello-world/     # Counter + todo list WASM demo
```

## Core API

### Signals

```rust
use domus_core::signal::signal;

let (value, set_value) = signal(0i32);
value.get();          // read — tracks dependencies automatically
set_value.set(42);    // write — notifies all subscribers
```

### Effects

```rust
use domus_core::effect::create_effect;

create_effect(move || {
    // re-runs whenever any signal read inside changes
    web_sys::console::log_1(&value.get().into());
});
```

### Batching

```rust
use domus_core::batch::batch;

batch(|| {
    set_a.set(1);
    set_b.set(2);
    // subscribers notified once, after the batch
});
```

### Scopes (memory management)

```rust
use domus_core::scope::{create_scope, dispose_scope, create_effect_in_scope};

let scope = create_scope(None);
create_effect_in_scope(scope, move || { /* reactive work */ });
dispose_scope(scope); // unsubscribes all effects — no leaks
```

### Components

```rust
use domus_web::component::{DomusComponent, DomusNode};

pub struct Counter;

impl DomusComponent for Counter {
    type Props = ();
    type State = CounterState;

    fn setup(_: ()) -> CounterState {
        CounterState { count: signal(0).0 }
    }

    fn render(state: &CounterState) -> DomusNode {
        // build and return a web_sys::Element
    }
}
```

### Router

```rust
use domus_web::router::Router;

let mut router: Router<fn(&HashMap<String, String>)> = Router::new();
router.register("/users/:id", handle_user);
router.register("/posts/*", handle_posts);

if let Some((handler, params)) = router.match_route("/users/42") {
    // params["id"] == "42"
}
```

### Context

```rust
use domus_web::context::{provide_context, use_context};

provide_context(AppConfig { theme: "dark".into() });

// anywhere in the tree:
let config = use_context::<AppConfig>().unwrap();
```

### Scoped CSS

```rust
use domus_cli::css_scoper::{generate_scope_hash, scope_css};

let css = ".btn { color: red; }";
let hash = generate_scope_hash("src/Button.rs", css);
let scoped = scope_css(css, &hash);
// → [data-domus="a3f2b1c0"] .btn { color: red; }
```

## hello-world example

```bash
cd examples/hello-world
wasm-pack build --target web
npx serve .
# Open http://localhost:3000
```

Demonstrates: reactive counter, dynamic todo list, event handlers, scope markers.

## Tests

```bash
cargo test --workspace --exclude hello-world
# 135 tests: 35 cli · 8 core · 38 macro · 54 web
```

## Design Philosophy

- **No VDOM** — direct `web_sys` DOM manipulation, O(1) signal updates
- **Scoped by default** — CSS scoped via FNV-1a hash, effects scoped via `ScopeId`
- **Convention over configuration** — components in `src/components/`, pages in `src/pages/`
- **Pure Rust** — no JS build step for the framework itself

## License

MIT
