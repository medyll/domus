//! Select component - Dropdown selection.

use domius_core::signal::Signal;
use web_sys::Element;

/// A select option.
#[derive(Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// Props for the Select component.
pub struct SelectProps {
    pub options: Vec<SelectOption>,
    pub value: Option<Signal<String>>,
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub multiple: bool,
    pub searchable: bool,
    pub clearable: bool,
    pub on_change: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for SelectProps {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            value: None,
            placeholder: None,
            disabled: false,
            multiple: false,
            searchable: false,
            clearable: false,
            on_change: None,
            class: None,
        }
    }
}

/// Select component.
pub struct Select;

impl Select {
    /// Create a select element.
    pub fn create(_props: SelectProps) -> (Element, Option<Signal<String>>) {
        // TODO: Implement select
        todo!("Select component implementation pending")
    }
}
