//! BackTop component for Domius.
//!
//! Button to scroll back to top.

use web_sys::Element;

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
/// # Example
///
/// ```ignore
/// use domius_web::components::backtop::{backtop, BackTopProps};
///
/// let backtop_node = backtop(BackTopProps {
///     visibility_height: 400,
///     ..Default::default()
/// });
/// ```
pub fn backtop(props: BackTopProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let container: Element = document.create_element("div").unwrap();
    
    let mut classes = String::from("backtop");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Store visibility height as data attribute
    container.set_attribute("data-visibility-height", &props.visibility_height.to_string()).ok();
    
    if let Some(target) = &props.target {
        container.set_attribute("data-target", target).ok();
    }

    // Button content
    let button: Element = document.create_element("button").unwrap();
    button.set_class_name("backtop-button");
    
    if let Some(content) = &props.content {
        button.set_text_content(Some(content));
    } else {
        // Default up arrow icon
        button.set_inner_html("&#8593;"); // ↑ symbol
    }
    
    container.append_child(&button).unwrap();

    // Initially hidden (shown via JS on scroll)
    container.set_attribute("hidden", "").ok();

    container
}
