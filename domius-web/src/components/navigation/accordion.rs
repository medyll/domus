//! Accordion component - Collapsible content panels.
//!
//! Stub implementation - to be completed.

use domius_core::signal::{signal, Signal};
use web_sys::Element;

/// Props for the Accordion component.
#[derive(Clone)]
pub struct AccordionProps {
    /// Allow multiple panels to be open simultaneously
    pub allow_multiple: bool,
    /// Initially expanded panel indices
    pub default_expanded: Vec<usize>,
    /// Additional CSS classes
    pub class: Option<String>,
}

impl Default for AccordionProps {
    fn default() -> Self {
        Self {
            allow_multiple: false,
            default_expanded: Vec::new(),
            class: None,
        }
    }
}

/// A single accordion item.
#[derive(Clone)]
pub struct AccordionItem {
    pub title: String,
    pub content: String,
    pub disabled: bool,
    pub icon: Option<String>,
}

/// Accordion component.
pub struct Accordion;

impl Accordion {
    /// Create an accordion element.
    pub fn create(
        _props: AccordionProps,
        _items: Vec<AccordionItem>,
    ) -> (Element, Signal<Vec<usize>>) {
        // TODO: Implement accordion
        todo!("Accordion component implementation pending")
    }
}
