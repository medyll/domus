//! BackTop component for Domius.
//!
//! A control that appears once the reader has scrolled, and sends them back up.

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::utils::scroll::{follow_scroll, ScrollTarget};

/// BackTop props.
#[derive(Clone, Default)]
pub struct BackTopProps {
    /// CSS class
    pub class: Option<String>,
    /// Visibility height (show after scrolling this far)
    pub visibility_height: u32,
    /// Target container selector
    pub target: Option<String>,
    /// Custom content
    pub content: Option<String>,
}

/// Build a BackTop component.
///
/// The control hides itself until the reader has scrolled past
/// `visibility_height`, and returns the window — or `target`, when given — to
/// the top when activated. It is a real button, so the keyboard reaches it.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::primitives::backtop::{backtop, BackTopProps};
///
/// let backtop_node = backtop(BackTopProps {
///     visibility_height: 400,
///     ..Default::default()
/// });
/// ```
pub fn backtop(props: BackTopProps) -> Element {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let container = document
        .create_element("div")
        .expect("create backtop container");
    let mut classes = vec!["backtop"];
    if let Some(class) = props.class.as_deref() {
        classes.push(class);
    }
    container.set_class_name(&classes.join(" "));
    container
        .set_attribute(
            "data-visibility-height",
            &props.visibility_height.to_string(),
        )
        .expect("set backtop threshold");
    if let Some(target) = props.target.as_deref() {
        container
            .set_attribute("data-target", target)
            .expect("set backtop target");
    }

    let button = document
        .create_element("button")
        .expect("create backtop button");
    button.set_class_name("backtop-button");
    button
        .set_attribute("type", "button")
        .expect("type backtop button");
    button
        .set_attribute("aria-label", "Back to top")
        .expect("label backtop button");
    // The arrow is decoration; the label above carries the meaning.
    let glyph = document
        .create_element("span")
        .expect("create backtop glyph");
    glyph
        .set_attribute("aria-hidden", "true")
        .expect("hide backtop glyph");
    glyph.set_text_content(Some(props.content.as_deref().unwrap_or("↑")));
    button.append_child(&glyph).expect("append backtop glyph");
    container
        .append_child(&button)
        .expect("append backtop button");

    let selector = props.target.clone();
    let activated = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        ScrollTarget::resolve(selector.as_deref()).to_top();
    });
    button
        .add_event_listener_with_callback("click", activated.as_ref().unchecked_ref())
        .expect("listen for backtop activation");
    activated.forget();

    let threshold = f64::from(props.visibility_height);
    let watched = container.clone();
    follow_scroll(props.target.as_deref(), move |target| {
        set_visible(&watched, target.offset() > threshold);
    });

    container
}

fn set_visible(container: &Element, visible: bool) {
    container
        .set_attribute("data-visible", &visible.to_string())
        .expect("expose backtop visibility");
    if visible {
        container.remove_attribute("hidden").expect("show backtop");
    } else {
        container.set_attribute("hidden", "").expect("hide backtop");
    }
}
