//! Tabs component - Organize content into selectable panels.
//!
//! # Example
//! ```ignore
//! let tabs = vec![
//!     Tab { id: "tab1".to_string(), label: "First".to_string() },
//!     Tab { id: "tab2".to_string(), label: "Second".to_string() },
//! ];
//! let props = TabsProps {
//!     tabs,
//!     default_tab: Some("tab1".to_string()),
//!     ..Default::default()
//! };
//! ```

use domius_core::signal::{signal, Signal};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, MouseEvent};

use crate::component::DomiusNode;

/// A single tab definition.
#[derive(Clone)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub disabled: bool,
    pub icon: Option<String>,
}

/// Props for the Tabs component.
pub struct TabsProps {
    /// List of tabs to display
    pub tabs: Vec<Tab>,
    /// Initially selected tab ID
    pub default_tab: Option<String>,
    /// Tab orientation
    pub orientation: TabOrientation,
    /// Whether tabs can be closed
    pub closable: bool,
    /// Callback when tab changes
    pub on_change: Option<Box<dyn Fn(String)>>,
    /// Callback when tab is closed
    pub on_close: Option<Box<dyn Fn(String)>>,
    /// Additional CSS classes
    pub class: Option<String>,
}

/// Tab orientation.
#[derive(Clone, PartialEq, Debug)]
pub enum TabOrientation {
    Horizontal,
    Vertical,
}

impl Default for TabOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl Default for TabsProps {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            default_tab: None,
            orientation: TabOrientation::default(),
            closable: false,
            on_change: None,
            on_close: None,
            class: None,
        }
    }
}

/// Internal state for the Tabs component.
pub struct TabsState {
    pub active_tab: Signal<String>,
    pub hover_tab: Signal<Option<String>>,
}

/// Tabs component.
pub struct Tabs;

impl Tabs {
    /// Create a tabs element.
    pub fn create(props: TabsProps) -> (Element, Signal<String>) {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        // Determine initial active tab
        let initial_active = props
            .default_tab
            .clone()
            .or_else(|| props.tabs.first().map(|t| t.id.clone()))
            .unwrap_or_else(|| String::new());

        let active_signal = signal(initial_active.clone());

        // Create container
        let container: HtmlElement = document.create_element("div").unwrap().dyn_into().unwrap();

        // Build class names
        let mut classes = vec!["domius-tabs".to_string()];
        classes.push(format!("domius-tabs-{:?}", props.orientation).to_lowercase());
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }
        container
            .set_attribute("class", &classes.join(" "))
            .unwrap();

        // Create tab list
        let tab_list: HtmlElement = document.create_element("div").unwrap().dyn_into().unwrap();
        tab_list.set_attribute("class", "domius-tab-list").unwrap();
        tab_list.set_attribute("role", "tablist").unwrap();

        // Create tabs
        for tab in &props.tabs {
            let tab_el: HtmlElement = document
                .create_element("button")
                .unwrap()
                .dyn_into()
                .unwrap();

            tab_el.set_attribute("class", "domius-tab").unwrap();
            tab_el.set_attribute("role", "tab");
            tab_el.set_attribute("aria-selected", "false");
            tab_el.set_attribute("data-tab-id", &tab.id);

            if tab.disabled {
                tab_el.set_attribute("disabled", "true").unwrap();
                tab_el.set_attribute("aria-disabled", "true").unwrap();
            }

            if tab.id == initial_active {
                tab_el.set_attribute("aria-selected", "true").unwrap();
                tab_el.set_attribute("data-active", "true").unwrap();
            }

            // Tab content
            let tab_content = if let Some(icon) = &tab.icon {
                format!("{} {}", icon, tab.label)
            } else {
                tab.label.clone()
            };
            tab_el.set_text_content(Some(&tab_content));

            // Click handler
            if !tab.disabled {
                let active_clone = active_signal.clone();
                let tab_id = tab.id.clone();
                let on_change_clone = props.on_change.as_ref().map(|_| {
                    let handler = props.on_change.as_ref().unwrap();
                    let tab_id_clone = tab.id.clone();
                    Closure::wrap(Box::new(move |_event: MouseEvent| {
                        handler(tab_id_clone.clone());
                    }) as Box<dyn FnMut(MouseEvent)>)
                });

                let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                    active_clone.set(tab_id.clone());
                }) as Box<dyn FnMut(MouseEvent)>);

                tab_el
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();

                if let Some(change_closure) = on_change_clone {
                    tab_el
                        .add_event_listener_with_callback(
                            "click",
                            change_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    change_closure.forget();
                }
            }

            tab_list.append_child(&tab_el).unwrap();
        }

        container.append_child(&tab_list).unwrap();

        (container.into(), active_signal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_orientation_default() {
        assert_eq!(TabOrientation::default(), TabOrientation::Horizontal);
    }

    #[test]
    fn test_tabs_props_default() {
        let props = TabsProps::default();
        assert!(props.tabs.is_empty());
        assert!(props.default_tab.is_none());
        assert!(!props.closable);
    }
}
