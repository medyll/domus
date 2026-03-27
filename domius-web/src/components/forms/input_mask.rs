//! InputMask component - Formatted text input.

use domius_core::signal::Signal;
use web_sys::Element;

/// Props for the InputMask component.
pub struct InputMaskProps {
    pub mask: String,
    pub value: Option<Signal<String>>,
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub on_change: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for InputMaskProps {
    fn default() -> Self {
        Self {
            mask: String::new(),
            value: None,
            placeholder: None,
            disabled: false,
            on_change: None,
            class: None,
        }
    }
}

/// InputMask component.
pub struct InputMask;

impl InputMask {
    /// Create an input mask element.
    pub fn create(_props: InputMaskProps) -> (Element, Option<Signal<String>>) {
        // TODO: Implement input mask
        todo!("InputMask component implementation pending")
    }
}
