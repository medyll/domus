//! ProgressBar component - Progress indicator.

use domius_core::signal::{signal, Signal};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

/// Progress bar variant.
#[derive(Clone, PartialEq)]
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
    /// Create a progress bar element.
    pub fn create(props: ProgressProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        match props.variant {
            ProgressVariant::Linear => {
                let container: HtmlElement = document
                    .create_element("div")
                    .unwrap()
                    .dyn_into()
                    .unwrap();

                let mut classes = vec![
                    "domius-progress".to_string(),
                    "domius-progress-linear".to_string(),
                    format!("domius-progress-{:?}", props.size).to_lowercase(),
                ];
                if props.indeterminate {
                    classes.push("domius-progress-indeterminate".to_string());
                }
                if let Some(class) = &props.class {
                    classes.push(class.clone());
                }
                container.set_attribute("class", &classes.join(" ")).unwrap();
                container.set_attribute("role", "progressbar").unwrap();
                container.set_attribute("aria-valuenow", &props.value.get().to_string()).unwrap();
                container.set_attribute("aria-valuemin", "0").unwrap();
                container.set_attribute("aria-valuemax", &props.max.to_string()).unwrap();

                // Bar
                let bar: HtmlElement = document
                    .create_element("div")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                bar.set_attribute("class", "domius-progress-bar").unwrap();

                if let Some(color) = &props.color {
                    bar.set_attribute("style", &format!("background-color: {}", color)).unwrap();
                }

                if !props.indeterminate {
                    let percentage = (props.value.get() as f64 / props.max as f64 * 100.0).min(100.0);
                    bar.set_attribute("style", &format!("width: {}%", percentage)).unwrap();
                }

                container.append_child(&bar).unwrap();

                // Label
                if props.show_label {
                    let label: HtmlElement = document
                        .create_element("span")
                        .unwrap()
                        .dyn_into()
                        .unwrap();
                    label.set_attribute("class", "domius-progress-label").unwrap();

                    let label_text = if let Some(format) = &props.label_format {
                        format.replace("{value}", &props.value.get().to_string())
                            .replace("{max}", &props.max.to_string())
                    } else {
                        format!("{}%", (props.value.get() as f64 / props.max as f64 * 100.0) as u8)
                    };
                    label.set_text_content(Some(&label_text));
                    container.append_child(&label).unwrap();
                }

                container.into()
            }
            ProgressVariant::Circular => {
                let container: HtmlElement = document
                    .create_element("div")
                    .unwrap()
                    .dyn_into()
                    .unwrap();

                let mut classes = vec![
                    "domius-progress".to_string(),
                    "domius-progress-circular".to_string(),
                    format!("domius-progress-{:?}", props.size).to_lowercase(),
                ];
                if let Some(class) = &props.class {
                    classes.push(class.clone());
                }
                container.set_attribute("class", &classes.join(" ")).unwrap();

                // SVG circle implementation would go here
                // For now, simplified version
                container.set_text_content(Some(&format!("{}%", props.value.get())));

                container.into()
            }
        }
    }
}
