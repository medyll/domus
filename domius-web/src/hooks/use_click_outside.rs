//! Hook to detect clicks outside a given element.
//!
//! Useful for closing modals, dropdowns, and popovers when clicking outside.

use domius_core::effect::create_effect;
use domius_core::signal::Signal;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, Window};

/// Returns a signal that is true when a click occurred outside the target element.
///
/// # Example
/// ```ignore
/// let element = document.create_element("div").unwrap();
/// let clicked_outside = use_click_outside(&element);
/// create_effect(move || {
///     if clicked_outside.get() {
///         // Close modal/dropdown
///     }
/// });
/// ```
pub fn use_click_outside(element: &Element) -> Signal<bool> {
    use domius_core::signal::signal;
    
    let clicked = signal(false);
    
    let element_weak = element.clone();
    let clicked_clone = clicked.clone();
    
    let closure = Closure::wrap(Box::new(move |event: Event| {
        // Check if the click target is outside our element
        let target = event.target().and_then(|t| t.dyn_into::<Element>().ok());
        
        if let Some(target_el) = target {
            // Check if target is the element or a descendant
            let is_inside = element_weak.contains(Some(&target_el));
            if !is_inside {
                clicked_clone.set(true);
            } else {
                clicked_clone.set(false);
            }
        }
    }) as Box<dyn FnMut(Event)>);
    
    if let Some(window) = web_sys::window() {
        window
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .expect("Failed to add click outside listener");
        closure.forget();
    }
    
    clicked
}

/// Hook that triggers a callback when clicking outside an element.
///
/// # Example
/// ```ignore
/// let element = document.create_element("div").unwrap();
/// use_click_outside_with_callback(&element, || {
///     // Close modal
/// });
/// ```
pub fn use_click_outside_with_callback<F>(element: &Element, callback: F)
where
    F: Fn() + 'static,
{
    let clicked = use_click_outside(element);
    
    create_effect(move || {
        if clicked.get() {
            callback();
        }
    });
}
