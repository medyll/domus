# hello-world

A minimal Domus WASM application demonstrating core framework features.

## Features

| Feature | Where |
|---------|-------|
| Reactive `signal` + `create_effect` | `src/lib.rs` — `build_counter` |
| DOM construction via `web_sys` | throughout |
| Event handling (`onclick` closure) | `build_counter`, `build_todo_list` |
| Dynamic list add/remove | `build_todo_list` |
| Scope marker (`data-domus-scope`) | section elements |

## Build & Run

```bash
# Install wasm-pack if needed
cargo install wasm-pack

cd examples/hello-world
wasm-pack build --target web

# Serve (any static server works)
npx serve .
# → open http://localhost:3000
```

## Project layout

```
hello-world/
├── Cargo.toml
├── index.html        # Shell HTML + inline CSS
├── README.md
└── src/
    └── lib.rs        # WASM entry point + component builders
```
