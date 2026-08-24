//! Watermark component - Security overlay with text/logo.

use std::cell::Cell;

use web_sys::{Document, Element};

/// Props for the Watermark component.
#[derive(Clone)]
pub struct WatermarkProps {
    pub text: Option<String>,
    pub image: Option<String>,
    pub opacity: f32,
    pub rotation: f32,
    pub gap: (u32, u32),
    pub offset: (u32, u32),
    pub font_size: u32,
    pub font_color: String,
    pub class: Option<String>,
}

impl Default for WatermarkProps {
    fn default() -> Self {
        Self {
            text: Some("CONFIDENTIAL".to_string()),
            image: None,
            opacity: 0.1,
            rotation: -30.0,
            gap: (100, 100),
            offset: (50, 50),
            font_size: 16,
            font_color: "#000000".to_string(),
            class: None,
        }
    }
}

thread_local! {
    /// Pattern ids must stay unique so several watermarks can share one page.
    static PATTERN_SEQUENCE: Cell<u32> = const { Cell::new(0) };
}

fn next_pattern_id() -> String {
    let sequence = PATTERN_SEQUENCE.with(|counter| {
        let next = counter.get().wrapping_add(1);
        counter.set(next);
        next
    });
    format!("domius-watermark-{sequence}")
}

/// Watermark component.
pub struct Watermark;

impl Watermark {
    /// Create a watermark overlay element.
    ///
    /// The overlay tiles the mark across its own box with an SVG pattern; the
    /// stylesheet owns how the box is stretched over the content it protects.
    pub fn create(props: WatermarkProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let overlay = document
            .create_element("div")
            .expect("create watermark overlay");
        let mut classes = vec!["domius-watermark"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        overlay.set_class_name(&classes.join(" "));
        overlay
            .set_attribute("data-role", "watermark")
            .expect("mark watermark overlay");

        let mark = Mark::from(&props);
        overlay
            .set_attribute("data-watermark", mark.description())
            .expect("describe watermark");
        let opacity = f64::from(props.opacity).clamp(0.0, 1.0);
        overlay
            .set_attribute("data-opacity", &format_number(opacity))
            .expect("set watermark opacity");
        overlay
            .set_attribute("data-rotation", &format_number(f64::from(props.rotation)))
            .expect("set watermark rotation");

        let (gap_x, gap_y) = (props.gap.0.max(1), props.gap.1.max(1));
        let pattern_id = next_pattern_id();
        let svg = svg_element(&document, "svg");
        svg.set_attribute("data-role", "watermark-layer")
            .expect("mark watermark layer");
        svg.set_attribute("width", "100%")
            .expect("stretch watermark layer");
        svg.set_attribute("height", "100%")
            .expect("stretch watermark layer");
        svg.set_attribute("role", "img")
            .expect("set watermark role");
        svg.set_attribute("aria-label", &format!("Watermark: {}", mark.description()))
            .expect("label watermark");

        let defs = svg_element(&document, "defs");
        let pattern = svg_element(&document, "pattern");
        pattern
            .set_attribute("id", &pattern_id)
            .expect("identify watermark pattern");
        pattern
            .set_attribute("patternUnits", "userSpaceOnUse")
            .expect("set watermark pattern units");
        pattern
            .set_attribute("width", &gap_x.to_string())
            .expect("set watermark tile width");
        pattern
            .set_attribute("height", &gap_y.to_string())
            .expect("set watermark tile height");
        pattern
            .set_attribute(
                "patternTransform",
                &format!("rotate({})", format_number(f64::from(props.rotation))),
            )
            .expect("rotate watermark pattern");
        pattern
            .append_child(&mark.render(&document, &props, opacity))
            .expect("append watermark mark");
        defs.append_child(&pattern)
            .expect("append watermark pattern");
        svg.append_child(&defs).expect("append watermark defs");

        let fill = svg_element(&document, "rect");
        fill.set_attribute("width", "100%")
            .expect("stretch watermark fill");
        fill.set_attribute("height", "100%")
            .expect("stretch watermark fill");
        fill.set_attribute("fill", &format!("url(#{pattern_id})"))
            .expect("apply watermark pattern");
        svg.append_child(&fill).expect("append watermark fill");
        overlay.append_child(&svg).expect("append watermark layer");
        overlay
    }
}

/// What the overlay repeats: a caption or a logo.
enum Mark<'a> {
    Text(&'a str),
    Image(&'a str),
}

impl<'a> Mark<'a> {
    /// Prefer text when both are supplied so the mark stays readable without assets.
    fn from(props: &'a WatermarkProps) -> Self {
        match (props.text.as_deref(), props.image.as_deref()) {
            (Some(text), _) if !text.is_empty() => Self::Text(text),
            (_, Some(image)) if !image.is_empty() => Self::Image(image),
            _ => Self::Text("CONFIDENTIAL"),
        }
    }

    fn description(&self) -> &str {
        match self {
            Self::Text(text) => text,
            Self::Image(image) => image,
        }
    }

    fn render(&self, document: &Document, props: &WatermarkProps, opacity: f64) -> Element {
        let (offset_x, offset_y) = (props.offset.0, props.offset.1);
        match self {
            Self::Text(text) => {
                let element = svg_element(document, "text");
                element
                    .set_attribute("x", &offset_x.to_string())
                    .expect("position watermark text");
                element
                    .set_attribute("y", &offset_y.to_string())
                    .expect("position watermark text");
                element
                    .set_attribute("font-size", &props.font_size.max(1).to_string())
                    .expect("size watermark text");
                element
                    .set_attribute("fill", &props.font_color)
                    .expect("colour watermark text");
                element
                    .set_attribute("opacity", &format_number(opacity))
                    .expect("fade watermark text");
                element
                    .set_attribute("text-anchor", "middle")
                    .expect("centre watermark text");
                element.set_text_content(Some(text));
                element
            }
            Self::Image(image) => {
                let element = svg_element(document, "image");
                element
                    .set_attribute("href", image)
                    .expect("source watermark image");
                element
                    .set_attribute("x", &offset_x.to_string())
                    .expect("position watermark image");
                element
                    .set_attribute("y", &offset_y.to_string())
                    .expect("position watermark image");
                element
                    .set_attribute("width", &props.gap.0.max(1).to_string())
                    .expect("size watermark image");
                element
                    .set_attribute("height", &props.gap.1.max(1).to_string())
                    .expect("size watermark image");
                element
                    .set_attribute("opacity", &format_number(opacity))
                    .expect("fade watermark image");
                element
            }
        }
    }
}

fn svg_element(document: &Document, tag: &str) -> Element {
    document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
        .expect("create SVG element")
}

fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "0".to_string();
    }
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}
