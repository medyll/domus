//! GanttChart component - Project timeline visualization.

use web_sys::Element;

/// Gantt task.
#[derive(Clone)]
pub struct GanttTask {
    pub id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub progress: u8,
    pub dependencies: Vec<String>,
    pub collapsed: bool,
    pub children: Vec<GanttTask>,
}

/// Gantt zoom level.
#[derive(Clone, PartialEq)]
pub enum GanttZoom {
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

/// Props for the GanttChart component.
pub struct GanttChartProps {
    pub tasks: Vec<GanttTask>,
    pub zoom: GanttZoom,
    pub show_dependencies: bool,
    pub show_progress: bool,
    pub readonly: bool,
    pub on_task_change: Option<Box<dyn Fn(GanttTask)>>,
    pub class: Option<String>,
}

impl Default for GanttChartProps {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            zoom: GanttZoom::Week,
            show_dependencies: true,
            show_progress: true,
            readonly: false,
            on_task_change: None,
            class: None,
        }
    }
}

/// GanttChart component.
pub struct GanttChart;

impl GanttChart {
    /// Create a Gantt chart element.
    pub fn create(_props: GanttChartProps) -> Element {
        // TODO: Implement Gantt chart
        todo!("GanttChart component implementation pending")
    }
}
