//! Badge component - Status indicator or label.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

/// Badge visual variant.
#[derive(Clone, PartialEq, Debug)]
pub enum BadgeVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Info,
    Neutral,
}

impl Default for BadgeVariant {
    fn default() -> Self {
        Self::Primary
    }
}

impl BadgeVariant {
    pub fn as_class(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => "domius-badge-primary",
            BadgeVariant::Secondary => "domius-badge-secondary",
            BadgeVariant::Success => "domius-badge-success",
            BadgeVariant::Warning => "domius-badge-warning",
            BadgeVariant::Error => "domius-badge-error",
            BadgeVariant::Info => "domius-badge-info",
            BadgeVariant::Neutral => "domius-badge-neutral",
        }
    }
}

/// Badge size.
#[derive(Clone, PartialEq, Debug)]
pub enum BadgeSize {
    Sm,
    Md,
    Lg,
}

impl Default for BadgeSize {
    fn default() -> Self {
        Self::Md
    }
}

/// Props for the Badge component.
#[derive(Clone)]
pub struct BadgeProps {
    pub children: String,
    pub variant: BadgeVariant,
    pub size: BadgeSize,
    pub dot: bool,
    pub icon: Option<String>,
    pub class: Option<String>,
}

impl Default for BadgeProps {
    fn default() -> Self {
        Self {
            children: String::new(),
            variant: BadgeVariant::default(),
            size: BadgeSize::default(),
            dot: false,
            icon: None,
            class: None,
        }
    }
}

/// Badge component.
pub struct Badge;

impl Badge {
    /// Create a badge element.
    pub fn create(props: BadgeProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let badge: HtmlElement = document
            .create_element("span")
            .unwrap()
            .dyn_into()
            .unwrap();

        // Build class names
        let mut classes = vec![
            "domius-badge".to_string(),
            props.variant.as_class().to_string(),
            format!("domius-badge-{:?}", props.size).to_lowercase(),
        ];
        if props.dot {
            classes.push("domius-badge-dot".to_string());
        }
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }
        badge.set_attribute("class", &classes.join(" ")).unwrap();

        // Add dot indicator if requested
        if props.dot {
            let dot: HtmlElement = document
                .create_element("span")
                .unwrap()
                .dyn_into()
                .unwrap();
            dot.set_attribute("class", "domius-badge-dot-indicator").unwrap();
            badge.append_child(&dot).unwrap();
        }

        // Add icon if provided
        if let Some(icon) = &props.icon {
            let icon_el: HtmlElement = document
                .create_element("span")
                .unwrap()
                .dyn_into()
                .unwrap();
            icon_el.set_attribute("class", "domius-badge-icon").unwrap();
            icon_el.set_text_content(Some(icon));
            badge.append_child(&icon_el).unwrap();
        }

        // Add text content
        let text: HtmlElement = document
            .create_element("span")
            .unwrap()
            .dyn_into()
            .unwrap();
        text.set_attribute("class", "domius-badge-text").unwrap();
        text.set_text_content(Some(&props.children));
        badge.append_child(&text).unwrap();

        badge.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_variant_default() {
        assert_eq!(BadgeVariant::default(), BadgeVariant::Primary);
    }

    #[test]
    fn test_badge_size_default() {
        assert_eq!(BadgeSize::default(), BadgeSize::Md);
    }
}
