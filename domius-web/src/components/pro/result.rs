//! Result component - Status page for operations.

use web_sys::{Document, Element};

/// Result status.
#[derive(Clone, PartialEq)]
pub enum ResultStatus {
    Success,
    Error,
    Info,
    Warning,
    Custom(String),
}

impl ResultStatus {
    fn token(&self) -> &str {
        match self {
            Self::Success => "success",
            Self::Error => "error",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Custom(status) => status,
        }
    }

    fn glyph(&self) -> Option<&'static str> {
        match self {
            Self::Success => Some("✓"),
            Self::Error => Some("✕"),
            Self::Info => Some("ℹ"),
            Self::Warning => Some("⚠"),
            Self::Custom(_) => None,
        }
    }
}

/// A way out of the result state, rendered as a real link.
#[derive(Clone)]
pub struct ResultAction {
    pub label: String,
    pub href: String,
    pub primary: bool,
}

impl ResultAction {
    /// Create a secondary action pointing at `href`.
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            primary: false,
        }
    }

    /// Mark this action as the recommended way out.
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }
}

/// Props for the Result component.
#[derive(Clone)]
pub struct ResultProps {
    pub status: ResultStatus,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub actions: Vec<ResultAction>,
    pub class: Option<String>,
}

impl Default for ResultProps {
    fn default() -> Self {
        Self {
            status: ResultStatus::Success,
            title: String::new(),
            description: None,
            icon: None,
            actions: Vec::new(),
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
        let container = document.create_element("section").expect("create result");
        let mut classes = vec!["domius-result"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        container.set_class_name(&classes.join(" "));
        container
            .set_attribute("data-status", props.status.token())
            .expect("set result status");
        // The result replaces a view, so announce it once it lands.
        container
            .set_attribute("role", "status")
            .expect("set result role");

        let icon = props
            .icon
            .as_deref()
            .or_else(|| props.status.glyph())
            .unwrap_or("•");
        append_text(&document, &container, "p", "domius-result-icon", icon)
            .set_attribute("aria-hidden", "true")
            .expect("hide decorative result icon");
        append_text(
            &document,
            &container,
            "h2",
            "domius-result-title",
            &props.title,
        );
        if let Some(description) = props.description.as_deref() {
            append_text(
                &document,
                &container,
                "p",
                "domius-result-description",
                description,
            );
        }
        if !props.actions.is_empty() {
            append_actions(&document, &container, &props.actions);
        }
        container
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

    /// Create a 404 result, with the links that lead out of it.
    pub fn not_found(actions: Vec<ResultAction>) -> Element {
        Self::create(ResultProps {
            status: ResultStatus::Custom("404".to_string()),
            title: "Page Not Found".to_string(),
            description: Some("The page you're looking for doesn't exist.".to_string()),
            actions,
            ..Default::default()
        })
    }
}

fn append_text(
    document: &Document,
    container: &Element,
    tag: &str,
    class: &str,
    text: &str,
) -> Element {
    let element = document.create_element(tag).expect("create result part");
    element.set_class_name(class);
    element.set_text_content(Some(text));
    container
        .append_child(&element)
        .expect("append result part");
    element
}

fn append_actions(document: &Document, container: &Element, actions: &[ResultAction]) {
    let nav = document
        .create_element("nav")
        .expect("create result actions");
    nav.set_class_name("domius-result-actions");
    nav.set_attribute("aria-label", "Result actions")
        .expect("label result actions");
    for action in actions {
        let link = document.create_element("a").expect("create result action");
        link.set_class_name("domius-result-action");
        link.set_attribute("href", &action.href)
            .expect("target result action");
        if action.primary {
            link.set_attribute("data-primary", "true")
                .expect("mark primary result action");
        }
        link.set_text_content(Some(&action.label));
        nav.append_child(&link).expect("append result action");
    }
    container.append_child(&nav).expect("append result actions");
}
