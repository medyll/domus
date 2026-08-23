//! Tag component for Domius.
//!
//! A small label with optional close button.

use web_sys::Element;

/// Tag color.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum TagColor {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

/// Tag props.
#[derive(Clone, Default)]
pub struct TagProps {
    /// Tag color
    pub color: TagColor,
    /// Tag text
    pub text: String,
    /// CSS class
    pub class: Option<String>,
    /// Show close button
    pub closable: bool,
}

/// Build a Tag component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::tag::{tag, TagProps, TagColor};
///
/// let tag_node = tag(TagProps {
///     color: TagColor::Primary,
///     text: "React".to_string(),
///     closable: true,
///     ..Default::default()
/// });
/// ```
pub fn tag(props: TagProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("span").unwrap();

    let mut classes = String::from("tag");
    match &props.color {
        TagColor::Default => classes.push_str(" tag-default"),
        TagColor::Primary => classes.push_str(" tag-primary"),
        TagColor::Success => classes.push_str(" tag-success"),
        TagColor::Warning => classes.push_str(" tag-warning"),
        TagColor::Error => classes.push_str(" tag-error"),
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Tag text
    let text_el: Element = document.create_element("span").unwrap();
    text_el.set_class_name("tag-text");
    text_el.set_text_content(Some(&props.text));
    container.append_child(&text_el).unwrap();

    // Close button
    if props.closable {
        let close_btn: Element = document.create_element("button").unwrap();
        close_btn.set_class_name("tag-close");
        close_btn.set_inner_html("&#215;"); // × symbol

        close_btn
            .set_attribute("onclick", "this.parentElement.remove()")
            .ok();

        container.append_child(&close_btn).unwrap();
    }

    container
}
