# Testing Guide for Domius

This document explains how to run tests for the Domius framework.

## Test Structure

Domius has three types of tests:

1. **Native Unit Tests** - Run on your local machine without WASM
2. **WASM Integration Tests** - Run in a browser environment
3. **Component Tests** - Test UI components with real DOM

## Running Tests

### Native Tests (No WASM Required)

```bash
# Run all native tests
cargo test --workspace --exclude hello-world

# Run tests for a specific crate
cargo test -p domius-core
cargo test -p domius-macro
cargo test -p domius-cli

# Run tests with output
cargo test --workspace --exclude hello-world -- --nocapture

# Run specific test
cargo test -p domius-core signal_get_set_works
```

### WASM Tests (Requires wasm-pack)

First, install wasm-pack:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Then run WASM tests:
```bash
# Run all WASM tests in headless Firefox
wasm-pack test --headless --firefox domius-core
wasm-pack test --headless --firefox domius-web

# Run in Chrome
wasm-pack test --headless --chrome domius-web

# Run specific test file
wasm-pack test --headless --firefox domius-web -- button_tests

# Run tests in browser (for debugging)
wasm-pack test --firefox domius-web
```

### Test Coverage by Crate

| Crate | Native Tests | WASM Tests | Description |
|-------|-------------|------------|-------------|
| `domius-core` | ✅ 19 tests | ✅ | Signal, Effect, Scope, Batch, Diamond, Re-entrancy |
| `domius-macro` | ✅ 38 tests | ❌ | RSX parser, codegen |
| `domius-cli` | ✅ 35 tests | ❌ | CSS scoper, scaffold |
| `domius-web` | ✅ 15 tests | ✅ 25 tests | Components, hooks, context, router |

## Writing Tests

### Native Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use domius_core::signal::signal;
    use domius_core::effect::create_effect;

    #[test]
    fn test_signal_get_set() {
        let count = signal(0);
        assert_eq!(count.get(), 0);
        
        count.set(42);
        assert_eq!(count.get(), 42);
    }

    #[test]
    fn test_effect_tracks_dependencies() {
        let count = signal(0);
        let runs = std::cell::RefCell::new(0);
        
        let count_clone = count.clone();
        create_effect(move || {
            let _ = count_clone.get();
            *runs.borrow_mut() += 1;
        });
        
        assert_eq!(*runs.borrow(), 1);
        
        count.set(1);
        assert_eq!(*runs.borrow(), 2);
    }
}
```

### WASM Integration Test Example

```rust
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use domius_web::components::{Button, ButtonProps, ButtonVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_button_creates_element() {
    let props = ButtonProps {
        children: "Click me".to_string(),
        variant: ButtonVariant::Primary,
        size: ButtonSize::Md,
        disabled: false,
        loading: false,
        loading_text: None,
        full_width: false,
        left_icon: None,
        right_icon: None,
        on_click: None,
        class: None,
        button_type: ButtonType::Button,
    };
    
    let (element, _) = Button::create(props);
    
    assert!(element.dyn_ref::<web_sys::HtmlButtonElement>().is_some());
}

#[wasm_bindgen_test]
fn test_button_click_handler() {
    use std::cell::RefCell;
    use std::rc::Rc;
    
    let clicked = Rc::new(RefCell::new(false));
    let clicked_clone = Rc::clone(&clicked);
    
    let props = ButtonProps {
        children: "Click me".to_string(),
        variant: ButtonVariant::Primary,
        size: ButtonSize::Md,
        disabled: false,
        loading: false,
        loading_text: None,
        full_width: false,
        left_icon: None,
        right_icon: None,
        on_click: Some(Box::new(move || {
            *clicked_clone.borrow_mut() = true;
        })),
        class: None,
        button_type: ButtonType::Button,
    };
    
    let (element, _) = Button::create(props);
    
    // Simulate click using test utility
    crate::test_utils::simulate_click(element.dyn_ref().unwrap());
    
    assert!(*clicked.borrow());
}
```

## Test Utilities

The `test_utils` module provides helpers for WASM testing:

```rust
use crate::test_utils::*;

// Create test container (auto-cleanup)
let _guard = TestContainerGuard::new("my-test");

// Create elements
let div = create_div();
let input = create_element("input");

// Simulate events
simulate_click(&element);
simulate_key_press(&element, "Enter");

// Query DOM
assert!(has_class(&element, "my-class"));
assert_eq!(get_text_content(&element), "Hello");
assert_eq!(get_attribute(&element, "data-value"), Some("42".to_string()));
```

## Continuous Integration

Tests run automatically on:
- Every push to `main` branch
- Every pull request

CI runs:
1. Native tests on Linux and Windows
2. WASM tests in headless Firefox
3. WASM example build
4. Documentation build

## Troubleshooting

### "no global window" error
This means you're running WASM tests without a browser. Use `wasm-pack test --headless`.

### Tests timeout
Increase timeout in `wasm-pack test` or check for infinite loops in effects.

### Import errors in WASM tests
Make sure to use `#![cfg(target_arch = "wasm32")]` at the top of WASM test files.

## Future Test Plans

- [ ] Visual regression tests for components
- [ ] Accessibility tests (a11y)
- [ ] Performance benchmarks
- [ ] End-to-end tests with example app
