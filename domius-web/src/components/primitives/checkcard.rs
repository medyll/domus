//! CheckCard component for Domius.
//!
//! A selectable card with checkbox functionality.

use web_sys::Element;

/// CheckCard props.
#[derive(Clone, Default)]
pub struct CheckCardProps {
    /// Card title
    pub title: Option<String>,
    /// Card description
    pub description: Option<String>,
    /// Card thumbnail/cover image
    pub cover: Option<String>,
    /// Card value
    pub value: String,
    /// CSS class
    pub class: Option<String>,
    /// Disabled state
    pub disabled: bool,
    /// Checked state
    pub checked: bool,
}

/// Build a CheckCard component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::checkcard::{checkcard, CheckCardProps};
///
/// let checkcard_node = checkcard(CheckCardProps {
///     title: Some("Option 1".to_string()),
///     value: "option1".to_string(),
///     checked: true,
///     ..Default::default()
/// });
/// ```
pub fn checkcard(props: CheckCardProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("checkcard");
    if props.checked {
        classes.push_str(" checkcard-checked");
    }
    if props.disabled {
        classes.push_str(" checkcard-disabled");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Store value as data attribute
    container.set_attribute("data-value", &props.value).ok();

    // Cover image
    if let Some(cover) = &props.cover {
        let cover_el: Element = document.create_element("img").unwrap();
        cover_el.set_class_name("checkcard-cover");
        cover_el.set_attribute("src", cover).ok();
        cover_el.set_attribute("alt", "cover").ok();
        container.append_child(&cover_el).unwrap();
    }

    // Content wrapper
    let content: Element = document.create_element("div").unwrap();
    content.set_class_name("checkcard-content");

    // Title
    if let Some(title) = &props.title {
        let title_el: Element = document.create_element("h4").unwrap();
        title_el.set_class_name("checkcard-title");
        title_el.set_text_content(Some(title));
        content.append_child(&title_el).unwrap();
    }

    // Description
    if let Some(description) = &props.description {
        let desc_el: Element = document.create_element("p").unwrap();
        desc_el.set_class_name("checkcard-description");
        desc_el.set_text_content(Some(description));
        content.append_child(&desc_el).unwrap();
    }

    container.append_child(&content).unwrap();

    // Checkbox indicator
    let checkbox: Element = document.create_element("div").unwrap();
    checkbox.set_class_name("checkcard-checkbox");
    if props.checked {
        checkbox.set_inner_html("&#10003;"); // ✓ symbol
    }
    container.append_child(&checkbox).unwrap();

    container
}
