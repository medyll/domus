//! Card component for Domius.
//!
//! A flexible container with header, body, and optional footer.

use web_sys::Element;

/// Card props.
#[derive(Clone, Default)]
pub struct CardProps {
    /// Card title (shown in header)
    pub title: Option<String>,
    /// Extra content for header (right side)
    pub extra: Option<String>,
    /// CSS class
    pub class: Option<String>,
    /// Show bordered style
    pub bordered: bool,
    /// Show hover effect
    pub hoverable: bool,
    /// Custom CSS style
    pub style: Option<String>,
}

/// Build a Card component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::card::{card, CardProps};
///
/// let card_node = card(CardProps {
///     title: Some("Card Title".to_string()),
///     bordered: true,
///     hoverable: false,
///     ..Default::default()
/// });
/// ```
pub fn card(props: CardProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    
    // Main container
    let container: Element = document.create_element("div").unwrap();
    
    let mut classes = String::from("card");
    if props.bordered {
        classes.push_str(" card-bordered");
    }
    if props.hoverable {
        classes.push_str(" card-hoverable");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);
    
    if let Some(style) = &props.style {
        container.set_attribute("style", style).ok();
    }

    // Header (if title or extra provided)
    if props.title.is_some() || props.extra.is_some() {
        let header: Element = document.create_element("div").unwrap();
        header.set_class_name("card-header");

        if let Some(title) = &props.title {
            let title_el: Element = document.create_element("h3").unwrap();
            title_el.set_class_name("card-title");
            title_el.set_text_content(Some(title));
            header.append_child(&title_el).unwrap();
        }

        if let Some(extra) = &props.extra {
            let extra_el: Element = document.create_element("span").unwrap();
            extra_el.set_class_name("card-extra");
            extra_el.set_text_content(Some(extra));
            header.append_child(&extra_el).unwrap();
        }

        container.append_child(&header).unwrap();
    }

    // Body
    let body: Element = document.create_element("div").unwrap();
    body.set_class_name("card-body");
    container.append_child(&body).unwrap();

    container
}

/// Build card body content (helper to append children to card body).
pub fn card_body(card: &Element, content: &str) {
    if let Some(body) = card.query_selector(".card-body").ok().flatten() {
        body.set_text_content(Some(content));
    }
}
