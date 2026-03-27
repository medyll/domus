//! Stepper component - Multi-step process wizard.

use domius_core::signal::Signal;
use web_sys::Element;

/// A single step in the stepper.
#[derive(Clone)]
pub struct Step {
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub optional: bool,
}

/// Stepper orientation.
#[derive(Clone, PartialEq)]
pub enum StepperOrientation {
    Horizontal,
    Vertical,
}

impl Default for StepperOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// Props for the Stepper component.
pub struct StepperProps {
    pub steps: Vec<Step>,
    pub active_step: Option<usize>,
    pub completed_steps: Vec<usize>,
    pub orientation: StepperOrientation,
    pub on_step_change: Option<Box<dyn Fn(usize)>>,
    pub class: Option<String>,
}

impl Default for StepperProps {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            active_step: Some(0),
            completed_steps: Vec::new(),
            orientation: StepperOrientation::default(),
            on_step_change: None,
            class: None,
        }
    }
}

/// Stepper component.
pub struct Stepper;

impl Stepper {
    /// Create a stepper element.
    pub fn create(_props: StepperProps) -> (Element, Signal<usize>) {
        // TODO: Implement stepper
        todo!("Stepper component implementation pending")
    }
}
