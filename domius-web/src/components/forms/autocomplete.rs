//! Autocomplete component - Text input with suggestions.

use domius_core::signal::Signal;
use web_sys::Element;

/// Props for the Autocomplete component.
pub struct AutocompleteProps {
    pub options: Vec<String>,
    pub value: Option<Signal<String>>,
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub clearable: bool,
    pub highlight_matches: bool,
    pub on_select: Option<Box<dyn Fn(String)>>,
    pub on_input: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for AutocompleteProps {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            value: None,
            placeholder: None,
            disabled: false,
            clearable: false,
            highlight_matches: true,
            on_select: None,
            on_input: None,
            class: None,
        }
    }
}

/// Autocomplete component.
pub struct Autocomplete;

impl Autocomplete {
    /// Create an autocomplete element.
    pub fn create(_props: AutocompleteProps) -> (Element, Option<Signal<String>>) {
        // TODO: Implement autocomplete
        todo!("Autocomplete component implementation pending")
    }
}
