//! Timeline component - Chronological event list.

use web_sys::Element;

/// A single timeline event.
#[derive(Clone)]
pub struct TimelineEvent {
    pub title: String,
    pub description: Option<String>,
    pub timestamp: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

/// Timeline orientation.
#[derive(Clone, PartialEq)]
pub enum TimelineOrientation {
    Vertical,
    Horizontal,
}

impl Default for TimelineOrientation {
    fn default() -> Self {
        Self::Vertical
    }
}

/// Props for the Timeline component.
#[derive(Clone)]
pub struct TimelineProps {
    pub events: Vec<TimelineEvent>,
    pub orientation: TimelineOrientation,
    pub alternate: bool,
    pub class: Option<String>,
}

impl Default for TimelineProps {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            orientation: TimelineOrientation::default(),
            alternate: false,
            class: None,
        }
    }
}

/// Timeline component.
pub struct Timeline;

impl Timeline {
    /// Create a timeline element.
    pub fn create(_props: TimelineProps) -> Element {
        // TODO: Implement timeline
        todo!("Timeline component implementation pending")
    }
}
