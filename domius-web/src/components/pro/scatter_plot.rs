//! ScatterPlot component - Correlation visualization.

use web_sys::Element;

/// Data point for scatter plot.
#[derive(Clone)]
pub struct ScatterPoint {
    pub x: f64,
    pub y: f64,
    pub label: Option<String>,
    pub color: Option<String>,
    pub size: Option<f64>,
}

/// Props for the ScatterPlot component.
#[derive(Clone)]
pub struct ScatterPlotProps {
    pub points: Vec<ScatterPoint>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub x_min: Option<f64>,
    pub x_max: Option<f64>,
    pub y_min: Option<f64>,
    pub y_max: Option<f64>,
    pub show_grid: bool,
    pub show_labels: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub class: Option<String>,
}

impl Default for ScatterPlotProps {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            x_label: None,
            y_label: None,
            x_min: None,
            x_max: None,
            y_min: None,
            y_max: None,
            show_grid: true,
            show_labels: false,
            width: Some(400),
            height: Some(300),
            class: None,
        }
    }
}

/// ScatterPlot component.
pub struct ScatterPlot;

impl ScatterPlot {
    /// Create a scatter plot element.
    pub fn create(_props: ScatterPlotProps) -> Element {
        // TODO: Implement scatter plot (SVG-based)
        todo!("ScatterPlot component implementation pending")
    }
}
