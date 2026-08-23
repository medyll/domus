//! Test utilities for WASM component tests.
//!
//! These helpers simplify writing tests that interact with the DOM.

#![cfg(target_arch = "wasm32")]
#![allow(dead_code)]

use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window};

/// Get the global window object.
pub fn window() -> Window {
    web_sys::window().expect("no global window")
}

/// Get the global document object.
pub fn document() -> Document {
    window().document().expect("no document")
}

/// Create an element by tag name.
pub fn create_element(tag: &str) -> HtmlElement {
    document()
        .create_element(tag)
        .expect("failed to create element")
        .dyn_into()
        .expect("element is not HtmlElement")
}

/// Create a div element.
pub fn create_div() -> HtmlElement {
    create_element("div")
}

/// Get element by ID.
pub fn get_element_by_id(id: &str) -> Option<Element> {
    document().get_element_by_id(id)
}

/// Append a child element to a parent.
pub fn append_child(parent: &Element, child: &Element) {
    parent.append_child(child).expect("failed to append child");
}

/// Remove an element from the DOM.
pub fn remove_element(element: &Element) {
    if let Some(parent) = element.parent_node() {
        parent.remove_child(element).ok();
    }
}

/// Create a test container with a unique ID.
/// Returns the container element.
pub fn create_test_container(id: &str) -> HtmlElement {
    let container = create_div();
    container.set_id(id);

    // Append to body
    let body = document()
        .query_selector("body")
        .ok()
        .flatten()
        .unwrap_or_else(|| {
            document()
                .create_element("body")
                .unwrap()
                .dyn_into()
                .unwrap()
        });

    body.dyn_ref::<Element>()
        .unwrap()
        .append_child(container.dyn_ref::<Element>().unwrap())
        .expect("failed to append test container");

    container
}

/// Remove a test container by ID.
pub fn remove_test_container(id: &str) {
    if let Some(container) = get_element_by_id(id) {
        remove_element(&container);
    }
}

/// RAII guard for test containers - automatically cleans up on drop.
pub struct TestContainerGuard {
    id: String,
}

impl TestContainerGuard {
    pub fn new(id: &str) -> Self {
        let _ = create_test_container(id);
        Self { id: id.to_string() }
    }

    pub fn element(&self) -> Option<Element> {
        get_element_by_id(&self.id)
    }
}

impl Drop for TestContainerGuard {
    fn drop(&mut self) {
        remove_test_container(&self.id);
    }
}

/// Wait for a condition to become true with timeout.
/// Returns true if condition was met, false on timeout.
pub async fn wait_for<F>(condition: F, timeout_ms: u64) -> bool
where
    F: Fn() -> bool,
{
    let performance = window()
        .performance()
        .expect("performance API is unavailable");
    let start = performance.now();

    while !condition() {
        if performance.now() - start > timeout_ms as f64 {
            return false;
        }
        // Let pending promise callbacks run before checking again.
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(
            &wasm_bindgen::JsValue::UNDEFINED,
        ))
        .await
        .expect("microtask should resolve");
    }

    true
}

/// Simulate a click event on an element.
pub fn simulate_click(element: &Element) {
    let event_init = web_sys::MouseEventInit::new();
    event_init.set_bubbles(true);
    event_init.set_cancelable(true);

    let event = web_sys::MouseEvent::new_with_mouse_event_init_dict("click", &event_init)
        .expect("failed to create click event");

    element.dispatch_event(&event).ok();
}

/// Simulate keyboard input on an element.
pub fn simulate_key_press(element: &Element, key: &str) {
    let event_init = web_sys::KeyboardEventInit::new();
    event_init.set_key(key);
    event_init.set_bubbles(true);
    event_init.set_cancelable(true);

    let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &event_init)
        .expect("failed to create keyboard event");

    element.dispatch_event(&event).ok();
}

/// Get text content of an element.
pub fn get_text_content(element: &Element) -> String {
    element.text_content().unwrap_or_default()
}

/// Set text content of an element.
pub fn set_text_content(element: &Element, text: &str) {
    element.set_text_content(Some(text));
}

/// Check if element has a class.
pub fn has_class(element: &Element, class: &str) -> bool {
    element.class_list().contains(class)
}

/// Add a class to an element.
pub fn add_class(element: &Element, class: &str) {
    element.class_list().add_1(class).ok();
}

/// Remove a class from an element.
pub fn remove_class(element: &Element, class: &str) {
    element.class_list().remove_1(class).ok();
}

/// Get attribute value from element.
pub fn get_attribute(element: &Element, name: &str) -> Option<String> {
    element.get_attribute(name)
}

/// Set attribute on element.
pub fn set_attribute(element: &Element, name: &str, value: &str) {
    element.set_attribute(name, value).ok();
}
