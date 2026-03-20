# Changelog

## [Unreleased] — Alpha

### Sprint 6 — CLI, Disposal, Examples
- `domus-cli`: `domus new project <name>`, `domus add component <name>`, `domus add page <name>` via `clap`
- `domus-cli`: project scaffolding generates `Cargo.toml`, `src/lib.rs`, `src/routes.rs`, `index.html`, `.gitignore`
- `domus-web`: MutationObserver-based automatic scope disposal (`disposal.rs`) — WASM-gated, no-op stub for native tests
- `examples/hello-world`: reactive counter + keyed todo list WASM demo with inline CSS

### Sprint 5 — Advanced Features
- `domus-cli`: scoped CSS transformer using FNV-1a hash — prefixes every selector with `[data-domus="<hash>"]`
- `domus-web`: TypeId-based Context API (`provide_context`, `use_context`, `remove_context`, `has_context`)
- `domus-web`: keyed list reconciliation (`diff_keys`) — O(N+M) using `HashMap`, returns `ListPatch` with removes + ops

### Sprint 4 — Routing
- `domus-web`: `RoutePattern` compiles URL patterns into `Vec<Segment>` (Exact / Param / Wildcard)
- `domus-web`: `Router<H>` generic over handler type with `register` and `match_route`
- `domus-web`: `DomusPage` trait extends `DomusComponent` with `route()` and `title()`

### Sprint 3 — Component System
- `domus-web`: `DomusComponent` trait (`Props`, `State`, `setup`, `render`, `mount`)
- `domus-web`: `DomusNode` type alias for `web_sys::Element`
- `domus-web`: `mount_component` helper

### Sprint 2 — RSX Macro & Code Generation
- `domus-macro`: full RSX parser supporting Rust-style (`div(class: "x") { }`) and HTML-style (`<div class="x">`)
- `domus-macro`: static DOM generation via `quote!` — creates `web_sys` elements
- `domus-macro`: dynamic bindings — `create_effect` updates on signal change
- `domus-macro`: event handlers — `Closure::new` + `set_onclick` / `set_oninput` etc. + `forget()`

### Sprint 1 — Reactive Core
- `domus-core`: `Signal<T>` with `get` (dependency tracking) and `set` (subscriber notification)
- `domus-core`: `Effect` with TLS-based automatic dependency registration
- `domus-core`: `batch()` — defers subscriber notification until batch completes
- `domus-core`: `create_scope` / `dispose_scope` / `create_effect_in_scope` for leak-free cleanup
- Signal registry with scope-based unsubscription
