# Domus Technical Specification

**Status:** In Progress
**Document Version:** 1.0
**Last Updated:** 2026-03-19

---

## 1. Core Module: `domus-core`

### 1.1 Signal<T>

**Purpose:** Reactive container for state that automatically notifies subscribers

**API:**
```rust
impl<T: Clone + 'static> Signal<T> {
    /// Create a new signal
    pub fn new(value: T) -> Self;

    /// Read value and register current effect as subscriber
    pub fn get(&self) -> T;

    /// Write value and notify all subscribers
    pub fn set(&self, new_value: T);

    /// Modify value in place
    pub fn update<F: Fn(&mut T)>(&self, f: F);

    /// Create a derived signal (computed)
    pub fn map<U: Clone + 'static, F: Fn(&T) -> U + 'static>(
        &self,
        f: F,
    ) -> Signal<U>;
}
```

**Implementation Details:**
- Internal: `Rc<RefCell<SignalInner<T>>>`
- Thread-safe via `RefCell` (WASM is single-threaded)
- Subscriber list automatically pruned on disposal

**Auto-Tracking Mechanism:**
```rust
pub fn get(&self) -> T {
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
```

### 1.2 Effect

**Purpose:** Reactive closure that automatically re-runs when dependencies change

**API:**
```rust
pub struct Effect {
    execute: Box<dyn Fn()>,
}

impl Effect {
    /// Create and immediately run effect
    pub fn new<F: Fn() + 'static>(f: F) -> Rc<Self>;

    /// Create effect in a specific scope
    pub fn in_scope<F: Fn() + 'static>(
        scope_id: ScopeId,
        f: F,
    ) -> Rc<Self>;
}
```

**Execution Flow:**
1. Save current effect to TLS
2. Execute closure (calls signal.get())
3. Signal registers effect as subscriber
4. Restore TLS

**Cleanup:**
- When Scope is dropped, all Effects unsubscribe from Signals
- Subscribers list uses weak references to avoid circular refs

### 1.3 Runtime & Batching

**Purpose:** Manage effect execution and batch updates

**API:**
```rust
pub fn batch<F: FnOnce()>(f: F);

pub struct BatchedUpdates {
    queue: Vec<Rc<Effect>>,
    in_batch: bool,
}

impl BatchedUpdates {
    fn enqueue(&mut self, effect: Rc<Effect>);
    fn flush(&mut self);
}
```

**Behavior:**
- Multiple `signal.set()` calls inside `batch()` queue effects
- Flush via `requestAnimationFrame` or microtask
- Prevents layout thrashing and visual glitches

---

## 2. Macro Module: `domus-macro`

### 2.1 RSX Parser

**Input Syntax (Dual Mode):**

**Rust-style:**
```rust
domus! {
    div(class: "container", id: "main") {
        span { "Static text" }
        p { "Dynamic: " {some_signal} }
    }
}
```

**HTML-style:**
```rust
domus! {
    <div class="container" id="main">
        <span>Static text</span>
        <p>Dynamic: {some_signal}</p>
    </div>
}
```

**Parser Rules:**
1. Detect opening token: `<` (HTML) or `Ident` (Rust)
2. Parse recursively until closing tag/brace
3. Identify expressions in `{}` as dynamic
4. Validate component names against DomusComponent trait

### 2.2 Code Generation

**Transformation Rules:**

#### Static Elements
```rust
// Input
domus! { div(class: "btn") { "Click" } }

// Output
{
    let __el = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    __el.set_attribute("class", "btn").unwrap();
    let __text = __el.owner_document().unwrap().create_text_node("Click");
    __el.append_child(&__text).unwrap();
    __el
}
```

#### Dynamic Text
```rust
// Input
domus! { span { {count_signal} } }

// Output
{
    let __el = document.create_element("span").unwrap();
    let __dyn = document.create_text_node("");
    __el.append_child(&__dyn).unwrap();

    let __count = count_signal.clone();
    create_effect(move || {
        __dyn.set_text_content(Some(&__count.get().to_string()));
    });
    __el
}
```

#### Dynamic Attributes
```rust
// Input
domus! { button(class: {theme_signal}) { "OK" } }

// Output
{
    let __el = document.create_element("button").unwrap();
    let __text = document.create_text_node("OK");
    __el.append_child(&__text).unwrap();

    let __theme = theme_signal.clone();
    create_effect(move || {
        __el.set_attribute("class", &__theme.get()).unwrap();
    });
    __el
}
```

#### Event Handlers
```rust
// Input
domus! { button(on_click: |_| signal.set(true)) { "Toggle" } }

// Output
{
    let __el = document.create_element("button").unwrap();
    let __text = document.create_text_node("Toggle");
    __el.append_child(&__text).unwrap();

    let __signal = signal.clone();
    __el.set_onclick(Some(&Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        __signal.set(true);
    }) as Box<dyn FnMut(_)>).into_js_value()));

    __el
}
```

### 2.3 Hygiene & Optimization

**Variable Naming:**
- Generated variables use `__` prefix to avoid collision
- Each nested element gets unique counter: `__el_0`, `__el_1`, etc.

**Dead Code Elimination:**
- If no dynamic bindings exist, macro generates zero Effects
- Static HTML-only components produce minimal WASM

---

## 3. Web Module: `domus-web`

### 3.1 DomusComponent Trait

```rust
pub trait DomusComponent {
    type Props;
    type State;

    fn setup(props: Self::Props) -> Self::State;
    fn render(state: &Self::State) -> DomusNode;
}
```

**Constraint:** Render() must only read State, never mutate it

**Lifecycle:**
1. Parent instantiates with `Props`
2. Framework calls `setup(props)` → returns `State`
3. Framework calls `render(&state)` → returns DOM tree
4. Element mounted in browser
5. Effects automatically created and subscribed
6. User interaction may call state mutations
7. Effects re-run automatically
8. Parent unmounts → Scope dropped → Effects unsubscribed

### 3.2 DomusPage Trait

```rust
pub trait DomusPage: DomusComponent {
    fn route() -> &'static str;
    fn title(state: &Self::State) -> String;
    async fn on_load(state: &Self::State);
}
```

**URL Patterns:**
- `/` — home
- `/profile/:id` — dynamic segment
- `/api/:service/*path` — catch-all
- `*` — 404 fallback

**Routing Table (domus_router! macro):**
```rust
domus_router! {
    "/"            => HomePage,
    "/dashboard"   => DashboardPage,
    "/user/:id"    => UserPage,
    "/profile/:id" => ProfilePage,
    "*"            => NotFoundPage,
}
```

**Generated Functions:**
```rust
// Type-safe navigation
pub fn navigate_user(id: u32) {
    router.navigate::<UserPage>(UserProps { id });
}
```

### 3.3 Router Implementation

**Algorithm:**
```
URL changes (popstate event)
    ↓
Extract path and params
    ↓
Match against route patterns
    ↓
Find matching DomusPage struct
    ↓
Unmount previous page (dispose scope)
    ↓
Call Page::setup(props)
    ↓
Call Page::render(&state)
    ↓
Mount into #app container
    ↓
Call Page::on_load(&state)
```

**Memory Management:**
- Each page gets unique ScopeID
- Previous page's scope dropped on navigation
- MutationObserver detects element removal
- All effects unsubscribed automatically

### 3.4 Context API

```rust
pub fn provide_context<T: 'static>(value: T);
pub fn use_context<T: 'static>() -> Option<T>;
```

**Implementation:**
- Context stored in thread-local registry by TypeId
- Scoped to current component and descendants
- Uses `Rc` for sharing, not cloning values

### 3.5 For<T> Component (List Reconciliation)

```rust
pub struct For<T: 'static> {
    items: Signal<Vec<T>>,
    key: fn(&T) -> String,
    render: Box<dyn Fn(&T) -> DomusNode>,
}
```

**Reconciliation Algorithm:**
```
Old list: [A, B, C, D]
New list: [B, A, C, E]

1. Extract keys: old=[A,B,C,D], new=[B,A,C,E]
2. Find common: [B,A,C]
3. Create moves:
   - B: index 1 → 0 (move up)
   - A: index 0 → 1 (move down)
   - C: stays at 2
   - Remove D
   - Insert E at 3
4. Execute DOM mutations
5. Create/destroy scopes only for new/removed items
```

**Usage:**
```rust
domus! {
    ul {
        <For each={self.items} key="id">
            {|item| domus! {
                li { {item.name} }
            }}
        </For>
    }
}
```

---

## 4. CLI Module: `domus-cli`

### 4.1 Commands

**Project Setup:**
```bash
domus new project my-app
# Creates:
# ├── Cargo.toml
# ├── src/main.rs
# ├── src/routes.rs
# ├── src/core/mod.rs
# ├── src/components/
# ├── src/pages/home/
# ├── domus.toml
# └── assets/
```

**Component Generation:**
```bash
domus add component Button
# Creates:
# ├── src/components/button/mod.rs
# │   └── Implements DomusComponent trait
# └── src/components/button/style.css
```

**Page Generation:**
```bash
domus add page Dashboard
# Creates:
# ├── src/pages/dashboard/controller.rs
# ├── src/pages/dashboard/view.rs
# ├── src/pages/dashboard/mod.rs
# └── Adds DashboardPage => route in routes.rs
```

**Development Server:**
```bash
domus dev
# Starts: wasm-pack build --watch
#        HTTP server at localhost:3000
#        Hot reload on file changes
```

**Production Build:**
```bash
domus build --release
# Outputs: dist/
#          ├── index.html
#          ├── app.wasm (minified)
#          ├── app.js
#          └── styles.css (scoped)
```

### 4.2 Boilerplate Templates

**Generated `pages/{name}/controller.rs`:**
```rust
use domus::prelude::*;

pub struct {Name}State {
    // Add reactive state here
}

impl DomusComponent for {Name}Page {
    type Props = ();
    type State = {Name}State;

    fn setup(_: ()) -> Self::State {
        {Name}State {
            // Initialize signals
        }
    }
}
```

---

## 5. Scoped CSS System

### 5.1 Mechanism

**Scope Hash Generation:**
- Hash component file path + content
- Generate `data-domus-{hash:4}` attribute
- Prepend to all selectors in component CSS

**Example:**
```css
/* Input: src/components/button/style.css */
.btn { color: blue; }
.btn:hover { color: darkblue; }

/* Generated (hash = "a1b2"): */
[data-domus="a1b2"] .btn { color: blue; }
[data-domus="a1b2"] .btn:hover { color: darkblue; }
```

**Application:**
```rust
domus! {
    div(data_domus: "a1b2") {  // Macro adds this
        button(class: "btn") { "Click" }
    }
}
```

### 5.2 Benefits

- ✅ No CSS conflicts across components
- ✅ Safe to refactor component styles
- ✅ Automatic via CLI (zero setup)
- ✅ Works with CSS nesting and custom properties

---

## 6. Memory Model

### 6.1 Scope Lifecycle

```rust
pub struct Scope {
    id: ScopeId,
    parent: Option<ScopeId>,
    effects: Vec<Rc<Effect>>,
    element: web_sys::Element,
}

impl Drop for Scope {
    fn drop(&mut self) {
        // Unsubscribe all effects
        for effect in &self.effects {
            // Disconnect from signals
        }
        // Remove element from DOM (if not already)
    }
}
```

### 6.2 Disposal Tracking

**MutationObserver:**
```rust
let observer = MutationObserver::new(|mutations| {
    for mutation in mutations {
        if mutation.removed_nodes().contains(&scope_element) {
            trigger_disposal(scope_id);
        }
    }
});

observer.observe(document.body(), &options);
```

### 6.3 Circular Reference Prevention

- Effects hold `Rc<Signal>` (strong reference)
- Signals hold `Vec<Rc<Effect>>` (strong reference)
- **Breaking the cycle:** Scope drop unsubscribes effects from signals

---

## 7. Type Safety Guarantees

| Guarantee | Mechanism | Example |
|-----------|-----------|---------|
| **Props type-checking** | Rust generics | `Props: MyProps` doesn't compile if wrong type passed |
| **State immutability** | `&` in render() | Cannot mutate state in render(), only in setup() |
| **Route type-safety** | Generated nav functions | `navigate::<UserPage>()` auto-detects required props |
| **Signal cloning** | Auto-clone in macro | No manual `.clone()` needed in closures |
| **Asset verification** | Compile-time macro | `asset!("missing.png")` fails to compile |

---

## 8. Error Handling

### 8.1 Compile-Time Errors

**Macro Validation:**
- Missing required props → compiler error
- Invalid HTML in domus! → compiler error
- Wrong Signal type → type error
- For without key → macro error

### 8.2 Runtime Errors

**Graceful Degradation:**
- Effect panic → caught, logged, effect disabled
- DOM operation failure → caught, logged, continues
- Async task cancelled → ignored (scope disposed)

---

## 9. Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Signal.get() | O(1) | Just reads value + registers effect |
| Signal.set() | O(N) | N = subscriber count |
| Effect re-run | O(1) | Executes single closure |
| For reconciliation | O(N) | N = list size (key-based diffing) |
| Component setup | O(1) | No tree operations |
| Component render | O(1) | No diffing, direct DOM calls |

---

## 10. Testing API

### 10.1 Unit Testing Signals

```rust
#[test]
fn test_signal_update() {
    let count = signal(0);
    let mut effects_run = 0;

    let count_clone = count.clone();
    create_effect(move || {
        assert_eq!(count_clone.get(), 42);
        effects_run += 1;
    });

    count.set(42);
    assert_eq!(effects_run, 2); // initial + update
}
```

### 10.2 Component Testing

```rust
#[test]
fn test_button_component() {
    let state = Button::setup(ButtonProps {
        label: signal("Click".into()),
    });
    let node = Button::render(&state);

    // Verify rendered HTML structure
    assert!(node.to_string().contains("Click"));
}
```

---

## 11. Version Strategy

- **0.1.x** — MVP (core + macro)
- **0.2.x** — Components + routing
- **0.3.x** — Advanced features (For, Context)
- **1.0.0** — Stable API

---

## 12. Known Limitations

1. **Single-page only:** No file-based SSG (initially)
2. **Styling:** No runtime theme switching (CSS variables only)
3. **Accessibility:** Must be manually added (not auto-generated)
4. **Browser support:** WASM-capable browsers only (ES2020+)

---

**Next Steps:**
1. Implementation of domus-core (Signal, Effect, TLS)
2. Implementation of domus-macro (RSX parser)
3. Testing framework setup
4. Example application development
