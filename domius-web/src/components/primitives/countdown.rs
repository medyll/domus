//! Countdown component for Domius.
//!
//! A countdown timer display.

use web_sys::Element;

/// Countdown format.
#[derive(Clone, Copy, Default)]
pub enum CountdownFormat {
    #[default]
    HHmmss,
    HHmm,
    DDHHmm,
    Custom,
}

/// Countdown props.
#[derive(Clone, Default)]
pub struct CountdownProps {
    /// Target time (timestamp in seconds)
    pub target: u64,
    /// Current time (timestamp in seconds)
    pub current: Option<u64>,
    /// CSS class
    pub class: Option<String>,
    /// Format template
    pub format: CountdownFormat,
    /// Custom format string (if Custom format)
    pub format_string: Option<String>,
    /// Show title
    pub title: Option<String>,
    /// Finished callback text
    pub finish_text: Option<String>,
}

/// Build a Countdown component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::countdown::{countdown, CountdownProps, CountdownFormat};
///
/// let countdown_node = countdown(CountdownProps {
///     target: 1735689600, // Future timestamp
///     format: CountdownFormat::DDHHmm,
///     ..Default::default()
/// });
/// ```
pub fn countdown(props: CountdownProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("countdown");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Title
    if let Some(title) = &props.title {
        let title_el: Element = document.create_element("div").unwrap();
        title_el.set_class_name("countdown-title");
        title_el.set_text_content(Some(title));
        container.append_child(&title_el).unwrap();
    }

    // Value display
    let value_el: Element = document.create_element("div").unwrap();
    value_el.set_class_name("countdown-value");

    // Calculate remaining time
    let current = props.current.unwrap_or_else(|| {
        // Use current time if not provided
        js_sys::Date::now() as u64 / 1000
    });

    let remaining = if props.target > current {
        props.target - current
    } else {
        // Countdown finished
        if let Some(finish_text) = &props.finish_text {
            value_el.set_text_content(Some(finish_text));
            container.append_child(&value_el).unwrap();
            return container;
        }
        0
    };

    // Format the time
    let formatted = format_countdown(remaining, &props.format, &props.format_string);
    value_el.set_text_content(Some(&formatted));
    container.append_child(&value_el).unwrap();

    container
}

fn format_countdown(
    remaining: u64,
    format: &CountdownFormat,
    format_string: &Option<String>,
) -> String {
    let days = remaining / 86400;
    let hours = (remaining % 86400) / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    match format {
        CountdownFormat::HHmmss => {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        }
        CountdownFormat::HHmm => {
            format!("{:02}:{:02}", hours, minutes)
        }
        CountdownFormat::DDHHmm => {
            format!("{}d {:02}h {:02}m", days, hours, minutes)
        }
        CountdownFormat::Custom => {
            if let Some(fmt) = format_string {
                fmt.replace("DD", &days.to_string())
                    .replace("HH", &format!("{:02}", hours))
                    .replace("mm", &format!("{:02}", minutes))
                    .replace("ss", &format!("{:02}", seconds))
            } else {
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
            }
        }
    }
}
