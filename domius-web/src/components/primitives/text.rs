//! Text component - Typography component for consistent text styling.

use domius_core::signal::Signal;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::component::DomiusNode;

/// Text visual variant.
#[derive(Clone, PartialEq)]
pub enum TextVariant {
    /// Body text (default)
    Body,
    /// Small text for captions, hints
    Caption,
    /// Overline text (uppercase, small)
    Overline,
    /// Subtitle text
    Subtitle1,
    Subtitle2,
    /// Heading text
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl Default for TextVariant {
    fn default() -> Self {
        Self::Body
    }
}

impl TextVariant {
    pub fn as_tag(&self) -> &'static str {
        match self {
            TextVariant::Body | TextVariant::Caption | TextVariant::Overline => "span",
            TextVariant::Subtitle1 | TextVariant::Subtitle2 => "h2",
            TextVariant::H1 => "h1",
            TextVariant::H2 => "h2",
            TextVariant::H3 => "h3",
            TextVariant::H4 => "h4",
            TextVariant::H5 => "h5",
            TextVariant::H6 => "h6",
        }
    }

    pub fn as_class(&self) -> &'static str {
        match self {
            TextVariant::Body => "domius-text-body",
            TextVariant::Caption => "domius-text-caption",
            TextVariant::Overline => "domius-text-overline",
            TextVariant::Subtitle1 => "domius-text-subtitle1",
            TextVariant::Subtitle2 => "domius-text-subtitle2",
            TextVariant::H1 => "domius-text-h1",
            TextVariant::H2 => "domius-text-h2",
            TextVariant::H3 => "domius-text-h3",
            TextVariant::H4 => "domius-text-h4",
            TextVariant::H5 => "domius-text-h5",
            TextVariant::H6 => "domius-text-h6",
        }
    }
}

/// Text color.
#[derive(Clone, PartialEq)]
pub enum TextColor {
    Primary,
    Secondary,
    Error,
    Warning,
    Success,
    Info,
    Inherit,
}

impl Default for TextColor {
    fn default() -> Self {
        Self::Primary
    }
}

impl TextColor {
    pub fn as_class(&self) -> &'static str {
        match self {
            TextColor::Primary => "domius-text-primary",
            TextColor::Secondary => "domius-text-secondary",
            TextColor::Error => "domius-text-error",
            TextColor::Warning => "domius-text-warning",
            TextColor::Success => "domius-text-success",
            TextColor::Info => "domius-text-info",
            TextColor::Inherit => "domius-text-inherit",
        }
    }
}

/// Props for the Text component.
#[derive(Clone)]
pub struct TextProps {
    /// Text content
    pub children: String,
    /// Visual variant
    pub variant: TextVariant,
    /// Text color
    pub color: TextColor,
    /// Whether text is bold
    pub bold: bool,
    /// Whether text is italic
    pub italic: bool,
    /// Whether text is uppercase
    pub uppercase: bool,
    /// Whether text is lowercase
    pub lowercase: bool,
    /// Whether text is capitalized (first letter uppercase)
    pub capitalize: bool,
    /// Text alignment
    pub align: TextAlignment,
    /// Whether to truncate text with ellipsis
    pub truncate: bool,
    /// Maximum lines before truncation (requires line-clamp support)
    pub max_lines: Option<u32>,
    /// Additional CSS classes
    pub class: Option<String>,
    /// Element ID
    pub id: Option<String>,
}

/// Text alignment.
#[derive(Clone, PartialEq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

impl Default for TextAlignment {
    fn default() -> Self {
        Self::Left
    }
}

impl TextAlignment {
    pub fn as_class(&self) -> &'static str {
        match self {
            TextAlignment::Left => "domius-text-left",
            TextAlignment::Center => "domius-text-center",
            TextAlignment::Right => "domius-text-right",
            TextAlignment::Justify => "domius-text-justify",
        }
    }
}

impl Default for TextProps {
    fn default() -> Self {
        Self {
            children: String::new(),
            variant: TextVariant::default(),
            color: TextColor::default(),
            bold: false,
            italic: false,
            uppercase: false,
            lowercase: false,
            capitalize: false,
            align: TextAlignment::default(),
            truncate: false,
            max_lines: None,
            class: None,
            id: None,
        }
    }
}

/// Text component.
pub struct Text;

impl Text {
    /// Create a text element with the given properties.
    pub fn create(props: TextProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let tag = props.variant.as_tag();
        let element: HtmlElement = document
            .create_element(tag)
            .unwrap()
            .dyn_into()
            .unwrap();

        // Build class names
        let mut classes = vec![
            "domius-text".to_string(),
            props.variant.as_class().to_string(),
            props.color.as_class().to_string(),
            props.align.as_class().to_string(),
        ];

        if props.bold {
            classes.push("domius-text-bold".to_string());
        }
        if props.italic {
            classes.push("domius-text-italic".to_string());
        }
        if props.uppercase {
            classes.push("domius-text-uppercase".to_string());
        }
        if props.lowercase {
            classes.push("domius-text-lowercase".to_string());
        }
        if props.capitalize {
            classes.push("domius-text-capitalize".to_string());
        }
        if props.truncate {
            classes.push("domius-text-truncate".to_string());
        }
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }

        element.set_attribute("class", &classes.join(" ")).unwrap();

        if let Some(id) = &props.id {
            element.set_id(id);
        }

        // Set text content
        let text_content = if props.uppercase {
            props.children.to_uppercase()
        } else if props.lowercase {
            props.children.to_lowercase()
        } else if props.capitalize {
            let mut chars = props.children.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        } else {
            props.children
        };

        element.set_text_content(Some(&text_content));

        // Add max-lines style if specified
        if let Some(max_lines) = props.max_lines {
            element
                .set_attribute(
                    "style",
                    &format!(
                        "display: -webkit-box; -webkit-line-clamp: {}; -webkit-box-orient: vertical; overflow: hidden;",
                        max_lines
                    ),
                )
                .unwrap();
        }

        element.into()
    }

    /// Create a paragraph text.
    pub fn body(text: impl Into<String>) -> Element {
        Self::create(TextProps {
            children: text.into(),
            variant: TextVariant::Body,
            ..Default::default()
        })
    }

    /// Create a heading level 1.
    pub fn h1(text: impl Into<String>) -> Element {
        Self::create(TextProps {
            children: text.into(),
            variant: TextVariant::H1,
            ..Default::default()
        })
    }

    /// Create a heading level 2.
    pub fn h2(text: impl Into<String>) -> Element {
        Self::create(TextProps {
            children: text.into(),
            variant: TextVariant::H2,
            ..Default::default()
        })
    }

    /// Create a heading level 3.
    pub fn h3(text: impl Into<String>) -> Element {
        Self::create(TextProps {
            children: text.into(),
            variant: TextVariant::H3,
            ..Default::default()
        })
    }

    /// Create a caption text.
    pub fn caption(text: impl Into<String>) -> Element {
        Self::create(TextProps {
            children: text.into(),
            variant: TextVariant::Caption,
            color: TextColor::Secondary,
            ..Default::default()
        })
    }

    /// Create an error text.
    pub fn error(text: impl Into<String>) -> Element {
        Self::create(TextProps {
            children: text.into(),
            variant: TextVariant::Caption,
            color: TextColor::Error,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_variant_default() {
        assert_eq!(TextVariant::default(), TextVariant::Body);
    }

    #[test]
    fn test_text_variant_as_tag() {
        assert_eq!(TextVariant::Body.as_tag(), "span");
        assert_eq!(TextVariant::H1.as_tag(), "h1");
        assert_eq!(TextVariant::H2.as_tag(), "h2");
    }

    #[test]
    fn test_text_color_default() {
        assert_eq!(TextColor::default(), TextColor::Primary);
    }

    #[test]
    fn test_text_alignment_default() {
        assert_eq!(TextAlignment::default(), TextAlignment::Left);
    }
}
