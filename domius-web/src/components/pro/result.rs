//! Result component - Status page for operations.

use web_sys::Element;

/// Result status.
#[derive(Clone, PartialEq)]
pub enum ResultStatus {
    Success,
    Error,
    Info,
    Warning,
    Custom(String),
}

/// Props for the Result component.
#[derive(Clone)]
pub struct ResultProps {
    pub status: ResultStatus,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub extra_actions: Option<String>,
    pub class: Option<String>,
}

impl Default for ResultProps {
    fn default() -> Self {
        Self {
            status: ResultStatus::Success,
            title: String::new(),
            description: None,
            icon: None,
            extra_actions: None,
            class: None,
        }
    }
}

/// Result component.
pub struct Result;

impl Result {
    /// Create a result element.
    pub fn create(props: ResultProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let container = document.create_element("div").unwrap();
        container.set_attribute("class", "domius-result").unwrap();

        // Icon
        let icon_container = document.create_element("div").unwrap();
        icon_container.set_attribute("class", "domius-result-icon").unwrap();
        
        let icon_char = match props.status {
            ResultStatus::Success => "✓",
            ResultStatus::Error => "✕",
            ResultStatus::Info => "ℹ",
            ResultStatus::Warning => "⚠",
            ResultStatus::Custom(_) => &props.icon.unwrap_or_else(|| "•".to_string()),
        };
        icon_container.set_text_content(Some(icon_char));
        container.append_child(&icon_container).unwrap();

        // Title
        let title = document.create_element("h2").unwrap();
        title.set_attribute("class", "domius-result-title").unwrap();
        title.set_text_content(Some(&props.title));
        container.append_child(&title).unwrap();

        // Description
        if let Some(desc) = &props.description {
            let desc_el = document.create_element("p").unwrap();
            desc_el.set_attribute("class", "domius-result-description").unwrap();
            desc_el.set_text_content(Some(desc));
            container.append_child(&desc_el).unwrap();
        }

        // Extra actions
        if let Some(actions) = &props.extra_actions {
            let actions_el = document.create_element("div").unwrap();
            actions_el.set_attribute("class", "domius-result-actions").unwrap();
            actions_el.set_inner_html(actions);
            container.append_child(&actions_el).unwrap();
        }

        container.into()
    }

    /// Create a success result.
    pub fn success(title: impl Into<String>, description: Option<&str>) -> Element {
        Self::create(ResultProps {
            status: ResultStatus::Success,
            title: title.into(),
            description: description.map(String::from),
            ..Default::default()
        })
    }

    /// Create an error result.
    pub fn error(title: impl Into<String>, description: Option<&str>) -> Element {
        Self::create(ResultProps {
            status: ResultStatus::Error,
            title: title.into(),
            description: description.map(String::from),
            ..Default::default()
        })
    }

    /// Create a 404 result.
    pub fn not_found() -> Element {
        Self::create(ResultProps {
            status: ResultStatus::Custom("404".to_string()),
            title: "Page Not Found".to_string(),
            description: Some("The page you're looking for doesn't exist.".to_string()),
            ..Default::default()
        })
    }
}
