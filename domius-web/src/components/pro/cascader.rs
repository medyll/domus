//! Cascader component - Hierarchical selection.

use domius_core::signal::Signal;
use web_sys::Element;

/// Cascader option.
#[derive(Clone)]
pub struct CascaderOption {
    pub value: String,
    pub label: String,
    pub children: Vec<CascaderOption>,
    pub disabled: bool,
    pub leaf: bool,
}

/// Props for the Cascader component.
pub struct CascaderProps {
    pub options: Vec<CascaderOption>,
    pub value: Option<Signal<Vec<String>>>,
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub multiple: bool,
    pub show_path: bool,
    pub on_change: Option<Box<dyn Fn(Vec<String>)>>,
    pub class: Option<String>,
}

impl Default for CascaderProps {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            value: None,
            placeholder: None,
            disabled: false,
            multiple: false,
            show_path: true,
            on_change: None,
            class: None,
        }
    }
}

/// Cascader component.
pub struct Cascader;

impl Cascader {
    /// Create a cascader element.
    pub fn create(_props: CascaderProps) -> (Element, Option<Signal<Vec<String>>>) {
        // TODO: Implement cascader
        todo!("Cascader component implementation pending")
    }
}
