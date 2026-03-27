//! Hook to manage focus state of an element.
//!
//! Useful for styling focused inputs, keyboard navigation, and accessibility.

use domius_core::signal::Signal;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, FocusEvent, HtmlElement};

/// Returns signals for focus and blur state of an element.
///
/// Returns a tuple of (is_focused_signal, focus_closure, blur_closure).
/// The closures should be attached to the element's onfocus and onblur events.
///
/// # Example
/// ```ignore
/// let (is_focused, focus_cb, blur_cb) = use_focus();
/// 
/// domius! {
///     input(
///         on_focus: focus_cb,
///         on_blur: blur_cb,
///         class: if is_focused.get() { "focused" } else { "" }
///     )
/// }
/// ```
pub fn use_focus() -> (Signal<bool>, Box<dyn Fn(FocusEvent)>, Box<dyn Fn(FocusEvent)>) {
    use domius_core::signal::signal;
    
    let is_focused = signal(false);
    
    let is_focused_focus = is_focused.clone();
    let focus_closure: Box<dyn Fn(FocusEvent)> = Box::new(move |_event: FocusEvent| {
        is_focused_focus.set(true);
    });
    
    let is_focused_blur = is_focused.clone();
    let blur_closure: Box<dyn Fn(FocusEvent)> = Box::new(move |_event: FocusEvent| {
        is_focused_blur.set(false);
    });
    
    (is_focused, focus_closure, blur_closure)
}

/// Hook that auto-attaches focus listeners to an element.
///
/// Returns a signal indicating whether the element is focused.
///
/// # Example
/// ```ignore
/// let element = document.create_element("input").unwrap();
/// let is_focused = use_focus_auto(&element);
/// ```
pub fn use_focus_auto(element: &Element) -> Signal<bool> {
    use domius_core::signal::signal;
    
    let is_focused = signal(false);
    
    // Focus listener
    let is_focused_focus = is_focused.clone();
    let element_focus = element.clone();
    let focus_closure = Closure::wrap(Box::new(move |_event: Event| {
        is_focused_focus.set(true);
    }) as Box<dyn FnMut(Event)>);
    
    element
        .add_event_listener_with_callback("focus", focus_closure.as_ref().unchecked_ref())
        .expect("Failed to add focus listener");
    focus_closure.forget();
    
    // Blur listener
    let is_focused_blur = is_focused.clone();
    let blur_closure = Closure::wrap(Box::new(move |_event: Event| {
        is_focused_blur.set(false);
    }) as Box<dyn FnMut(Event)>);
    
    element
        .add_event_listener_with_callback("blur", blur_closure.as_ref().unchecked_ref())
        .expect("Failed to add blur listener");
    blur_closure.forget();
    
    is_focused
}

/// Focus an element programmatically.
///
/// # Example
/// ```ignore
/// let input = document.create_element("input").unwrap();
/// focus_element(&input);
/// ```
pub fn focus_element(element: &Element) {
    if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
        html_element.focus().ok();
    }
}

/// Blur an element programmatically.
///
/// # Example
/// ```ignore
/// let input = document.create_element("input").unwrap();
/// blur_element(&input);
/// ```
pub fn blur_element(element: &Element) {
    if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
        html_element.blur().ok();
    }
}

/// Returns a signal that is true when the element is focused,
/// and closures to attach for focus/blur handling.
///
/// This is a simpler version that returns closures as JsValue for direct use.
pub fn use_focus_closures() -> (
    Signal<bool>,
    Closure<dyn FnMut(FocusEvent)>,
    Closure<dyn FnMut(FocusEvent)>,
) {
    use domius_core::signal::signal;
    
    let is_focused = signal(false);
    
    let is_focused_focus = is_focused.clone();
    let focus_closure = Closure::wrap(Box::new(move |_event: FocusEvent| {
        is_focused_focus.set(true);
    }) as Box<dyn FnMut(FocusEvent)>);
    
    let is_focused_blur = is_focused.clone();
    let blur_closure = Closure::wrap(Box::new(move |_event: FocusEvent| {
        is_focused_blur.set(false);
    }) as Box<dyn FnMut(FocusEvent)>);
    
    (is_focused, focus_closure, blur_closure)
}
