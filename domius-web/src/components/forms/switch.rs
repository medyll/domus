//! Switch/Toggle component - Binary on/off control.

use domius_core::signal::{signal, Signal};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, MouseEvent};

/// Props for the Switch component.
pub struct SwitchProps {
    pub checked: Signal<bool>,
    pub label: Option<String>,
    pub disabled: bool,
    pub size: SwitchSize,
    pub on_change: Option<Box<dyn Fn(bool)>>,
    pub class: Option<String>,
}

/// Switch size.
#[derive(Clone, PartialEq, Debug)]
pub enum SwitchSize {
    Sm,
    Md,
    Lg,
}

impl Default for SwitchSize {
    fn default() -> Self {
        Self::Md
    }
}

impl Default for SwitchProps {
    fn default() -> Self {
        Self {
            checked: signal(false),
            label: None,
            disabled: false,
            size: SwitchSize::default(),
            on_change: None,
            class: None,
        }
    }
}

/// Switch component.
pub struct Switch;

impl Switch {
    /// Create a switch element.
    pub fn create(props: SwitchProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let container: HtmlElement = document
            .create_element("label")
            .unwrap()
            .dyn_into()
            .unwrap();

        // Build class names
        let mut classes = vec!["domius-switch".to_string()];
        classes.push(format!("domius-switch-{:?}", props.size).to_lowercase());
        if props.disabled {
            classes.push("domius-switch-disabled".to_string());
        }
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }
        container
            .set_attribute("class", &classes.join(" "))
            .unwrap();

        // Create hidden checkbox input
        let input: HtmlElement = document
            .create_element("input")
            .unwrap()
            .dyn_into()
            .unwrap();
        input.set_attribute("type", "checkbox").unwrap();
        input.set_attribute("class", "domius-switch-input").unwrap();

        if props.checked.get() {
            input.set_attribute("checked", "true").unwrap();
        }
        if props.disabled {
            input.set_attribute("disabled", "true").unwrap();
        }

        // Create slider visual
        let slider: HtmlElement = document.create_element("span").unwrap().dyn_into().unwrap();
        slider.set_attribute("class", "domius-slider").unwrap();

        container.append_child(&input).unwrap();
        container.append_child(&slider).unwrap();

        // Add label if provided
        if let Some(label) = &props.label {
            let label_el: HtmlElement =
                document.create_element("span").unwrap().dyn_into().unwrap();
            label_el
                .set_attribute("class", "domius-switch-label")
                .unwrap();
            label_el.set_text_content(Some(label));
            container.append_child(&label_el).unwrap();
        }

        // Click handler
        if !props.disabled {
            let checked_clone = props.checked.clone();
            let on_change_clone = props.on_change.map(|handler| {
                let checked_inner = checked_clone.clone();
                Closure::wrap(Box::new(move |_event: MouseEvent| {
                    let new_val = !checked_inner.get();
                    handler(new_val);
                }) as Box<dyn FnMut(MouseEvent)>)
            });

            let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                checked_clone.set(!checked_clone.get());
            }) as Box<dyn FnMut(MouseEvent)>);

            container
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();

            if let Some(change_closure) = on_change_clone {
                container
                    .add_event_listener_with_callback(
                        "click",
                        change_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                change_closure.forget();
            }
        }

        container.into()
    }
}
