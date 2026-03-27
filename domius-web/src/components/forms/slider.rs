//! Slider component - Range input for numeric selection.

use domius_core::signal::{signal, Signal};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, InputEvent};

/// Props for the Slider component.
pub struct SliderProps {
    pub value: Signal<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    pub show_marks: bool,
    pub marks: Vec<f64>,
    pub show_value: bool,
    pub orientation: SliderOrientation,
    pub on_change: Option<Box<dyn Fn(f64)>>,
    pub class: Option<String>,
}

/// Slider orientation.
#[derive(Clone, PartialEq, Debug)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

impl Default for SliderOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl Default for SliderProps {
    fn default() -> Self {
        Self {
            value: signal(0.0),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            show_marks: false,
            marks: Vec::new(),
            show_value: false,
            orientation: SliderOrientation::default(),
            on_change: None,
            class: None,
        }
    }
}

/// Slider component.
pub struct Slider;

impl Slider {
    /// Create a slider element.
    pub fn create(props: SliderProps) -> (Element, Signal<f64>) {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let container: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();

        // Build class names
        let mut classes = vec!["domius-slider".to_string()];
        classes.push(format!("domius-slider-{:?}", props.orientation).to_lowercase());
        if props.disabled {
            classes.push("domius-slider-disabled".to_string());
        }
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }
        container.set_attribute("class", &classes.join(" ")).unwrap();

        // Create range input
        let input: HtmlElement = document
            .create_element("input")
            .unwrap()
            .dyn_into()
            .unwrap();
        input.set_attribute("type", "range").unwrap();
        input.set_attribute("class", "domius-slider-input").unwrap();
        input.set_attribute("min", &props.min.to_string()).unwrap();
        input.set_attribute("max", &props.max.to_string()).unwrap();
        input.set_attribute("step", &props.step.to_string()).unwrap();
        
        if props.disabled {
            input.set_attribute("disabled", "true").unwrap();
        }

        // Set initial value
        let value_str = props.value.get().to_string();
        input.set_attribute("value", &value_str).unwrap();

        container.append_child(&input).unwrap();

        // Add value display if requested
        if props.show_value {
            let value_display: HtmlElement = document
                .create_element("span")
                .unwrap()
                .dyn_into()
                .unwrap();
            value_display.set_attribute("class", "domius-slider-value").unwrap();
            value_display.set_text_content(Some(&value_str));
            container.append_child(&value_display).unwrap();
        }

        // Input handler
        let value_clone = props.value.clone();
        let on_change_clone = props.on_change.map(|handler| {
            let value_inner = value_clone.clone();
            Closure::wrap(Box::new(move |event: InputEvent| {
                let target = event.target().unwrap();
                let input_el: HtmlElement = target.dyn_into().unwrap();
                let val: f64 = input_el.get_attribute("value")
                    .unwrap_or_else(|| "0".to_string())
                    .parse()
                    .unwrap_or(0.0);
                handler(val);
            }) as Box<dyn FnMut(InputEvent)>)
        });

        let closure = Closure::wrap(Box::new(move |event: InputEvent| {
            let target = event.target().unwrap();
            let input_el: HtmlElement = target.dyn_into().unwrap();
            let val: f64 = input_el.get_attribute("value")
                .unwrap_or_else(|| "0".to_string())
                .parse()
                .unwrap_or(0.0);
            value_clone.set(val);
        }) as Box<dyn FnMut(InputEvent)>);

        input.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        if let Some(change_closure) = on_change_clone {
            input.add_event_listener_with_callback("input", change_closure.as_ref().unchecked_ref())
                .unwrap();
            change_closure.forget();
        }

        (container.into(), props.value)
    }
}
