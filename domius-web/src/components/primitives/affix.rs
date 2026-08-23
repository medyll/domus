//! Affix component for Domius.
//!
//! Fix an element to a specific viewport position on scroll.

use web_sys::Element;

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
/// # Example
///
/// ```ignore
/// use domius_web::components::affix::{affix, AffixProps};
///
/// let affix_node = affix(AffixProps {
///     offset_top: 64,
///     ..Default::default()
/// });
/// ```
pub fn affix(props: AffixProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("affix");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Store offset as data attribute for CSS/JS handling
    container
        .set_attribute("data-offset-top", &props.offset_top.to_string())
        .ok();

    if let Some(offset_bottom) = props.offset_bottom {
        container
            .set_attribute("data-offset-bottom", &offset_bottom.to_string())
            .ok();
    }

    if let Some(target) = &props.target {
        container.set_attribute("data-target", target).ok();
    }

    // Placeholder to maintain layout
    let placeholder: Element = document.create_element("div").unwrap();
    placeholder.set_class_name("affix-placeholder");
    placeholder.set_attribute("hidden", "").ok();
    container.append_child(&placeholder).unwrap();

    container
}
