//! Input component - Text input field with various options.

use domius_core::signal::{signal, Signal};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement, InputEvent, FocusEvent, KeyboardEvent};

use crate::component::DomiusNode;
use crate::hooks::use_focus;

/// Input type attribute.
#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DatetimeLocal,
    Month,
    Week,
    Color,
    File,
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DatetimeLocal => "datetime-local",
            InputType::Month => "month",
            InputType::Week => "week",
            InputType::Color => "color",
            InputType::File => "file",
        }
    }
}

/// Props for the Input component.
pub struct InputProps {
    /// Input type
    pub input_type: InputType,
    /// Current value (for controlled inputs)
    pub value: Option<Signal<String>>,
    /// Placeholder text
    pub placeholder: Option<String>,
    /// Label text
    pub label: Option<String>,
    /// Helper text below input
    pub helper_text: Option<String>,
    /// Error message (shows error state)
    pub error: Option<String>,
    /// Whether input is disabled
    pub disabled: bool,
    /// Whether input is read-only
    pub read_only: bool,
    /// Whether input is required
    pub required: bool,
    /// Minimum value (for number/date types)
    pub min: Option<String>,
    /// Maximum value (for number/date types)
    pub max: Option<String>,
    /// Step increment (for number types)
    pub step: Option<String>,
    /// Maximum length
    pub max_length: Option<u32>,
    /// Minimum length
    pub min_length: Option<u32>,
    /// Input pattern (regex for validation)
    pub pattern: Option<String>,
    /// Auto-complete hint
    pub auto_complete: Option<String>,
    /// Left addon (icon or text)
    pub left_addon: Option<String>,
    /// Right addon (icon or text)
    pub right_addon: Option<String>,
    /// Full width input
    pub full_width: bool,
    /// Size variant
    pub size: InputSize,
    /// Change handler (for uncontrolled inputs)
    pub on_change: Option<Box<dyn Fn(String)>>,
    /// Input handler (fires on every keystroke)
    pub on_input: Option<Box<dyn Fn(String)>>,
    /// Focus handler
    pub on_focus: Option<Box<dyn Fn()>>,
    /// Blur handler
    pub on_blur: Option<Box<dyn Fn()>>,
    /// Key down handler
    pub on_key_down: Option<Box<dyn Fn(KeyboardEvent)>>,
    /// Additional CSS classes
    pub class: Option<String>,
    /// Input ID
    pub id: Option<String>,
    /// Input name attribute
    pub name: Option<String>,
}

/// Input size.
#[derive(Clone, PartialEq, Debug)]
pub enum InputSize {
    Sm,
    Md,
    Lg,
}

impl Default for InputSize {
    fn default() -> Self {
        Self::Md
    }
}

impl Default for InputProps {
    fn default() -> Self {
        Self {
            input_type: InputType::default(),
            value: None,
            placeholder: None,
            label: None,
            helper_text: None,
            error: None,
            disabled: false,
            read_only: false,
            required: false,
            min: None,
            max: None,
            step: None,
            max_length: None,
            min_length: None,
            pattern: None,
            auto_complete: None,
            left_addon: None,
            right_addon: None,
            full_width: false,
            size: InputSize::default(),
            on_change: None,
            on_input: None,
            on_focus: None,
            on_blur: None,
            on_key_down: None,
            class: None,
            id: None,
            name: None,
        }
    }
}

/// Internal state for the Input component.
pub struct InputState {
    pub is_focused: Signal<bool>,
    pub has_value: Signal<bool>,
    pub internal_value: Signal<String>,
}

/// Input component.
pub struct Input;

impl Input {
    /// Create an input element with the given properties.
    ///
    /// Returns the element and optionally a signal for the current value.
    pub fn create(props: InputProps) -> (Element, Option<Signal<String>>) {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let input: HtmlInputElement = document
            .create_element("input")
            .unwrap()
            .dyn_into()
            .unwrap();

        // Set type
        input.set_attribute("type", props.input_type.as_str()).unwrap();

        // Set attributes
        if let Some(placeholder) = &props.placeholder {
            input.set_placeholder(placeholder);
        }
        if props.disabled {
            input.set_disabled(true);
        }
        if props.read_only {
            input.set_read_only(true);
        }
        if props.required {
            input.set_required(true);
        }
        if let Some(min) = &props.min {
            input.set_attribute("min", min).unwrap();
        }
        if let Some(max) = &props.max {
            input.set_attribute("max", max).unwrap();
        }
        if let Some(step) = &props.step {
            input.set_attribute("step", step).unwrap();
        }
        if let Some(max_length) = props.max_length {
            input.set_max_length(max_length as i32);
        }
        if let Some(min_length) = props.min_length {
            // min_length not directly supported on input, would need custom validation
        }
        if let Some(pattern) = &props.pattern {
            input.set_attribute("pattern", pattern).unwrap();
        }
        if let Some(auto_complete) = &props.auto_complete {
            input.set_attribute("autocomplete", auto_complete).unwrap();
        }
        if let Some(id) = &props.id {
            input.set_id(id);
        }
        if let Some(name) = &props.name {
            input.set_attribute("name", name).unwrap();
        }

        // Set initial value if provided
        let value_signal = if let Some(value) = props.value {
            // Controlled input
            let value_clone = value.clone();
            input.set_value(&value_clone.get());
            Some(value)
        } else {
            // Uncontrolled input - create internal state
            let internal_value = signal(String::new());
            input.set_value(&internal_value.get());
            Some(internal_value.clone())
        };

        // Build class names
        let mut classes = vec!["domius-input".to_string()];
        classes.push(format!("domius-input-{:?}", props.size).to_lowercase());
        
        if props.error.is_some() {
            classes.push("domius-input-error".to_string());
        }
        if props.disabled {
            classes.push("domius-input-disabled".to_string());
        }
        if props.full_width {
            classes.push("domius-input-full-width".to_string());
        }
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }

        input.set_attribute("class", &classes.join(" ")).unwrap();

        // Attach event handlers
        let on_input_closure = props.on_input.map(|handler| {
            let closure = Closure::wrap(Box::new(move |event: InputEvent| {
                let target = event.target().unwrap();
                let input_el: HtmlInputElement = target.dyn_into().unwrap();
                handler(input_el.value());
            }) as Box<dyn FnMut(InputEvent)>);

            input
                .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())
                .unwrap();
            closure
        });

        let on_change_closure = props.on_change.map(|handler| {
            let closure = Closure::wrap(Box::new(move |event: InputEvent| {
                let target = event.target().unwrap();
                let input_el: HtmlInputElement = target.dyn_into().unwrap();
                handler(input_el.value());
            }) as Box<dyn FnMut(InputEvent)>);

            input
                .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                .unwrap();
            closure
        });

        if let Some(handler) = props.on_focus {
            let closure = Closure::wrap(Box::new(move |_event: FocusEvent| {
                handler();
            }) as Box<dyn FnMut(FocusEvent)>);
            input
                .add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }

        if let Some(handler) = props.on_blur {
            let closure = Closure::wrap(Box::new(move |_event: FocusEvent| {
                handler();
            }) as Box<dyn FnMut(FocusEvent)>);
            input
                .add_event_listener_with_callback("blur", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }

        if let Some(handler) = props.on_key_down {
            let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                handler(event);
            }) as Box<dyn FnMut(KeyboardEvent)>);
            input
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }

        // Forget closures that don't have explicit cleanup
        if let Some(c) = on_input_closure {
            c.forget();
        }
        if let Some(c) = on_change_closure {
            c.forget();
        }

        (input.into(), value_signal)
    }

    /// Create an input with a reactive signal for two-way binding.
    pub fn controlled(
        value: Signal<String>,
        input_type: InputType,
        placeholder: Option<&str>,
    ) -> Element {
        let props = InputProps {
            input_type,
            value: Some(value.clone()),
            placeholder: placeholder.map(String::from),
            ..Default::default()
        };

        let (element, _) = Self::create(props);

        // Sync DOM when signal changes
        let value_clone = value.clone();
        let element_clone = element.clone();
        domius_core::effect::create_effect(move || {
            if let Some(input_el) = element_clone.dyn_ref::<HtmlInputElement>() {
                input_el.set_value(&value_clone.get());
            }
        });

        element
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_type_as_str() {
        assert_eq!(InputType::Text.as_str(), "text");
        assert_eq!(InputType::Password.as_str(), "password");
        assert_eq!(InputType::Email.as_str(), "email");
    }

    #[test]
    fn test_input_size_default() {
        assert_eq!(InputSize::default(), InputSize::Md);
    }

    #[test]
    fn test_input_props_default() {
        let props = InputProps::default();
        assert_eq!(props.input_type, InputType::Text);
        assert!(!props.disabled);
        assert!(!props.required);
    }
}
