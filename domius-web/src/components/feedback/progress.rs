//! ProgressBar component - how far along something is.

use domius_core::signal::{signal, Signal};
use web_sys::{Document, Element};

use crate::disposal::ViewScope;

/// Progress bar variant.
#[derive(Clone, PartialEq, Debug)]
pub enum ProgressVariant {
    Linear,
    Circular,
}

impl Default for ProgressVariant {
    fn default() -> Self {
        Self::Linear
    }
}

/// Props for the ProgressBar component.
#[derive(Clone)]
pub struct ProgressProps {
    pub value: Signal<u8>,
    pub max: u8,
    pub variant: ProgressVariant,
    pub size: ProgressSize,
    pub color: Option<String>,
    pub show_label: bool,
    pub label_format: Option<String>,
    pub indeterminate: bool,
    pub class: Option<String>,
}

/// Progress bar size.
#[derive(Clone, PartialEq, Debug)]
pub enum ProgressSize {
    Sm,
    Md,
    Lg,
}

impl ProgressSize {
    fn token(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

impl Default for ProgressSize {
    fn default() -> Self {
        Self::Md
    }
}

impl Default for ProgressProps {
    fn default() -> Self {
        Self {
            value: signal(0),
            max: 100,
            variant: ProgressVariant::default(),
            size: ProgressSize::default(),
            color: None,
            show_label: false,
            label_format: None,
            indeterminate: false,
            class: None,
        }
    }
}

/// ProgressBar component.
pub struct ProgressBar;

impl ProgressBar {
    /// Create a progress bar that follows its value signal.
    ///
    /// The linear variant is a native `<progress>`, so it reports itself to
    /// assistive technology and draws without a stylesheet. The circular one is
    /// an SVG arc whose length is an attribute rather than inline geometry.
    pub fn create(props: ProgressProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let container = document
            .create_element("div")
            .expect("create progress container");
        let mut classes = vec![
            "domius-progress".to_string(),
            format!("domius-progress-{}", variant_token(&props.variant)),
            format!("domius-progress-{}", props.size.token()),
        ];
        if props.indeterminate {
            classes.push("domius-progress-indeterminate".to_string());
        }
        if let Some(class) = props.class.as_deref() {
            classes.push(class.to_string());
        }
        container.set_class_name(&classes.join(" "));
        container
            .set_attribute("data-variant", variant_token(&props.variant))
            .expect("expose progress variant");
        if let Some(color) = props.color.as_deref() {
            // A token, not a colour: the sheet owns the palette.
            container
                .set_attribute("data-color", color)
                .expect("expose progress colour");
        }

        let max = props.max.max(1);
        let indicator = build_indicator(&document, &props, max);
        container
            .append_child(&indicator)
            .expect("append progress indicator");

        let label = props.show_label.then(|| {
            let label = document
                .create_element("span")
                .expect("create progress label");
            label.set_class_name("domius-progress-label");
            container
                .append_child(&label)
                .expect("append progress label");
            label
        });

        // An indeterminate bar has no value to follow.
        if props.indeterminate {
            return container;
        }

        let scope = ViewScope::attach(&container);
        let value = props.value.clone();
        let format = props.label_format.clone();
        let variant = props.variant.clone();
        let host = container.clone();
        scope.effect(move || {
            let current = value.get().min(max);
            let percentage = f64::from(current) / f64::from(max) * 100.0;
            host.set_attribute("data-percentage", &format!("{percentage:.0}"))
                .expect("expose progress percentage");
            update_indicator(&indicator, &variant, current, max, percentage);
            if let Some(label) = &label {
                label.set_text_content(Some(&match &format {
                    Some(format) => format
                        .replace("{value}", &current.to_string())
                        .replace("{max}", &max.to_string()),
                    None => format!("{percentage:.0}%"),
                }));
            }
        });

        container
    }
}

fn build_indicator(document: &Document, props: &ProgressProps, max: u8) -> Element {
    match props.variant {
        ProgressVariant::Linear => {
            let bar = document
                .create_element("progress")
                .expect("create progress bar");
            bar.set_class_name("domius-progress-bar");
            bar.set_attribute("max", &max.to_string())
                .expect("set progress maximum");
            if props.indeterminate {
                // A `<progress>` with no value is the indeterminate one.
                bar.remove_attribute("value").ok();
            }
            bar
        }
        ProgressVariant::Circular => {
            let svg = svg_element(document, "svg");
            svg.set_attribute("viewBox", "0 0 40 40")
                .expect("set progress view box");
            svg.set_attribute("role", "progressbar")
                .expect("set progress role");
            svg.set_attribute("aria-valuemin", "0")
                .expect("set progress minimum");
            svg.set_attribute("aria-valuemax", &max.to_string())
                .expect("set progress maximum");
            let track = svg_element(document, "circle");
            set_arc(&track, "domius-progress-track");
            svg.append_child(&track).expect("append progress track");
            let arc = svg_element(document, "circle");
            set_arc(&arc, "domius-progress-bar");
            svg.append_child(&arc).expect("append progress arc");
            svg
        }
    }
}

fn update_indicator(
    indicator: &Element,
    variant: &ProgressVariant,
    current: u8,
    max: u8,
    percentage: f64,
) {
    match variant {
        ProgressVariant::Linear => {
            indicator
                .set_attribute("value", &current.to_string())
                .expect("set progress value");
        }
        ProgressVariant::Circular => {
            indicator
                .set_attribute("aria-valuenow", &current.to_string())
                .expect("set progress value");
            indicator
                .set_attribute("aria-valuetext", &format!("{percentage:.0}%"))
                .expect("describe progress value");
            let _ = max;
            if let Some(arc) = indicator
                .query_selector(".domius-progress-bar")
                .expect("query progress arc")
            {
                let circumference = std::f64::consts::TAU * ARC_RADIUS;
                let filled = circumference * percentage / 100.0;
                arc.set_attribute(
                    "stroke-dasharray",
                    &format!("{filled:.2} {:.2}", circumference - filled),
                )
                .expect("draw progress arc");
            }
        }
    }
}

/// Radius of the circular variant, in its own view box units.
const ARC_RADIUS: f64 = 16.0;

fn set_arc(circle: &Element, class: &str) {
    // An SVG element's className is read-only, so set the attribute directly.
    circle
        .set_attribute("class", class)
        .expect("class progress arc");
    circle
        .set_attribute("cx", "20")
        .expect("centre progress arc");
    circle
        .set_attribute("cy", "20")
        .expect("centre progress arc");
    circle
        .set_attribute("r", &ARC_RADIUS.to_string())
        .expect("size progress arc");
    circle
        .set_attribute("fill", "none")
        .expect("hollow progress arc");
    // Start the arc at twelve o'clock rather than three.
    circle
        .set_attribute("transform", "rotate(-90 20 20)")
        .expect("orient progress arc");
}

fn variant_token(variant: &ProgressVariant) -> &'static str {
    match variant {
        ProgressVariant::Linear => "linear",
        ProgressVariant::Circular => "circular",
    }
}

fn svg_element(document: &Document, tag: &str) -> Element {
    document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
        .expect("create SVG element")
}
