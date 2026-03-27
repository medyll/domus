# Domus Architecture Plan

**Status:** In Progress
**Last Updated:** 2026-03-19
**Architect Role:** Primary

---

## 1. Architecture Vision

**Domus** is a fine-grained reactivity framework for Rust/WebAssembly that eliminates the Virtual DOM in favor of direct DOM manipulation coupled with automatic dependency tracking.

### Core Philosophy
- **No VDOM:** Direct DOM binding via `web-sys`
- **O(1) Updates:** State change → targeted DOM node update (no tree diffing)
- **Rust Safety:** Type-safe components, routes, and props
- **Convention Over Config:** Enforced file structure and traits prevent "analysis paralysis"

---

## 2. System Architecture

### 2.1 Layered Architecture

```
┌─────────────────────────────────────────┐
│   Application Layer (Pages, Components)  │
│   - DomusComponent trait implementations │
│   - domus! macro usage                   │
└────────────────┬────────────────────────┘
                 │
┌─────────────────────────────────────────┐
│   UI Framework Layer                    │
│   - Router (pages & routes)             │
│   - Scoped CSS system                   │
│   - For<T> (list reconciliation)        │
│   - Context API                         │
└────────────────┬────────────────────────┘
                 │
┌─────────────────────────────────────────┐
│   Macro & Compilation Layer             │
│   - domus! macro (RSX → imperative code)│
│   - Code generation (quote! + syn)      │
│   - Asset macro (asset!)                │
└────────────────┬────────────────────────┘
                 │
┌─────────────────────────────────────────┐
│   Reactive Core Layer                   │
│   - Signal<T> (reactive state)          │
│   - Effect (computed subscriptions)     │
│   - Runtime (TLS + dependency tracking) │
│   - Batching scheduler                  │
└────────────────┬────────────────────────┘
                 │
┌─────────────────────────────────────────┐
│   DOM Bridge Layer                      │
│   - web-sys bindings                    │
│   - wasm-bindgen FFI                    │
│   - MutationObserver (disposal tracking)│
└─────────────────────────────────────────┘
```

### 2.2 Data Flow

```
User Input (click, input, etc.)
    ↓
Event Listener (native browser event)
    ↓
Signal.set(new_value)
    ↓
Notify All Subscribers (Effects)
    ↓
Effect::run() [executes closure]
    ↓
DOM Mutation (set_text_content, set_attribute, etc.)
    ↓
Visual Update (browser reflow)
```

---

## 3. Core Components

### 3.1 Reactive Core (`domius-core` crate)

**Responsibility:** Fine-grained reactivity and dependency tracking

**Key Types:**
```rust
pub struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Rc<Effect>>>>,
}

pub struct Effect {
    execute: Box<dyn Fn()>,
}

thread_local! {
    static RUNNING_EFFECT: RefCell<Option<Rc<Effect>>>;
}
```

**Key Functions:**
- `signal(T) -> Signal<T>` — create reactive state
- `create_effect(F: Fn())` — create auto-tracking effect
- `batch(F: Fn())` — batch multiple mutations into one update

**Features:**
- Auto-tracking via TLS (no manual dependency registration)
- O(1) subscriptions (no tree traversal)
- Batching scheduler to prevent layout thrashing
- Scope-based disposal (prevents memory leaks)

### 3.2 Macro Layer (`domius-macro` crate)

**Responsibility:** Transform declarative syntax into imperative DOM calls

**Key Macro:** `domus!`

**Hybrid Syntax Support:**
```rust
// Rust-style (recommended)
domus! { div(class: "x") { span { {signal} } } }

// HTML-style (for migration)
domus! { <div class="x"><span>{signal}</span></div> }
```

**Code Generation Pipeline:**
1. **Parse** — tokenize RSX into AST
2. **Analyze** — identify static vs. dynamic nodes
3. **Generate** — create web-sys calls for static, Effects for dynamic
4. **Optimize** — omit unnecessary allocations

**Example Transformation:**
```rust
// Input
domus! { span { "Count: " {count} } }

// Generated
{
    let __el = document.create_element("span").unwrap();
    let __static = document.create_text_node("Count: ");
    __el.append_child(&__static).unwrap();

    let __dyn = document.create_text_node("");
    __el.append_child(&__dyn).unwrap();

    let __count_clone = count.clone();
    create_effect(move || {
        __dyn.set_text_content(Some(&__count_clone.get().to_string()));
    });
    __el
}
```

### 3.3 Component System (`domius-web` crate)

**Responsibility:** Component traits, rendering, and lifecycle

**Key Traits:**
```rust
pub trait DomusComponent {
    type Props;
    type State;

    fn setup(props: Self::Props) -> Self::State;
    fn render(state: &Self::State) -> DomusNode;
}

pub trait DomusPage: DomusComponent {
    fn route() -> &'static str;
    fn title(state: &Self::State) -> String;
    async fn on_load(state: &Self::State);
}
```

**Key Types:**
- `DomusNode` — represents a rendered element tree
- `Scope` — manages lifecycle and cleanup for a component
- `Router` — handles page transitions and URL matching

### 3.4 Runtime (`domius-web` crate)

**Responsibility:** Manage Effects, Scopes, and DOM cleanup

**Key Features:**
- **Scope Tree:** Hierarchical scopes for parent-child relationships
- **MutationObserver:** Detect DOM removals and trigger cleanup
- **Disposal:** Automatically unsubscribe Effects when scope is dropped
- **ScopeID:** Unique identifier per component instance

**Cleanup Flow:**
```
Element removed from DOM
    ↓
MutationObserver fires
    ↓
Lookup ScopeID in registry
    ↓
Find all Effects for that scope
    ↓
Unsubscribe from Signals
    ↓
Free WASM memory
```

---

## 4. Crate Structure

```
domus/
├── domius-core/          # Signal, Effect, Runtime, TLS
│   ├── src/
│   │   ├── signal.rs
│   │   ├── effect.rs
│   │   ├── runtime.rs
│   │   ├── scope.rs
│   │   └── lib.rs
│   └── Cargo.toml
│
├── domius-macro/         # Procedural macro for domus!
│   ├── src/
│   │   ├── lib.rs       # proc_macro entry
│   │   ├── parser.rs    # RSX parser (syn)
│   │   ├── codegen.rs   # Code generation (quote!)
│   │   └── transforms.rs
│   └── Cargo.toml
│
├── domius-web/           # Web bindings and components
│   ├── src/
│   │   ├── lib.rs
│   │   ├── component.rs # DomusComponent trait
│   │   ├── page.rs      # DomusPage trait
│   │   ├── router.rs    # Router implementation
│   │   ├── context.rs   # Context API
│   │   ├── list.rs      # For<T> component
│   │   ├── dom.rs       # DOM utilities
│   │   └── scope.rs     # Scope management
│   └── Cargo.toml
│
├── domius-cli/           # Code generation tool
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/
│   │   │   ├── new.rs
│   │   │   ├── add_component.rs
│   │   │   ├── add_page.rs
│   │   │   └── dev.rs
│   │   └── templates/
│   └── Cargo.toml
│
└── examples/            # Full working examples
    ├── hello-world/
    ├── todo-app/
    └── dashboard/
```

---

## 5. File Organization Conventions

### 5.1 Project Structure

```
my-domus-app/
├── src/
│   ├── core/                    # Runtime exports
│   │   └── mod.rs
│   ├── components/              # Reusable components
│   │   ├── button/
│   │   │   ├── mod.rs          # Logic + DomusComponent
│   │   │   └── style.css       # Scoped styles
│   │   └── input/
│   ├── pages/                   # Route pages
│   │   ├── home/
│   │   │   ├── controller.rs   # State + Signals
│   │   │   ├── view.rs         # domus! macro
│   │   │   └── mod.rs          # Exports
│   │   └── profile/
│   ├── contexts/                # Shared context
│   │   ├── auth.rs
│   │   └── mod.rs
│   ├── routes.rs                # Routing table
│   ├── main.rs                  # WASM entry
│   └── lib.rs                   # Public API
├── assets/                      # Static files
├── domus.toml                   # Config
└── Cargo.toml
```

### 5.2 Three Golden Rules

| Rule | Enforcement | Benefit |
|------|------------|---------|
| **No Local Mut** | Compiler error if mutable local in setup() | All state is Signals → traceable |
| **Explicit Keys** | Macro error if For without key | O(1) list updates |
| **Folder-as-Module** | CLI enforces one component per folder | Auto Scoped CSS |

---

## 6. Key Design Decisions

### 6.1 Why No VDOM?

**Trade-offs:**
- **Pros:** O(1) updates, smaller WASM, faster rendering, no reconciliation overhead
- **Cons:** Ownership complexity, manual disposal required, steeper learning curve

**Our Choice:** Performance and simplicity of compiled output outweigh learning curve. Rust's type system catches most errors at compile time.

### 6.2 Why Signal-Based Reactivity?

**Alternatives Considered:**
- Callback hell (React-style): too verbose for Rust ownership
- Declarative functions (Elm): lose connection to DOM updates

**Our Choice:** TLS-based auto-tracking provides magical simplicity without sacrificing safety.

### 6.3 Why Enforce File Structure?

**Alternatives Considered:**
- Full flexibility (React): "where do I put this?"
- Minimal structure: developers create their own patterns

**Our Choice:** Rails-like conventions reduce cognitive load and prevent sprawl at scale.

---

## 7. Integration Points

### 7.1 External Dependencies

**Critical:**
- `wasm-bindgen` — FFI to JavaScript
- `web-sys` — DOM bindings
- `js-sys` — JavaScript utilities

**Macro:**
- `proc-macro2`, `quote`, `syn` — code generation
- `darling` — attribute parsing (future)

**Testing:**
- `wasm-bindgen-test` — WASM test runner
- `headless-chrome` (optional) — integration tests

### 7.2 Asset Pipeline

**Images/Fonts:**
- `asset!("logo.png")` macro verifies existence at compile time
- CLI hashes filenames for cache-busting
- Small SVGs can be inlined directly

**CSS:**
- Scoped via unique `data-domus-{hash}` attribute
- Compiled by CLI (CSS nesting → flat styles)
- Isolated per component

---

## 8. Performance Targets

| Metric | Target | Method |
|--------|--------|--------|
| **Update Latency** | < 16ms (60fps) | O(1) effect execution |
| **Initial WASM** | < 200KB gzipped | No diffing algorithm |
| **Component Setup** | < 1ms | No VDOM reconciliation |
| **List Rendering** | 1000 items in < 100ms | Surgical DOM updates |

---

## 9. Security Considerations

### 9.1 XSS Prevention

- All text automatically escaped via `set_text_content` (never `innerHTML`)
- Attributes validated by type system
- No `eval`-like features

### 9.2 Memory Safety

- Rust's ownership prevents use-after-free
- Scope-based cleanup prevents leaks
- RefCell panics caught and reported

### 9.3 WASM Isolation

- All DOM access goes through wasm-bindgen
- No direct memory manipulation of browser internals
- Sandbox enforced by browser

---

## 10. Roadmap

### Phase 1: Core (MVP 1-2)
- [ ] Signal + Effect + TLS Runtime
- [ ] Basic domus! macro
- [ ] Simple component system
- [ ] web-sys bindings

### Phase 2: Framework (MVP 3-4)
- [ ] DomusComponent trait
- [ ] DomusPage trait + routing
- [ ] Scoped CSS
- [ ] CLI tool generation

### Phase 3: Advanced (MVP 5-6)
- [ ] For<T> list component
- [ ] Context API
- [ ] Batching scheduler
- [ ] Disposal cleanup

### Phase 4: Production (MVP 7+)
- [ ] Error boundaries
- [ ] Asset optimization
- [ ] Testing utils
- [ ] Documentation

---

## 11. Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    User Application                         │
│              Pages + Components (domus!)                    │
└────────────────────────────┬────────────────────────────────┘
                             │
┌────────────────────────────────────────────────────────────┐
│              DomusComponent Trait System                    │
│       Props → State → render() → DomusNode                 │
└────────────────────────┬───────────────────────────────────┘
                         │
┌────────────────────────────────────────────────────────────┐
│                   Signal Reactivity                         │
│    Signal<T> ← Auto-Track → Effect ← TLS Runtime          │
└────────────────────────┬───────────────────────────────────┘
                         │
┌────────────────────────────────────────────────────────────┐
│                    DOM Bridge Layer                         │
│    web-sys::Element ← MutationObserver ← Scope Registry   │
└─────────────────────────────────────────────────────────────┘
```

---

## 12. Success Criteria

✅ **Architecture is successful when:**
- Developers can create components without understanding TLS internals
- Performance scales to 5000+ component trees without slowdown
- No memory leaks even with dynamic list mutations
- Type system prevents 90% of common UI bugs
- New developers onboarded in < 2 hours

---

**Next Steps:**
1. Create technical specifications (crate APIs)
2. Define macro transformation rules
3. Design Router URL matching algorithm
4. Plan disposal and cleanup mechanics
