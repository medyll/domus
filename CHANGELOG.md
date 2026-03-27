# Changelog

## [Unreleased] — Alpha

### Sprint 6 — CLI, Disposal, Examples
- `domius-cli`: `domus new project <name>`, `domus add component <name>`, `domus add page <name>` via `clap`
- `domius-cli`: project scaffolding generates `Cargo.toml`, `src/lib.rs`, `src/routes.rs`, `index.html`, `.gitignore`
- `domius-web`: MutationObserver-based automatic scope disposal (`disposal.rs`) — WASM-gated, no-op stub for native tests
- `examples/hello-world`: reactive counter + keyed todo list WASM demo with inline CSS

### Sprint 5 — Advanced Features
- `domius-cli`: scoped CSS transformer using FNV-1a hash — prefixes every selector with `[data-domus="<hash>"]`
- `domius-web`: TypeId-based Context API (`provide_context`, `use_context`, `remove_context`, `has_context`)
- `domius-web`: keyed list reconciliation (`diff_keys`) — O(N+M) using `HashMap`, returns `ListPatch` with removes + ops

### Sprint 4 — Routing
- `domius-web`: `RoutePattern` compiles URL patterns into `Vec<Segment>` (Exact / Param / Wildcard)
- `domius-web`: `Router<H>` generic over handler type with `register` and `match_route`
- `domius-web`: `DomusPage` trait extends `DomusComponent` with `route()` and `title()`

### Sprint 3 — Component System
- `domius-web`: `DomusComponent` trait (`Props`, `State`, `setup`, `render`, `mount`)
- `domius-web`: `DomusNode` type alias for `web_sys::Element`
- `domius-web`: `mount_component` helper

### Sprint 2 — RSX Macro & Code Generation
- `domius-macro`: full RSX parser supporting Rust-style (`div(class: "x") { }`) and HTML-style (`<div class="x">`)
- `domius-macro`: static DOM generation via `quote!` — creates `web_sys` elements
- `domius-macro`: dynamic bindings — `create_effect` updates on signal change
- `domius-macro`: event handlers — `Closure::new` + `set_onclick` / `set_oninput` etc. + `forget()`

### Sprint 1 — Reactive Core
- `domius-core`: `Signal<T>` with `get` (dependency tracking) and `set` (subscriber notification)
- `domius-core`: `Effect` with TLS-based automatic dependency registration
- `domius-core`: `batch()` — defers subscriber notification until batch completes
- `domius-core`: `create_scope` / `dispose_scope` / `create_effect_in_scope` for leak-free cleanup
- Signal registry with scope-based unsubscription
