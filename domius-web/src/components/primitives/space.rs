//! Space component for Domius.
//!
//! Flexible spacing between child elements.

use web_sys::Element;

/// Space direction.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum SpaceDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Space align.
#[derive(Clone, Copy, Default)]
pub enum SpaceAlign {
    Start,
    #[default]
    Center,
    End,
    Baseline,
}

/// Space justify.
#[derive(Clone, Copy, Default)]
pub enum SpaceJustify {
    Start,
    Center,
    #[default]
    Between,
    Around,
    End,
}

/// Space props.
#[derive(Clone, Default)]
pub struct SpaceProps {
    /// Direction of spacing
    pub direction: SpaceDirection,
    /// Space size in px
    pub size: u32,
    /// CSS class
    pub class: Option<String>,
    /// Wrap children
    pub wrap: bool,
    /// Align items
    pub align: Option<SpaceAlign>,
    /// Justify content
    pub justify: Option<SpaceJustify>,
}

/// Build a Space component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::space::{space, SpaceProps, SpaceDirection};
///
/// let space_node = space(SpaceProps {
///     direction: SpaceDirection::Horizontal,
///     size: 16,
///     ..Default::default()
/// });
/// ```
pub fn space(props: SpaceProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("space");
    if props.direction == SpaceDirection::Vertical {
        classes.push_str(" space-vertical");
    }
    if props.wrap {
        classes.push_str(" space-wrap");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Inline styles for spacing
    let gap = format!("{}px", props.size);
    let mut style = format!("gap: {};", gap);

    if let Some(align) = props.align {
        let align_val = match align {
            SpaceAlign::Start => "flex-start",
            SpaceAlign::Center => "center",
            SpaceAlign::End => "flex-end",
            SpaceAlign::Baseline => "baseline",
        };
        style.push_str(&format!(" align-items: {};", align_val));
    }

    if let Some(justify) = props.justify {
        let justify_val = match justify {
            SpaceJustify::Start => "flex-start",
            SpaceJustify::Center => "center",
            SpaceJustify::Between => "space-between",
            SpaceJustify::Around => "space-around",
            SpaceJustify::End => "flex-end",
        };
        style.push_str(&format!(" justify-content: {};", justify_val));
    }

    container.set_attribute("style", &style).ok();

    container
}

/// Space item wrapper (for consistent spacing).
pub fn space_item(content: &str) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let item: Element = document.create_element("div").unwrap();
    item.set_class_name("space-item");
    item.set_text_content(Some(content));
    item
}
