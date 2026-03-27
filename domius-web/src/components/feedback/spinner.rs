//! Spinner component for Domius.
//!
//! Loading indicator with various styles.

use web_sys::Element;

/// Spinner size.
#[derive(Clone, Copy, Default)]
pub enum SpinnerSize {
    #[default]
    Small,
    Medium,
    Large,
}

/// Spinner type.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum SpinnerType {
    #[default]
    Circular,
    Dots,
    Bars,
}

/// Spinner props.
#[derive(Clone, Default)]
pub struct SpinnerProps {
    /// Spinner size
    pub size: SpinnerSize,
    /// Spinner type
    pub spinner_type: SpinnerType,
    /// CSS class
    pub class: Option<String>,
    /// Custom color
    pub color: Option<String>,
    /// Show with text label
    pub tip: Option<String>,
}

/// Build a Spinner component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::spinner::{spinner, SpinnerProps, SpinnerSize};
///
/// let spinner_node = spinner(SpinnerProps {
///     size: SpinnerSize::Large,
///     tip: Some("Loading...".to_string()),
///     ..Default::default()
/// });
/// ```
pub fn spinner(props: SpinnerProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let container: Element = document.create_element("div").unwrap();
    let mut classes = String::from("spinner");
    
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Size class
    let size_class = match props.size {
        SpinnerSize::Small => "spinner-sm",
        SpinnerSize::Medium => "spinner-md",
        SpinnerSize::Large => "spinner-lg",
    };
    container.set_attribute("data-size", size_class).ok();

    // Type class
    let type_class = match props.spinner_type {
        SpinnerType::Circular => "spinner-circular",
        SpinnerType::Dots => "spinner-dots",
        SpinnerType::Bars => "spinner-bars",
    };
    container.set_attribute("data-type", type_class).ok();

    // Color
    if let Some(color) = &props.color {
        container.set_attribute("style", &format!("--spinner-color: {};", color)).ok();
    }

    // Spinner SVG (circular)
    if props.spinner_type == SpinnerType::Circular {
        let svg_ns = "http://www.w3.org/2000/svg";
        let svg = document.create_element_ns(Some(svg_ns), "svg").unwrap();
        svg.set_attribute("viewBox", "0 0 50 50").ok();
        svg.set_attribute("class", "spinner-svg").ok();
        
        let circle = document.create_element_ns(Some(svg_ns), "circle").unwrap();
        circle.set_attribute("cx", "25").ok();
        circle.set_attribute("cy", "25").ok();
        circle.set_attribute("r", "20").ok();
        circle.set_attribute("fill", "none").ok();
        circle.set_attribute("stroke", "currentColor").ok();
        circle.set_attribute("stroke-width", "4").ok();
        circle.set_attribute("stroke-dasharray", "80 20").ok();
        circle.set_attribute("class", "spinner-circle").ok();
        
        svg.append_child(&circle).unwrap();
        container.append_child(&svg).unwrap();
    } else if props.spinner_type == SpinnerType::Dots {
        // Three dots
        for i in 0..3 {
            let dot: Element = document.create_element("span").unwrap();
            dot.set_class_name("spinner-dot");
            dot.set_attribute("data-index", &i.to_string()).ok();
            container.append_child(&dot).unwrap();
        }
    } else if props.spinner_type == SpinnerType::Bars {
        // Five bars
        for i in 0..5 {
            let bar: Element = document.create_element("span").unwrap();
            bar.set_class_name("spinner-bar");
            bar.set_attribute("data-index", &i.to_string()).ok();
            container.append_child(&bar).unwrap();
        }
    }

    // Tip text
    if let Some(tip) = &props.tip {
        let tip_el: Element = document.create_element("div").unwrap();
        tip_el.set_class_name("spinner-tip");
        tip_el.set_text_content(Some(tip));
        container.append_child(&tip_el).unwrap();
    }

    container
}
