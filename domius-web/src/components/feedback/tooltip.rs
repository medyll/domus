//! Tooltip component - Hover information popup.

use domius_core::signal::{signal, Signal};
use domius_core::effect::create_effect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, MouseEvent};

/// Tooltip position.
#[derive(Clone, PartialEq, Debug)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    BottomStart,
    BottomEnd,
}

impl Default for TooltipPosition {
    fn default() -> Self {
        Self::Top
    }
}

/// Props for the Tooltip component.
#[derive(Clone)]
pub struct TooltipProps {
    pub content: String,
    pub position: TooltipPosition,
    pub delay: u64,
    pub disabled: bool,
    pub children: Element,
    pub class: Option<String>,
}

impl Default for TooltipProps {
    fn default() -> Self {
        Self {
            content: String::new(),
            position: TooltipPosition::default(),
            delay: 200,
            disabled: false,
            children: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("span")
                .unwrap()
                .into(),
            class: None,
        }
    }
}

/// Tooltip component.
pub struct Tooltip;

impl Tooltip {
    /// Create a tooltip wrapper element.
    pub fn create(props: TooltipProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        // Wrapper container
        let wrapper: HtmlElement = document
            .create_element("span")
            .unwrap()
            .dyn_into()
            .unwrap();
        wrapper.set_attribute("class", "domius-tooltip-wrapper").unwrap();

        // Append children
        wrapper.append_child(&props.children).unwrap();

        if props.disabled {
            return wrapper.into();
        }

        // Create tooltip element (hidden by default)
        let tooltip: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();
        
        let mut classes = vec![
            "domius-tooltip".to_string(),
            format!("domius-tooltip-{:?}", props.position).to_lowercase(),
        ];
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }
        tooltip.set_attribute("class", &classes.join(" ")).unwrap();
        tooltip.set_attribute("role", "tooltip").unwrap();
        tooltip.set_text_content(Some(&props.content));
        tooltip.set_attribute("aria-hidden", "true").unwrap();

        // State
        let is_visible = signal(false);

        // Show handler
        let wrapper_show = wrapper.clone();
        let tooltip_show = tooltip.clone();
        let is_visible_show = is_visible.clone();
        let delay_show = props.delay;
        let show_closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
            let is_visible_clone = is_visible_show.clone();
            let tooltip_clone = tooltip_show.clone();
            
            if delay_show > 0 {
                let timeout_closure = Closure::once(move || {
                    is_visible_clone.set(true);
                    tooltip_clone.set_attribute("aria-hidden", "false").ok();
                });
                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        timeout_closure.as_ref().unchecked_ref(),
                        delay_show as i32,
                    );
                    timeout_closure.forget();
                }
            } else {
                is_visible_clone.set(true);
                tooltip_clone.set_attribute("aria-hidden", "false").ok();
            }
        }) as Box<dyn FnMut(MouseEvent)>);

        wrapper.add_event_listener_with_callback("mouseenter", show_closure.as_ref().unchecked_ref())
            .unwrap();
        show_closure.forget();

        // Hide handler
        let is_visible_hide = is_visible.clone();
        let tooltip_hide = tooltip.clone();
        let hide_closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
            is_visible_hide.set(false);
            tooltip_hide.set_attribute("aria-hidden", "true").ok();
        }) as Box<dyn FnMut(MouseEvent)>);

        wrapper.add_event_listener_with_callback("mouseleave", hide_closure.as_ref().unchecked_ref())
            .unwrap();
        hide_closure.forget();

        wrapper.append_child(&tooltip).unwrap();

        wrapper.into()
    }
}
