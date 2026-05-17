//! Toast/Snackbar component - Non-intrusive notifications.

use domius_core::signal::{signal, Signal};
use domius_core::effect::create_effect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, MouseEvent};

use crate::context::{provide_context, use_context};

/// Toast variant.
#[derive(Clone, PartialEq)]
pub enum ToastVariant {
    Info,
    Success,
    Warning,
    Error,
}

impl Default for ToastVariant {
    fn default() -> Self {
        Self::Info
    }
}

impl ToastVariant {
    pub fn as_class(&self) -> &'static str {
        match self {
            ToastVariant::Info => "domius-toast-info",
            ToastVariant::Success => "domius-toast-success",
            ToastVariant::Warning => "domius-toast-warning",
            ToastVariant::Error => "domius-toast-error",
        }
    }

    pub fn as_icon(&self) -> &'static str {
        match self {
            ToastVariant::Info => "ℹ",
            ToastVariant::Success => "✓",
            ToastVariant::Warning => "⚠",
            ToastVariant::Error => "✕",
        }
    }
}

/// A single toast notification.
#[derive(Clone)]
pub struct ToastData {
    pub id: String,
    pub message: String,
    pub title: Option<String>,
    pub variant: ToastVariant,
    pub duration: Option<u64>,
    pub dismissible: bool,
}

/// Props for the Toast component.
pub struct ToastProps {
    pub data: ToastData,
    pub on_dismiss: Option<Box<dyn Fn(String)>>,
}

/// Toast component (single notification).
pub struct Toast;

impl Toast {
    /// Create a toast element.
    pub fn create(props: ToastProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let toast: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();

        let mut classes = vec![
            "domius-toast".to_string(),
            props.data.variant.as_class().to_string(),
        ];
        toast.set_attribute("class", &classes.join(" ")).unwrap();

        // Icon
        let icon: HtmlElement = document
            .create_element("span")
            .unwrap()
            .dyn_into()
            .unwrap();
        icon.set_attribute("class", "domius-toast-icon").unwrap();
        icon.set_text_content(Some(props.data.variant.as_icon()));
        toast.append_child(&icon).unwrap();

        // Content
        let content: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();
        content.set_attribute("class", "domius-toast-content").unwrap();

        if let Some(title) = &props.data.title {
            let title_el: HtmlElement = document
                .create_element("div")
                .unwrap()
                .dyn_into()
                .unwrap();
            title_el.set_attribute("class", "domius-toast-title").unwrap();
            title_el.set_text_content(Some(title));
            content.append_child(&title_el).unwrap();
        }

        let message_el: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();
        message_el.set_attribute("class", "domius-toast-message").unwrap();
        message_el.set_text_content(Some(&props.data.message));
        content.append_child(&message_el).unwrap();

        toast.append_child(&content).unwrap();

        // Dismiss button
        if props.data.dismissible {
            let dismiss_btn: HtmlElement = document
                .create_element("button")
                .unwrap()
                .dyn_into()
                .unwrap();
            dismiss_btn.set_attribute("class", "domius-toast-dismiss").unwrap();
            dismiss_btn.set_text_content(Some("×"));

            if let Some(handler) = props.on_dismiss.as_ref() {
                let toast_id = props.data.id.clone();
                let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                    handler(toast_id.clone());
                }) as Box<dyn FnMut(MouseEvent)>);
                dismiss_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
            }

            toast.append_child(&dismiss_btn).unwrap();
        }

        toast.into()
    }
}

/// Toast manager context for managing multiple toasts.
#[derive(Clone)]
pub struct ToastManager {
    pub toasts: Signal<Vec<ToastData>>,
}

impl ToastManager {
    /// Create a new toast manager.
    pub fn new() -> Self {
        Self {
            toasts: signal(Vec::new()),
        }
    }

    /// Add a toast.
    pub fn add(&self, toast: ToastData) {
        let mut current = self.toasts.get();
        current.push(toast);
        self.toasts.set(current);
    }

    /// Remove a toast by ID.
    pub fn remove(&self, id: &str) {
        let mut current = self.toasts.get();
        current.retain(|t| t.id != id);
        self.toasts.set(current);
    }

    /// Show an info toast.
    pub fn info(&self, message: impl Into<String>) {
        let id = format!("toast-{}", js_sys::Date::now() as u64);
        self.add(ToastData {
            id,
            message: message.into(),
            title: None,
            variant: ToastVariant::Info,
            duration: Some(5000),
            dismissible: true,
        });
    }

    /// Show a success toast.
    pub fn success(&self, message: impl Into<String>) {
        let id = format!("toast-{}", js_sys::Date::now() as u64);
        self.add(ToastData {
            id,
            message: message.into(),
            title: None,
            variant: ToastVariant::Success,
            duration: Some(3000),
            dismissible: true,
        });
    }

    /// Show a warning toast.
    pub fn warning(&self, message: impl Into<String>) {
        let id = format!("toast-{}", js_sys::Date::now() as u64);
        self.add(ToastData {
            id,
            message: message.into(),
            title: None,
            variant: ToastVariant::Warning,
            duration: Some(5000),
            dismissible: true,
        });
    }

    /// Show an error toast.
    pub fn error(&self, message: impl Into<String>) {
        let id = format!("toast-{}", js_sys::Date::now() as u64);
        self.add(ToastData {
            id,
            message: message.into(),
            title: None,
            variant: ToastVariant::Error,
            duration: Some(7000),
            dismissible: true,
        });
    }
}

impl Default for ToastManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Toast container component that renders all toasts.
pub struct ToastContainer;

impl ToastContainer {
    /// Create a toast container element.
    pub fn create() -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let container: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();
        container.set_attribute("class", "domius-toast-container").unwrap();

        // Get toast manager from context or create new one
        let manager = use_context::<ToastManager>()
            .unwrap_or_else(|| {
                let m = ToastManager::new();
                provide_context(m.clone());
                m
            });

        // Render toasts reactively
        let container_clone = container.clone();
        create_effect(move || {
            let toasts = manager.toasts.get();
            
            // Clear container
            while container_clone.first_child().is_some() {
                container_clone.remove_child(&container_clone.first_child().unwrap()).ok();
            }

            // Add each toast
            for toast_data in toasts.iter() {
                let manager_clone = manager.clone();
                let toast_el = Toast::create(ToastProps {
                    data: toast_data.clone(),
                    on_dismiss: Some(Box::new(move |id| {
                        manager_clone.remove(&id);
                    })),
                });
                container_clone.append_child(&toast_el).unwrap();
            }
        });

        container.into()
    }
}
