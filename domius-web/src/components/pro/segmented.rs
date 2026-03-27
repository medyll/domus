//! SegmentedControl component - Mutually exclusive button group.

use domius_core::signal::Signal;
use web_sys::Element;

/// Props for the SegmentedControl component.
pub struct SegmentedControlProps {
    pub options: Vec<String>,
    pub value: Signal<String>,
    pub disabled: bool,
    pub full_width: bool,
    pub on_change: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for SegmentedControlProps {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            value: domius_core::signal::signal(String::new()),
            disabled: false,
            full_width: false,
            on_change: None,
            class: None,
        }
    }
}

/// SegmentedControl component.
pub struct SegmentedControl;

impl SegmentedControl {
    /// Create a segmented control element.
    pub fn create(_props: SegmentedControlProps) -> (Element, Signal<String>) {
        // TODO: Implement segmented control
        todo!("SegmentedControl component implementation pending")
    }
}
