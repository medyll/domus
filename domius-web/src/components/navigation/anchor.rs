//! Anchor component for Domius.
//!
//! Smooth scroll navigation with anchor links.

use web_sys::Element;

/// Anchor link.
#[derive(Clone)]
pub struct AnchorLink {
    pub href: String,
    pub title: String,
}

/// Anchor props.
#[derive(Clone, Default)]
pub struct AnchorProps {
    /// Anchor links
    pub links: Vec<AnchorLink>,
    /// CSS class
    pub class: Option<String>,
    /// Offset from top (in px)
    pub offset_top: u32,
    /// Show boundary indicator line
    pub show_boundary: bool,
    /// Scroll container target
    pub target_container: Option<String>,
}

/// Build an Anchor component.
pub fn anchor(props: AnchorProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("anchor");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Boundary indicator line
    if props.show_boundary {
        let line: Element = document.create_element("div").unwrap();
        line.set_class_name("anchor-line");
        container.append_child(&line).unwrap();
    }

    // Links list
    let list: Element = document.create_element("ul").unwrap();
    list.set_class_name("anchor-list");

    for link in &props.links {
        let li: Element = document.create_element("li").unwrap();
        li.set_class_name("anchor-link-item");

        let a: Element = document.create_element("a").unwrap();
        a.set_class_name("anchor-link");
        a.set_attribute("href", &link.href).ok();
        a.set_text_content(Some(&link.title));

        li.append_child(&a).unwrap();
        list.append_child(&li).unwrap();
    }

    container.append_child(&list).unwrap();

    container
}
