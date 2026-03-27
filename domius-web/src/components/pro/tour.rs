//! Tour component - User onboarding guide.

use domius_core::signal::Signal;
use web_sys::Element;

/// Tour step.
#[derive(Clone)]
pub struct TourStep {
    pub target_id: String,
    pub title: String,
    pub description: String,
    pub position: TourPosition,
}

/// Tour step position.
#[derive(Clone, PartialEq)]
pub enum TourPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for TourPosition {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Props for the Tour component.
pub struct TourProps {
    pub steps: Vec<TourStep>,
    pub active: Signal<bool>,
    pub current_step: Signal<usize>,
    pub show_arrows: bool,
    pub show_indicators: bool,
    pub close_on_overlay: bool,
    pub on_finish: Option<Box<dyn Fn()>>,
    pub on_skip: Option<Box<dyn Fn()>>,
    pub on_step_change: Option<Box<dyn Fn(usize)>>,
    pub class: Option<String>,
}

impl Default for TourProps {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            active: domius_core::signal::signal(false),
            current_step: domius_core::signal::signal(0),
            show_arrows: true,
            show_indicators: true,
            close_on_overlay: true,
            on_finish: None,
            on_skip: None,
            on_step_change: None,
            class: None,
        }
    }
}

/// Tour component.
pub struct Tour;

impl Tour {
    /// Create a tour overlay element.
    pub fn create(_props: TourProps) -> Element {
        // TODO: Implement tour with positioned step bubbles
        todo!("Tour component implementation pending")
    }
}
