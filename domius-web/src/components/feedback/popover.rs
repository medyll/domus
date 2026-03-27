//! Popover component - Click-activated popup.

use domius_core::signal::{signal, Signal};
use web_sys::Element;

/// Popover position.
#[derive(Clone, PartialEq)]
pub enum PopoverPosition {
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    BottomStart,
    BottomEnd,
}

impl Default for PopoverPosition {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Props for the Popover component.
#[derive(Clone)]
pub struct PopoverProps {
    pub content: String,
    pub title: Option<String>,
    pub position: PopoverPosition,
    pub trigger: PopoverTrigger,
    pub disabled: bool,
    pub children: Element,
    pub class: Option<String>,
}

/// Popover trigger type.
#[derive(Clone, PartialEq)]
pub enum PopoverTrigger {
    Click,
    Hover,
    Focus,
}

impl Default for PopoverTrigger {
    fn default() -> Self {
        Self::Click
    }
}

impl Default for PopoverProps {
    fn default() -> Self {
        Self {
            content: String::new(),
            title: None,
            position: PopoverPosition::default(),
            trigger: PopoverTrigger::default(),
            disabled: false,
            children: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("span")
                .unwrap()
                .into(),
            class: None,
        }
    }
}

/// Popover component.
pub struct Popover;

impl Popover {
    /// Create a popover wrapper element.
    pub fn create(_props: PopoverProps) -> Element {
        // TODO: Implement popover
        todo!("Popover component implementation pending")
    }
}
