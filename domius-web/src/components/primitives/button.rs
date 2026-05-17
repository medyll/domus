//! Button component - A clickable button with various styles and states.

use domius_core::signal::Signal;
use domius_core::effect::create_effect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlButtonElement, MouseEvent, FocusEvent};

use crate::component::{DomiusComponent, DomiusNode};
use crate::hooks::use_focus;

/// Button visual variant.
#[derive(Clone, PartialEq, Debug)]
pub enum ButtonVariant {
    /// Primary action button (filled, prominent color)
    Primary,
    /// Secondary action button (outlined)
    Secondary,
    /// Tertiary action button (text-only, minimal)
    Text,
    /// Danger/destructive action
    Danger,
    /// Ghost button (transparent background)
    Ghost,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

/// Button size.
#[derive(Clone, PartialEq, Debug)]
pub enum ButtonSize {
    /// Small button
    Sm,
    /// Medium button (default)
    Md,
    /// Large button
    Lg,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Md
    }
}

/// Props for the Button component.
pub struct ButtonProps {
    /// Button text content
    pub children: String,
    /// Visual variant
    pub variant: ButtonVariant,
    /// Size
    pub size: ButtonSize,
    /// Whether the button is disabled
    pub disabled: bool,
    /// Whether the button is in loading state
    pub loading: bool,
    /// Loading text to display (optional)
    pub loading_text: Option<String>,
    /// Full width button
    pub full_width: bool,
    /// Left icon (emoji or character)
    pub left_icon: Option<String>,
    /// Right icon (emoji or character)
    pub right_icon: Option<String>,
    /// Click handler
    pub on_click: Option<Box<dyn Fn()>>,
    /// Additional CSS classes
    pub class: Option<String>,
    /// Button type
    pub button_type: ButtonType,
}

/// Button type attribute.
#[derive(Clone, PartialEq, Debug)]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Button
    }
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            children: String::new(),
            variant: ButtonVariant::default(),
            size: ButtonSize::default(),
            disabled: false,
            loading: false,
            loading_text: None,
            full_width: false,
            left_icon: None,
            right_icon: None,
            on_click: None,
            class: None,
            button_type: ButtonType::default(),
        }
    }
}

/// Internal state for the Button component.
pub struct ButtonState {
    pub is_hovered: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub is_focused: Signal<bool>,
}

/// Button component.
///
/// # Example
/// ```ignore
/// let button_props = ButtonProps {
///     children: "Click me".to_string(),
///     variant: ButtonVariant::Primary,
///     on_click: Some(Box::new(|| println!("Clicked!"))),
///     ..Default::default()
/// };
/// mount_component::<Button>(&button_props, &parent);
/// ```
pub struct Button;

impl DomiusComponent for Button {
    type Props = ButtonProps;
    type State = ButtonState;

    fn setup(_props: ButtonProps) -> Self::State {
        ButtonState {
            is_hovered: domius_core::signal::signal(false),
            is_pressed: domius_core::signal::signal(false),
            is_focused: domius_core::signal::signal(false),
        }
    }

    fn render(state: &ButtonState) -> DomiusNode {
        // This will be called by mount_component which passes props
        // For now, we need to store props somewhere accessible
        // In a real implementation, we'd use a different pattern
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let button: HtmlButtonElement = document
            .create_element("button")
            .unwrap()
            .dyn_into()
            .unwrap();

        button.into()
    }
}

// We need a different approach - let's create a simpler implementation
// that doesn't require the trait pattern for now

/// Create a button element with the given properties.
///
/// This is a helper function that creates a button without using the full
/// DomiusComponent trait pattern, for simpler use cases.
pub fn create_button(
    children: &str,
    variant: ButtonVariant,
    on_click: Option<Box<dyn Fn()>>,
) -> Element {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");

    let button: HtmlButtonElement = document
        .create_element("button")
        .unwrap()
        .dyn_into()
        .unwrap();

    // Build class names
    let mut classes = vec!["domius-btn".to_string()];
    classes.push(format!("domius-btn-{:?}", variant).to_lowercase());

    button.set_attribute("class", &classes.join(" ")).unwrap();
    button.set_text_content(Some(children));

    // Attach click handler
    if let Some(handler) = on_click {
        let closure = Closure::wrap(handler as Box<dyn Fn()>);
        button
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    button.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variant_default() {
        assert_eq!(ButtonVariant::default(), ButtonVariant::Primary);
    }

    #[test]
    fn test_button_size_default() {
        assert_eq!(ButtonSize::default(), ButtonSize::Md);
    }

    #[test]
    fn test_button_props_default() {
        let props = ButtonProps::default();
        assert_eq!(props.variant, ButtonVariant::Primary);
        assert_eq!(props.size, ButtonSize::Md);
        assert!(!props.disabled);
        assert!(!props.loading);
    }
}
