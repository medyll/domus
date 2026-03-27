//! MultiSelect component - Multiple value selection with tags.

use domius_core::signal::Signal;
use web_sys::Element;

/// Props for the MultiSelect component.
pub struct MultiSelectProps {
    pub options: Vec<String>,
    pub values: Signal<Vec<String>>,
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub searchable: bool,
    pub creatable: bool,
    pub max_selections: Option<usize>,
    pub on_change: Option<Box<dyn Fn(Vec<String>)>>,
    pub class: Option<String>,
}

impl Default for MultiSelectProps {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            values: domius_core::signal::signal(Vec::new()),
            placeholder: None,
            disabled: false,
            searchable: true,
            creatable: false,
            max_selections: None,
            on_change: None,
            class: None,
        }
    }
}

/// MultiSelect component.
pub struct MultiSelect;

impl MultiSelect {
    /// Create a multi-select element.
    pub fn create(_props: MultiSelectProps) -> (Element, Signal<Vec<String>>) {
        // TODO: Implement multi-select
        todo!("MultiSelect component implementation pending")
    }
}
