//! Skeleton component - Loading placeholder.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

/// Skeleton variant.
#[derive(Clone, PartialEq)]
pub enum SkeletonVariant {
    Text,
    Circular,
    Rectangular,
    Rounded,
}

impl Default for SkeletonVariant {
    fn default() -> Self {
        Self::Text
    }
}

/// Props for the Skeleton component.
#[derive(Clone)]
pub struct SkeletonProps {
    pub variant: SkeletonVariant,
    pub width: Option<String>,
    pub height: Option<String>,
    pub animation: SkeletonAnimation,
    pub lines: Option<usize>,
    pub class: Option<String>,
}

/// Skeleton animation type.
#[derive(Clone, PartialEq, Debug)]
pub enum SkeletonAnimation {
    Pulse,
    Wave,
    None,
}

impl Default for SkeletonAnimation {
    fn default() -> Self {
        Self::Pulse
    }
}

impl Default for SkeletonProps {
    fn default() -> Self {
        Self {
            variant: SkeletonVariant::default(),
            width: None,
            height: None,
            animation: SkeletonAnimation::default(),
            lines: None,
            class: None,
        }
    }
}

/// Skeleton component.
pub struct Skeleton;

impl Skeleton {
    /// Create a skeleton placeholder element.
    pub fn create(props: SkeletonProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        match props.variant {
            SkeletonVariant::Circular => {
                let skeleton: HtmlElement = document
                    .create_element("div")
                    .unwrap()
                    .dyn_into()
                    .unwrap();

                let mut classes = vec![
                    "domius-skeleton".to_string(),
                    "domius-skeleton-circular".to_string(),
                ];
                if props.animation != SkeletonAnimation::None {
                    classes.push(format!(
                        "domius-skeleton-{:?}",
                        props.animation
                    ).to_lowercase());
                }
                if let Some(class) = &props.class {
                    classes.push(class.clone());
                }
                skeleton.set_attribute("class", &classes.join(" ")).unwrap();

                // Set size
                let size = props.height.unwrap_or_else(|| "40px".to_string());
                skeleton.set_attribute("style", &format!("width: {}; height: {};", size, size)).unwrap();

                skeleton.into()
            }
            SkeletonVariant::Rectangular | SkeletonVariant::Rounded => {
                let skeleton: HtmlElement = document
                    .create_element("div")
                    .unwrap()
                    .dyn_into()
                    .unwrap();

                let mut classes = vec![
                    "domius-skeleton".to_string(),
                    if props.variant == SkeletonVariant::Rounded {
                        "domius-skeleton-rounded".to_string()
                    } else {
                        "domius-skeleton-rectangular".to_string()
                    },
                ];
                if props.animation != SkeletonAnimation::None {
                    classes.push(format!(
                        "domius-skeleton-{:?}",
                        props.animation
                    ).to_lowercase());
                }
                if let Some(class) = &props.class {
                    classes.push(class.clone());
                }
                skeleton.set_attribute("class", &classes.join(" ")).unwrap();

                // Build style
                let mut styles = Vec::new();
                if let Some(width) = &props.width {
                    styles.push(format!("width: {}", width));
                }
                if let Some(height) = &props.height {
                    styles.push(format!("height: {}", height));
                }
                if !styles.is_empty() {
                    skeleton.set_attribute("style", &styles.join("; ")).unwrap();
                }

                skeleton.into()
            }
            SkeletonVariant::Text => {
                let container: HtmlElement = document
                    .create_element("div")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                container.set_attribute("class", "domius-skeleton-text").unwrap();

                let lines = props.lines.unwrap_or(1);
                for i in 0..lines {
                    let line: HtmlElement = document
                        .create_element("div")
                        .unwrap()
                        .dyn_into()
                        .unwrap();

                    let mut classes = vec![
                        "domius-skeleton".to_string(),
                        "domius-skeleton-text-line".to_string(),
                    ];
                    if props.animation != SkeletonAnimation::None {
                        classes.push(format!(
                            "domius-skeleton-{:?}",
                            props.animation
                        ).to_lowercase());
                    }
                    line.set_attribute("class", &classes.join(" ")).unwrap();

                    // Last line shorter for natural look
                    if i == lines - 1 && lines > 1 {
                        line.set_attribute("style", "width: 60%").unwrap();
                    }

                    container.append_child(&line).unwrap();
                }

                container.into()
            }
        }
    }
}
