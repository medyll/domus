//! Affix component for Domius.
//!
//! Keeps its contents in view once the reader scrolls past where they sat.

use web_sys::Element;

use crate::utils::scroll::follow_scroll;

/// Affix props.
#[derive(Clone, Default)]
pub struct AffixProps {
    /// Offset from top (in px)
    pub offset_top: u32,
    /// Offset from bottom (in px)
    pub offset_bottom: Option<u32>,
    /// CSS class
    pub class: Option<String>,
    /// Target container selector
    pub target: Option<String>,
}

/// Build an Affix component wrapper.
///
/// Once the reader scrolls past `offset_top`, the wrapper is marked
/// `data-affixed` and its placeholder takes over the space the contents used to
/// occupy, so the page does not jump. Where an affixed block actually sits is
/// the stylesheet's business; this only says when.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::primitives::affix::{affix, AffixProps};
///
/// let affix_node = affix(AffixProps {
///     offset_top: 64,
///     ..Default::default()
/// });
/// ```
pub fn affix(props: AffixProps) -> Element {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let container = document
        .create_element("div")
        .expect("create affix container");
    let mut classes = vec!["affix"];
    if let Some(class) = props.class.as_deref() {
        classes.push(class);
    }
    container.set_class_name(&classes.join(" "));
    container
        .set_attribute("data-offset-top", &props.offset_top.to_string())
        .expect("set affix top offset");
    if let Some(offset_bottom) = props.offset_bottom {
        container
            .set_attribute("data-offset-bottom", &offset_bottom.to_string())
            .expect("set affix bottom offset");
    }
    if let Some(target) = props.target.as_deref() {
        container
            .set_attribute("data-target", target)
            .expect("set affix target");
    }

    let placeholder = document
        .create_element("div")
        .expect("create affix placeholder");
    placeholder.set_class_name("affix-placeholder");
    placeholder
        .set_attribute("aria-hidden", "true")
        .expect("hide affix placeholder");
    placeholder
        .set_attribute("hidden", "")
        .expect("collapse affix placeholder");
    container
        .append_child(&placeholder)
        .expect("append affix placeholder");

    let threshold = f64::from(props.offset_top);
    let watched = container.clone();
    let reserved = placeholder.clone();
    follow_scroll(props.target.as_deref(), move |target| {
        let affixed = target.offset() > threshold;
        watched
            .set_attribute("data-affixed", &affixed.to_string())
            .expect("expose affix state");
        if affixed {
            reserved
                .remove_attribute("hidden")
                .expect("reserve affix space");
        } else {
            reserved
                .set_attribute("hidden", "")
                .expect("release affix space");
        }
    });

    container
}
