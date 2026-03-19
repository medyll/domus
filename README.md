# Domus: Fine-Grained Reactivity Framework for Rust/WASM

> A web framework that eliminates the Virtual DOM in favor of **direct DOM manipulation** coupled with **automatic dependency tracking**.

## 🎯 Vision

**Domus** is designed for developers who demand:
- **Deterministic Performance** — O(1) updates, no diffing algorithm
- **Type Safety** — Routes, props, and assets type-checked at compile time
- **Clarity** — Convention-over-configuration prevents "analysis paralysis"
- **Simplicity** — No VDOM means the code you write is the code that executes

## 📊 Project Status

```
Planning:     ████████████░░░░░░░░░░░░░░░░░░░░░░ 35% Complete
Development:  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0% (Upcoming)
Testing:      ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0% (Upcoming)
Release:      ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0% (Upcoming)
```

**Artifacts:**
- ✅ Product Requirements (PROJECT.md)
- ✅ Architecture Plan (bmad/artifacts/plan-arch.md)
- ✅ Technical Specification (bmad/artifacts/plan-spec.md)
- 🔄 CLI Specification (in progress)
- 🔄 Test Strategy (in progress)

## 🏗️ Architecture

```
Application Layer
  (Pages + Components)
         ↓
UI Framework Layer
  (Router, Context, For<T>)
         ↓
Macro Layer
  (domus! code generation)
         ↓
Reactive Core
  (Signal, Effect, TLS Runtime)
         ↓
DOM Bridge
  (web-sys, wasm-bindgen)
```

## 🚀 Quick Start (When Ready)

```bash
# Create new project
domus new project my-app

# Add a component
domus add component Button

# Add a page
domus add page Dashboard

# Start development server
domus dev

# Build for production
domus build --release
```

## 💡 Key Concepts

### Signals: Reactive State
```rust
let count = signal(0);
count.set(42);  // Automatically notifies subscribers
```

### Effects: Auto-Tracked Reactions
```rust
create_effect(move || {
    println!("Count is now: {}", count.get());
});
// Runs automatically whenever count changes
```

### Components: Traits Define Structure
```rust
pub trait DomusComponent {
    type Props;
    type State;

    fn setup(props: Self::Props) -> Self::State;
    fn render(state: &Self::State) -> DomusNode;
}
```

### domus! Macro: Declarative UI
```rust
domus! {
    div(class: "counter") {
        p { "Count: " {count} }
        button(on_click: |_| count.set(count.get() + 1)) {
            "Increment"
        }
    }
}
```

## 📁 Project Structure

```
domus/
├── domus-core/      # Signal, Effect, TLS Runtime
├── domus-macro/     # RSX parser and code generation
├── domus-web/       # Component system, Router, DOM bindings
├── domus-cli/       # Code generation tool
├── examples/        # Working example applications
└── bmad/           # Project orchestration artifacts
    ├── status.yaml
    └── artifacts/
        ├── plan-arch.md    # Architecture plan
        ├── plan-spec.md    # Technical specification
        └── stories/        # Implementation stories
```

## 📚 Documentation

- **[PROJECT.md](./PROJECT.md)** — Complete framework specification
- **[Architecture Plan](./bmad/artifacts/plan-arch.md)** — System design and integration points
- **[Technical Spec](./bmad/artifacts/plan-spec.md)** — API signatures and detailed behavior

## 🛠️ Development Roadmap

### MVP 1: Reactive Core ⏳
- [ ] `Signal<T>` implementation
- [ ] `Effect` and TLS-based dependency tracking
- [ ] Basic WASM compilation
- **Estimated:** 2-3 weeks

### MVP 2: Rendering ⏳
- [ ] Basic `domus!` macro (static + dynamic text)
- [ ] Element creation via `web-sys`
- [ ] Event listeners
- **Estimated:** 2-3 weeks

### MVP 3: Components ⏳
- [ ] `DomusComponent` trait
- [ ] Props and State separation
- [ ] Component composition
- **Estimated:** 2-3 weeks

### MVP 4: Routing ⏳
- [ ] `DomusPage` trait
- [ ] URL pattern matching
- [ ] Type-safe navigation
- **Estimated:** 2 weeks

### MVP 5: Advanced Features ⏳
- [ ] `For<T>` list component
- [ ] Context API
- [ ] Scoped CSS
- [ ] CLI tool
- **Estimated:** 3-4 weeks

### MVP 6: Production Ready ⏳
- [ ] Memory management & disposal
- [ ] Batching scheduler
- [ ] Error boundaries
- [ ] Testing utilities
- **Estimated:** 2-3 weeks

## 🎨 Design Philosophy

### Convention Over Configuration
Every file has a designated place. Decisions are made for you:
- Components go in `src/components/`
- Pages go in `src/pages/`
- Styles are automatically scoped

### Three Golden Rules
1. **No Local Mut** — All state must be Signals (enables reactivity)
2. **Explicit Keys** — Lists require unique keys (O(1) updates)
3. **Folder-as-Module** — One component per folder (auto scoped CSS)

### No VDOM
- Direct DOM manipulation via `web-sys`
- O(1) updates (no tree diffing)
- Smaller WASM bundle
- Faster execution

## 📈 Performance Targets

| Metric | Target | How |
|--------|--------|-----|
| Initial WASM Size | < 200KB gzipped | No diffing algorithm |
| Single State Update | < 1ms | O(1) effect execution |
| List of 1000 items | < 100ms | Surgical DOM updates |
| 60 FPS animations | Guaranteed | Batching + scheduling |

## 🔒 Type Safety Guarantees

✅ **Routes:** `navigate::<UserPage>(UserProps { id: 42 })` — type-checked
✅ **Props:** Wrong prop type → compiler error
✅ **Assets:** `asset!("missing.png")` → compile error if file missing
✅ **State:** Cannot mutate in `render()` → compiler error
✅ **Signals:** Must explicitly clone or move — no hidden references

## 🧪 Testing Strategy

**Headless Testing:** Test Signal state transitions without a browser
```rust
#[test]
fn test_counter() {
    let count = signal(0);
    let mut runs = 0;

    let count_clone = count.clone();
    create_effect(move || {
        assert_eq!(count_clone.get(), 42);
        runs += 1;
    });

    count.set(42);
    assert_eq!(runs, 2);
}
```

## 🤝 Contributing

This project is in active development. See [bmad/status.yaml](./bmad/status.yaml) for current progress.

**To get involved:**
1. Review [PROJECT.md](./PROJECT.md) for overall vision
2. Check [bmad/artifacts/](./bmad/artifacts/) for detailed specs
3. Look at implementation stories in [bmad/artifacts/stories/](./bmad/artifacts/stories/)

## 📄 License

MIT

## 👨‍💻 Author

Built with ❤️ and TDAH-friendly structure by the Domus team.

---

**Status:** Pre-alpha (Planning phase)
**Last Updated:** 2026-03-19
**Next Milestone:** Begin MVP 1 (domus-core crate)
