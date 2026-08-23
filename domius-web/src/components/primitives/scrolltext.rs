//! ScrollText component for Domius.
//!
//! Auto-scrolling text display.

use web_sys::Element;

/// ScrollText direction.
#[derive(Clone, Copy, Default)]
pub enum ScrollTextDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

/// ScrollText props.
#[derive(Clone, Default)]
pub struct ScrollTextProps {
    /// Text content (array of lines)
    pub lines: Vec<String>,
    /// CSS class
    pub class: Option<String>,
    /// Scroll direction
    pub direction: ScrollTextDirection,
    /// Auto-scroll speed (ms per scroll)
    pub speed: u32,
    /// Enable auto-scroll
    pub auto_scroll: bool,
    /// Pause on hover
    pub pause_on_hover: bool,
    /// Scroll one line at a time
    pub one_by_one: bool,
}

/// Build a ScrollText component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::scrolltext::{scrolltext, ScrollTextProps, ScrollTextDirection};
///
/// let scrolltext_node = scrolltext(ScrollTextProps {
///     lines: vec!["Line 1".to_string(), "Line 2".to_string()],
///     direction: ScrollTextDirection::Up,
///     auto_scroll: true,
///     speed: 3000,
///     ..Default::default()
/// });
/// ```
pub fn scrolltext(props: ScrollTextProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("scrolltext");
    match props.direction {
        ScrollTextDirection::Up => classes.push_str(" scrolltext-up"),
        ScrollTextDirection::Down => classes.push_str(" scrolltext-down"),
        ScrollTextDirection::Left => classes.push_str(" scrolltext-left"),
        ScrollTextDirection::Right => classes.push_str(" scrolltext-right"),
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Store scroll settings as data attributes
    container
        .set_attribute("data-speed", &props.speed.to_string())
        .ok();
    container
        .set_attribute("data-auto-scroll", &props.auto_scroll.to_string())
        .ok();
    container
        .set_attribute("data-pause-on-hover", &props.pause_on_hover.to_string())
        .ok();
    container
        .set_attribute("data-one-by-one", &props.one_by_one.to_string())
        .ok();

    // Viewport (overflow hidden)
    let viewport: Element = document.create_element("div").unwrap();
    viewport.set_class_name("scrolltext-viewport");
    viewport.set_attribute("style", "overflow: hidden;").ok();

    // Content wrapper
    let content: Element = document.create_element("div").unwrap();
    content.set_class_name("scrolltext-content");

    // Add lines
    for line in &props.lines {
        let line_el: Element = document.create_element("div").unwrap();
        line_el.set_class_name("scrolltext-line");
        line_el.set_text_content(Some(line));
        content.append_child(&line_el).unwrap();
    }

    viewport.append_child(&content).unwrap();
    container.append_child(&viewport).unwrap();

    container
}
