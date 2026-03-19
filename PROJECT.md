# Domus: A Fine-Grained Reactivity Framework for Rust and WebAssembly

## Executive Summary

**Domus** is a Rust-based web framework that eliminates the Virtual DOM in favor of **fine-grained reactivity** via signals and direct DOM manipulation. It combines the performance of compiled Rust with the developer experience of a convention-over-configuration framework, similar to Rails or NestJS.

Unlike VDOM-based frameworks (Dioxus, Yew), Domus achieves O(1) performance updates by coupling reactive primitives directly to real DOM nodes, with automatic dependency tracking via Thread Local Storage.

---

## Part 1: Architecture Overview

### The Three Pillars of Domus

1. **Fine-Grained Reactivity (The Signal System)**
   - Rust cannot intercept struct field access natively like JavaScript Proxies
   - Solution: Use a **Signal<T>** wrapper around `Rc<RefCell<T>>` for reactive state
   - Automatic dependency tracking via Thread Local Storage (TLS)

2. **Direct DOM Binding (No-VDOM)**
   - No intermediate virtual tree representation
   - Each Signal is bound directly to a real DOM node via `web-sys`
   - Compilation generates imperative DOM calls instead of declarative tree structures

3. **Normative Architecture**
   - Convention-over-configuration structure enforced via Rust traits
   - Strict file system hierarchy prevents "analysis paralysis" and code sprawl
   - Every component type has a designated location and interface

### Architectural Comparison

| Aspect | Dioxus (VDOM) | Domus (Fine-Grained) |
|--------|---------------|----------------------|
| **State Change** | Marks component as "dirty" | Notifies specific subscribers |
| **Update Mechanism** | Tree diffing algorithm | Direct closure execution |
| **Performance** | O(N) based on tree size | O(1) per state change |
| **Complexity** | High (diff algorithm) | High (ownership management) |
| **Bundle Size** | Medium | Very small |

---

## Part 2: The Reactive Core

### 2.1 Thread Local Storage Runtime

The TLS Runtime is the invisible heart of Domus. It tracks which Effect is currently executing, allowing Signals to automatically register dependencies.

```rust
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    /// The effect currently being executed
    pub static RUNNING_EFFECT: RefCell<Option<Rc<Effect>>> = RefCell::new(None);
}
```

**Why this works in WASM:**
- JavaScript is single-threaded
- TLS acts as a secure global singleton for dependency tracking
- Auto-tracking occurs when `signal.get()` is called during effect execution

### 2.2 The Signal<T> Struct

A Signal is a wrapper that stores a value and tracks subscribers.

```rust
pub struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Rc<Effect>>>>,
}

impl<T: Clone + 'static> Signal<T> {
    pub fn get(&self) -> T {
        // AUTO-TRACKING: Register current effect as subscriber
        RUNNING_EFFECT.with(|rt| {
            if let Some(effect) = rt.borrow().as_ref() {
                let mut subs = self.subscribers.borrow_mut();
                if !subs.iter().any(|s| Rc::ptr_eq(s, effect)) {
                    subs.push(Rc::clone(effect));
                }
            }
        });
        self.value.borrow().clone()
    }

    pub fn set(&self, new_val: T) {
        *self.value.borrow_mut() = new_val;
        // NOTIFICATION: Trigger all dependent effects
        for effect in self.subscribers.borrow().iter() {
            (effect.execute)();
        }
    }
}
```

**Key Features:**
- Zero boilerplate: developers write `count.get()` without manual dependency registration
- O(1) notifications: direct closure execution vs. tree traversal
- No false positives: duplicate subscriptions are prevented via `Rc::ptr_eq`

### 2.3 The Effect Struct

An Effect is a reactive closure that automatically tracks its dependencies.

```rust
pub struct Effect {
    pub(crate) execute: Box<dyn Fn()>,
}

impl Effect {
    pub fn new<F: Fn() + 'static>(f: F) -> Rc<Self> {
        let effect = Rc::new(Self {
            execute: Box::new(f),
        });
        // First execution to register dependencies
        Self::run(Rc::clone(&effect));
        effect
    }

    fn run(effect: Rc<Self>) {
        // 1. Place effect in TLS
        RUNNING_EFFECT.with(|rt| *rt.borrow_mut() = Some(Rc::clone(&effect)));

        // 2. Execute (this will call signal.get())
        (effect.execute)();

        // 3. Clean up TLS
        RUNNING_EFFECT.with(|rt| *rt.borrow_mut() = None);
    }
}
```

---

## Part 3: Component Architecture

### 3.1 The DomusComponent Trait

Every component must implement this trait to enforce separation of concerns.

```rust
pub trait DomusComponent {
    /// Data received from parent (Signals or static values)
    type Props;

    /// Internal reactive state
    type State;

    /// Initialization: Transform Props into State
    /// Ideal for async data fetching
    fn setup(props: Self::Props) -> Self::State;

    /// Render: Called ONCE at instantiation
    /// Must return DomusNode representing the component's structure
    fn render(state: &Self::State) -> DomusNode;
}
```

**Enforcement:**
- No logic in `render()`: it only executes once, all logic must be in reactive signals
- Type safety: if Props change, compiler forces `setup()` updates
- Performance: render outputs direct DOM instructions, not virtual trees

### 3.2 Example: UserProfile Component

**File: `src/components/user_profile/mod.rs`**

```rust
use domus::prelude::*;

pub struct UserProfile;

pub struct UserProps {
    pub user_id: u32,
    pub theme: Signal<Theme>,
}

pub struct UserState {
    pub name: Signal<String>,
    pub avatar: Signal<String>,
}

impl DomusComponent for UserProfile {
    type Props = UserProps;
    type State = UserState;

    fn setup(props: Self::Props) -> Self::State {
        let name = signal("Loading...".to_string());
        let avatar = signal("default.png".to_string());

        // Async data fetching tied to component lifecycle
        spawn_local(async move {
            let user = api::fetch_user(props.user_id).await;
            name.set(user.name);
            avatar.set(user.avatar);
        });

        UserState { name, avatar }
    }

    fn render(state: &Self::State) -> DomusNode {
        domus! {
            div(class: "profile-card") {
                img(src: {state.avatar}, alt: "Avatar")
                h2 { {state.name} }
            }
        }
    }
}
```

### 3.3 Props Propagation Strategy

Props are passed by reference (via `Rc`), making the system extremely efficient.

```rust
pub struct ButtonProps {
    pub label: Signal<String>, // Reactive prop
    pub color: String,         // Static prop (immutable)
}

// In parent:
let btn_props = ButtonProps {
    label: self.count_signal.clone(),
    color: "blue".to_string(),
};
let btn_node = Button::setup(btn_props).render();
```

**Key insight:** A child that reads `props.label.get()` registers directly with the parent's Signal. No intermediate re-renders. Parent → Child signal flows directly to Effects.

### 3.4 ReadOnly<T> vs Signal<T>

For safety, Domus distinguishes between read-only and mutable props:

| Type | Usage | Behavior |
|------|-------|----------|
| `ReadOnly<T>` | Descendant data | Child can read, but not modify |
| `Signal<T>` | Bidirectional | Child can modify parent's value |

---

## Part 4: The `domus!` Macro

### 4.1 Hybrid Syntax Support

The macro accepts two syntaxes:

**Rust-style (recommended):**
```rust
domus! {
    div(class: "container") {
        span { "Value: " {self.count} }
        button(on_click: move |_| self.count.increment()) { "+" }
    }
}
```

**HTML-style (for migration):**
```rust
domus! {
    <div class="container">
        <span>Value: {self.count}</span>
        <button on_click={move |_| self.count.increment()}>+</button>
    </div>
}
```

### 4.2 Code Generation

The macro performs three critical transformations:

#### A. Parsing
- Identifies static nodes (created once)
- Identifies dynamic bindings (expressions in `{}`)
- Validates component usage against `DomusComponent` trait

#### B. Static Generation
```rust
let __parent = document.create_element("div").unwrap();
__parent.set_attribute("class", "container").unwrap();
```

#### C. Dynamic Binding (Effect Creation)
For each `{expression}`, the macro generates an Effect:

```rust
// For text content:
let __dyn_text = document.create_text_node("").unwrap();
let __count_clone = count.clone();
create_effect(move || {
    __dyn_text.set_text_content(Some(&__count_clone.get().to_string()));
});

// For attributes:
let __class_clone = class_signal.clone();
create_effect(move || {
    __el.set_attribute("class", &__class_clone.get()).unwrap();
});
```

### 4.3 The "Magic Move" of Domus

The macro automatically clones signals into closures, eliminating boilerplate:

```rust
// Developer writes:
button(on_click: |_| count.set(1)) { "Click" }

// Macro generates (invisible):
{
    let __count = count.clone();
    let __btn = document.create_element("button").unwrap();
    __btn.add_event_listener_with_callback("click",
        move |_| __count.set(1)
    );
    __btn
}
```

---

## Part 5: File System Hierarchy

### 5.1 Standard Structure

```
src/
├── core/               # Runtime (Signals, Effects, TLS)
├── components/         # Reusable UI components
│   ├── button/
│   │   ├── mod.rs      # Logic & DomusComponent trait
│   │   └── style.css   # Scoped styles
│   └── input/
├── pages/              # Main views (routes)
│   ├── home/
│   │   ├── controller.rs  # State & Signals
│   │   └── view.rs        # domus! macro only
│   └── profile/
├── contexts/           # Shared context providers
├── routes.rs           # Centralized routing table
└── main.rs             # WASM entry point

assets/                # Static files
├── images/
└── fonts/

domus.toml            # Framework configuration
Cargo.toml            # Dependencies
```

### 5.2 The Three Golden Rules

| Rule | Application | Why? |
|------|-------------|------|
| **No Local Mut** | Prohibition of `let mut` in `setup()` | All state must be Signals for traceability |
| **Explicit Keys** | Lists require unique `key` attributes | O(1) surgical DOM updates |
| **Folder-as-Module** | One component = one dedicated folder | Enables automatic Scoped CSS |

### 5.3 Module Organization

**File: `src/components/button/mod.rs`**
```rust
use domus::prelude::*;

pub struct Button;

pub struct ButtonProps {
    pub label: Signal<String>,
}

pub struct ButtonState;

impl DomusComponent for Button {
    type Props = ButtonProps;
    type State = ButtonState;
    fn setup(props: Self::Props) -> Self::State { ButtonState }
    fn render(state: &Self::State) -> DomusNode { /* ... */ }
}
```

**File: `src/components/button/style.css`**
```css
/* Auto-scoped to [data-domus="dm-7a2b"] */
.btn { color: red; }
```

---

## Part 6: Routing

### 6.1 The DomusPage Trait

Pages are special components with lifecycle hooks and metadata.

```rust
pub trait DomusPage: DomusComponent {
    /// URL pattern (e.g., "/profile/:id")
    fn route() -> &'static str;

    /// Window title
    fn title(state: &Self::State) -> String;

    /// Pre-fetching hook
    async fn on_load(state: &Self::State);
}
```

### 6.2 Router Configuration

**File: `src/routes.rs`**
```rust
domus_router! {
    "/"         => HomePage,
    "/login"    => LoginPage,
    "/user/:id" => UserProfilePage,
    "*"         => NotFoundPage,
}
```

### 6.3 Typed Navigation

Routes are type-safe, preventing URL typos:

```rust
// Instead of: router.push("/user/42")
// Domus enforces:
state.router.navigate::<UserProfilePage>(UserProps { id: 42 });
```

### 6.4 No-VDOM Router Mechanics

1. **Matching:** URL changes → Find corresponding DomusPage struct
2. **Mounting:** Call `Page::setup()`, inject `render()` output into main slot
3. **Cleanup:** Remove old page from DOM, MutationObserver detects scope removal and destroys all Effects

---

## Part 7: Scoped CSS & Assets

### 7.1 Automatic CSS Scoping

The macro generates a unique hash for each component and scopes CSS accordingly.

**Mechanism:**
1. Macro detects `style.css` in component folder
2. Generates unique ID: `data-domus="dm-7a2b"`
3. Root element receives this attribute
4. CSS compiler transforms selectors to only target that ID

**Your CSS:**
```css
.btn { color: red; }
```

**Compiled output:**
```css
[data-domus="dm-7a2b"] .btn { color: red; }
```

### 7.2 Asset Management

Assets are verified at compile time:

```rust
domus! {
    img(src: asset!("logo.png"), alt: "Logo")
}
```

**Features:**
- File existence verified at compile time
- Automatic cache-busting (hash appended to filename)
- Small SVGs can be inlined directly into DOM

---

## Part 8: Advanced Topics

### 8.1 The For<T> Component (List Reconciliation)

Without a VDOM, lists require special handling via the `For` component:

```rust
domus! {
    ul {
        <For each={self.items} key="id">
            {|item| domus! { li { {item.name} } }}
        </For>
    }
}
```

**Algorithm:**
1. Compare old and new list keys
2. Calculate minimal movements (add, remove, reorder)
3. Use `insertBefore()` and `remove()` for surgical DOM updates
4. Create child Scope for each item's Effects

**Performance:** O(1) per mutation, not O(N)

### 8.2 Context API

Context provides shared state without prop drilling:

```rust
pub struct AuthContext {
    pub user: Signal<Option<User>>,
    pub is_logged: Signal<bool>,
}

// In provider:
fn setup(props: Props) -> State {
    let auth = AuthContext {
        user: signal(None),
        is_logged: signal(false)
    };
    provide_context(auth);
}

// In consumer:
let auth = use_context::<AuthContext>().expect("AuthContext required");
domus! {
    <div>
        {if auth.is_logged.get() { "Welcome!" } else { "Login" }}
    </div>
}
```

**Key advantages over React Context:**
- No intermediate component re-renders
- Context only updated when its Signals change
- Type-safe lookup via `TypeId`

### 8.3 Batching for Performance

Multiple state mutations are grouped into a single render cycle:

```rust
fn update_profile(state: &State) {
    batch(|| {
        state.firstname.set("John".into());
        state.lastname.set("Doe".into());
        state.age.set(30);
    });
    // DOM receives only ONE update notification
}
```

### 8.4 Memory Management & Disposal

Each component gets a unique ScopeID. When removed from DOM:

1. MutationObserver detects element removal
2. Runtime triggers `dispose(scope_id)`
3. All Effects linked to that scope are unsubscribed
4. All WASM memory associated with that scope is freed

**Result:** Zero memory leaks, even with infinite lists.

---

## Part 9: The domus CLI

### 9.1 Command Reference

| Command | Action | Result |
|---------|--------|--------|
| `domus new project my-app` | Bootstrap project | Creates `src/`, `domus.toml`, `routes.rs` |
| `domus add component Button` | Generate component | Creates `components/button/mod.rs` with boilerplate |
| `domus add page Login` | Generate page | Creates `pages/login/controller.rs`, `view.rs` |
| `domus dev` | Development server | Hot reload with WASM compilation |
| `domus build` | Production build | Optimized WASM + minified CSS |
| `domus check` | Static analysis | Validates structure and trait implementations |

### 9.2 Generated Boilerplate

The CLI generates complete, working code:

```rust
// pages/login/controller.rs
use domus::prelude::*;

pub struct LoginState {
    pub email: Signal<String>,
}

impl DomusComponent for LoginPage {
    type Props = ();
    type State = LoginState;

    fn setup(_: ()) -> Self::State {
        LoginState { email: signal(String::new()) }
    }
}
```

---

## Part 10: Testing Strategy

### 10.1 Headless Logic Testing

Test Signal state transitions without a browser:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_logic() {
        let state = LoginPage::setup(());
        state.email.set("test@domus.rs".into());

        assert_eq!(state.email.get(), "test@domus.rs");
    }
}
```

### 10.2 Reactive Binding Tests

Verify that Signal changes trigger Effects correctly:

```rust
#[test]
fn test_reactive_binding() {
    let count = signal(0);
    let mut mock_node = MockElement::new("span");

    let count_clone = count.clone();
    create_effect(move || {
        mock_node.set_text(&format!("Count: {}", count_clone.get()));
    });

    assert_eq!(mock_node.get_text(), "Count: 0");

    count.set(42);
    assert_eq!(mock_node.get_text(), "Count: 42");
}
```

---

## Part 11: Positioning & Marketing

### Domus: The Framework for Rigorous Developers

**Domus** is a Rust-based web framework designed for developers who demand **deterministic performance** without sacrificing **architectural clarity**. Unlike VDOM-based solutions, Domus achieves O(1) updates through fine-grained reactivity and direct DOM manipulation.

### Why Domus?

- **Performance:** No diffing algorithm. State change → Target DOM node update.
- **Architecture:** Convention over configuration. Everyone knows where code belongs.
- **Developer Experience:** A CLI that generates structure, not guesswork.
- **Type Safety:** Routes, props, and assets are type-safe. If it compiles, it works.
- **Maintenance:** Long-term projects become easier, not harder, to maintain.

### The Domus Way

| Need | Solution |
|------|----------|
| Rapid component creation | `domus add component Navbar` |
| Zero CSS conflicts | Automatic scoping per component |
| Type-safe navigation | Generated route functions |
| Debugging clarity | No VDOM, no surprise re-renders |
| Performance assurance | O(1) updates guaranteed by architecture |

---

## Part 12: MVP Implementation Roadmap

### MVP 1: Reactive Core
- [ ] `Signal<T>` and `Effect` working
- [ ] TLS-based dependency tracking
- [ ] Basic WASM32 compilation

### MVP 2: Basic Rendering
- [ ] `domus!` macro supporting static tags and dynamic text
- [ ] Element creation via `web-sys`
- [ ] Simple event listeners

### MVP 3: Component System
- [ ] `DomusComponent` trait implementation
- [ ] Props and State separation
- [ ] Component composition

### MVP 4: Routing & Pages
- [ ] `DomusPage` trait
- [ ] Type-safe navigation
- [ ] URL pattern matching

### MVP 5: Advanced Features
- [ ] `For<T>` component for lists
- [ ] Context API
- [ ] Scoped CSS
- [ ] CLI tool generation

### MVP 6: Production Ready
- [ ] Memory management & disposal
- [ ] Batching scheduler
- [ ] Error boundaries
- [ ] Asset optimization
- [ ] Testing utilities

---

## Part 13: Technical Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Reactivity** | `Rc<RefCell<T>>` + TLS | State and dependency tracking |
| **Runtime** | `SlotMap` / `IndexMap` | Efficient Effect and Node storage |
| **DOM Bridge** | `web-sys` / `wasm-bindgen` | Direct FFI to browser DOM |
| **Syntax** | `proc-macro2` / `quote` / `syn` | RSX parsing and code generation |
| **CLI** | Rust binary | Project scaffolding and generation |

---

## Part 14: Conclusion

**Domus** represents a fundamental shift in how Rust frameworks approach web development. By eliminating the Virtual DOM and embracing fine-grained reactivity, it achieves the performance of hand-written JavaScript while maintaining Rust's type safety and memory guarantees.

The framework's normative architecture ensures that projects scale cleanly. Whether you're building a small single-page app or a large enterprise dashboard, Domus's structure keeps complexity manageable and enables teams to onboard new developers in minutes, not weeks.

This is the framework for developers who want clarity, performance, and confidence in their web applications.
