//! Divider component for Domius.
//!
//! A visual separator line.

use web_sys::Element;

/// Divider orientation.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Divider props.
#[derive(Clone, Default)]
pub struct DividerProps {
    /// Orientation
    pub orientation: DividerOrientation,
    /// Text label (centered)
    pub text: Option<String>,
    /// CSS class
    pub class: Option<String>,
    /// Dashed style
    pub dashed: bool,
    /// Plain style (no lines on sides)
    pub plain: bool,
}

/// Build a Divider component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::divider::{divider, DividerProps, DividerOrientation};
///
/// let divider_node = divider(DividerProps {
///     orientation: DividerOrientation::Horizontal,
///     text: Some("OR".to_string()),
///     ..Default::default()
/// });
/// ```
pub fn divider(props: DividerProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let divider: Element = document.create_element("div").unwrap();

    let mut classes = String::from("divider");
    if props.orientation == DividerOrientation::Vertical {
        classes.push_str(" divider-vertical");
    }
    if props.dashed {
        classes.push_str(" divider-dashed");
    }
    if props.text.is_some() {
        classes.push_str(" divider-with-text");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    divider.set_class_name(&classes);

    // Left line (if text provided and not plain)
    if props.text.is_some() && !props.plain {
        let left_line: Element = document.create_element("div").unwrap();
        left_line.set_class_name("divider-line divider-line-left");
        divider.append_child(&left_line).unwrap();
    }

    // Text content
    if let Some(text) = &props.text {
        let text_el: Element = document.create_element("span").unwrap();
        text_el.set_class_name("divider-text");
        text_el.set_text_content(Some(text));
        divider.append_child(&text_el).unwrap();
    }

    // Right line (if text provided and not plain)
    if props.text.is_some() && !props.plain {
        let right_line: Element = document.create_element("div").unwrap();
        right_line.set_class_name("divider-line divider-line-right");
        divider.append_child(&right_line).unwrap();
    }

    divider
}
