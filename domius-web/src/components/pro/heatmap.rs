//! Heatmap component - Data density visualization.

use web_sys::Element;

/// Heatmap cell data.
#[derive(Clone)]
pub struct HeatmapCell {
    pub x: usize,
    pub y: usize,
    pub value: f64,
}

/// Props for the Heatmap component.
pub struct HeatmapProps {
    pub data: Vec<HeatmapCell>,
    pub x_labels: Vec<String>,
    pub y_labels: Vec<String>,
    pub color_scale: HeatmapColorScale,
    pub show_values: bool,
    pub on_cell_click: Option<Box<dyn Fn(usize, usize)>>,
    pub class: Option<String>,
}

/// Color scale for heatmap.
#[derive(Clone)]
pub enum HeatmapColorScale {
    Sequential(Vec<String>),
    Diverging(Vec<String>),
    Categorical(Vec<String>),
}

impl Default for HeatmapProps {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            x_labels: Vec::new(),
            y_labels: Vec::new(),
            color_scale: HeatmapColorScale::Sequential(vec![
                "#ffffff".to_string(),
                "#0000ff".to_string(),
            ]),
            show_values: false,
            on_cell_click: None,
            class: None,
        }
    }
}

/// Heatmap component.
pub struct Heatmap;

impl Heatmap {
    /// Create a heatmap element.
    pub fn create(_props: HeatmapProps) -> Element {
        // TODO: Implement heatmap
        todo!("Heatmap component implementation pending")
    }
}
