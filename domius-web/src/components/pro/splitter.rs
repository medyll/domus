//! Splitter component - Resizable divider between panels.

use domius_core::signal::Signal;
use web_sys::Element;

/// Props for the Splitter component.
pub struct SplitterProps {
    pub initial_position: f64,
    pub orientation: SplitterOrientation,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub on_change: Option<Box<dyn Fn(f64)>>,
    pub class: Option<String>,
}

/// Splitter orientation.
#[derive(Clone, PartialEq)]
pub enum SplitterOrientation {
    Horizontal,
    Vertical,
}

impl Default for SplitterOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl Default for SplitterProps {
    fn default() -> Self {
        Self {
            initial_position: 50.0,
            orientation: SplitterOrientation::default(),
            min: None,
            max: None,
            step: None,
            on_change: None,
            class: None,
        }
    }
}

/// Splitter component.
pub struct Splitter;

impl Splitter {
    /// Create a splitter element.
    pub fn create(_props: SplitterProps) -> (Element, Signal<f64>) {
        // TODO: Implement splitter with drag handle
        todo!("Splitter component implementation pending")
    }
}
