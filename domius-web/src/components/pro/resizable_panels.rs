//! ResizablePanels component - User-resizable panel groups.

use domius_core::signal::Signal;
use web_sys::Element;

/// Props for the ResizablePanels component.
pub struct ResizablePanelsProps {
    pub initial_sizes: Vec<f64>,
    pub min_sizes: Vec<f64>,
    pub orientation: PanelOrientation,
    pub handle_size: u32,
    pub on_resize: Option<Box<dyn Fn(Vec<f64>)>>,
    pub class: Option<String>,
}

/// Panel orientation.
#[derive(Clone, PartialEq)]
pub enum PanelOrientation {
    Horizontal,
    Vertical,
}

impl Default for PanelOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl Default for ResizablePanelsProps {
    fn default() -> Self {
        Self {
            initial_sizes: vec![50.0, 50.0],
            min_sizes: vec![0.0, 0.0],
            orientation: PanelOrientation::default(),
            handle_size: 4,
            on_resize: None,
            class: None,
        }
    }
}

/// ResizablePanels component.
pub struct ResizablePanels;

impl ResizablePanels {
    /// Create a resizable panels element.
    pub fn create(_props: ResizablePanelsProps) -> (Element, Signal<Vec<f64>>) {
        // TODO: Implement resizable panels with drag handles
        todo!("ResizablePanels component implementation pending")
    }
}
