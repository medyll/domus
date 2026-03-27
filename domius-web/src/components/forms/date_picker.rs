//! DatePicker component - Calendar date selection.

use domius_core::signal::Signal;
use web_sys::Element;

/// Props for the DatePicker component.
pub struct DatePickerProps {
    pub value: Option<Signal<String>>,
    pub min_date: Option<String>,
    pub max_date: Option<String>,
    pub disabled: bool,
    pub format: String,
    pub show_time: bool,
    pub on_change: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for DatePickerProps {
    fn default() -> Self {
        Self {
            value: None,
            min_date: None,
            max_date: None,
            disabled: false,
            format: "YYYY-MM-DD".to_string(),
            show_time: false,
            on_change: None,
            class: None,
        }
    }
}

/// DatePicker component.
pub struct DatePicker;

impl DatePicker {
    /// Create a date picker element.
    pub fn create(_props: DatePickerProps) -> (Element, Option<Signal<String>>) {
        // TODO: Implement date picker
        todo!("DatePicker component implementation pending")
    }
}
