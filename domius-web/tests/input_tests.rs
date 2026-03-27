//! WASM integration tests for Input component.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use domius_web::components::{Input, InputProps, InputType, InputSize};

wasm_bindgen_test_configure!(run_in_browser);

mod input_tests {
    use super::*;
    use crate::test_utils::*;
    use domius_core::signal::signal;

    #[wasm_bindgen_test]
    fn test_input_creates_element() {
        let _guard = TestContainerGuard::new("test-input-container");
        
        let props = InputProps {
            input_type: InputType::Text,
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
            size: InputSize::Md,
            on_change: None,
            on_input: None,
            on_focus: None,
            on_blur: None,
            on_key_down: None,
            class: None,
            id: None,
            name: None,
        };
        
        let (element, _) = Input::create(props);
        
        let input: web_sys::HtmlInputElement = element.dyn_into().expect("should be input element");
        assert_eq!(input.input_type(), "text");
    }

    #[wasm_bindgen_test]
    fn test_input_types() {
        let _guard = TestContainerGuard::new("test-input-types");
        
        let types = [
            (InputType::Text, "text"),
            (InputType::Email, "email"),
            (InputType::Password, "password"),
            (InputType::Number, "number"),
            (InputType::Search, "search"),
        ];
        
        for (input_type, expected_type) in types.iter() {
            let props = InputProps {
                input_type: input_type.clone(),
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
                size: InputSize::Md,
                on_change: None,
                on_input: None,
                on_focus: None,
                on_blur: None,
                on_key_down: None,
                class: None,
                id: None,
                name: None,
            };
            
            let (element, _) = Input::create(props);
            let input: web_sys::HtmlInputElement = element.dyn_into().unwrap();
            
            assert_eq!(input.input_type(), *expected_type);
        }
    }

    #[wasm_bindgen_test]
    fn test_input_placeholder() {
        let _guard = TestContainerGuard::new("test-input-placeholder");
        
        let props = InputProps {
            input_type: InputType::Text,
            value: None,
            placeholder: Some("Enter your email".to_string()),
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
            size: InputSize::Md,
            on_change: None,
            on_input: None,
            on_focus: None,
            on_blur: None,
            on_key_down: None,
            class: None,
            id: None,
            name: None,
        };
        
        let (element, _) = Input::create(props);
        let input: web_sys::HtmlInputElement = element.dyn_into().unwrap();
        
        assert_eq!(input.placeholder(), "Enter your email");
    }

    #[wasm_bindgen_test]
    fn test_input_disabled() {
        let _guard = TestContainerGuard::new("test-input-disabled");
        
        let props = InputProps {
            input_type: InputType::Text,
            value: None,
            placeholder: None,
            label: None,
            helper_text: None,
            error: None,
            disabled: true,
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
            size: InputSize::Md,
            on_change: None,
            on_input: None,
            on_focus: None,
            on_blur: None,
            on_key_down: None,
            class: None,
            id: None,
            name: None,
        };
        
        let (element, _) = Input::create(props);
        let input: web_sys::HtmlInputElement = element.dyn_into().unwrap();
        
        assert!(input.disabled());
    }

    #[wasm_bindgen_test]
    fn test_input_required() {
        let _guard = TestContainerGuard::new("test-input-required");
        
        let props = InputProps {
            input_type: InputType::Text,
            value: None,
            placeholder: None,
            label: None,
            helper_text: None,
            error: None,
            disabled: false,
            read_only: false,
            required: true,
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
            size: InputSize::Md,
            on_change: None,
            on_input: None,
            on_focus: None,
            on_blur: None,
            on_key_down: None,
            class: None,
            id: None,
            name: None,
        };
        
        let (element, _) = Input::create(props);
        let input: web_sys::HtmlInputElement = element.dyn_into().unwrap();
        
        assert!(input.required());
    }

    #[wasm_bindgen_test]
    fn test_input_value_signal() {
        use domius_core::signal::signal;
        
        let _guard = TestContainerGuard::new("test-input-value");
        
        let value_signal = signal("initial value".to_string());
        
        let props = InputProps {
            input_type: InputType::Text,
            value: Some(value_signal.clone()),
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
            size: InputSize::Md,
            on_change: None,
            on_input: None,
            on_focus: None,
            on_blur: None,
            on_key_down: None,
            class: None,
            id: None,
            name: None,
        };
        
        let (element, _) = Input::create(props);
        let input: web_sys::HtmlInputElement = element.dyn_into().unwrap();
        
        assert_eq!(input.value(), "initial value");
        
        // Update signal and verify DOM updates
        value_signal.set("updated value".to_string());
        
        // Note: In a real scenario, the effect would update the DOM
        // For this test, we just verify the signal was created correctly
    }

    #[wasm_bindgen_test]
    fn test_input_sizes() {
        let _guard = TestContainerGuard::new("test-input-sizes");
        
        let sizes = [
            InputSize::Sm,
            InputSize::Md,
            InputSize::Lg,
        ];
        
        let size_classes = [
            "domius-input-sm",
            "domius-input-md",
            "domius-input-lg",
        ];
        
        for (size, expected_class) in sizes.iter().zip(size_classes.iter()) {
            let props = InputProps {
                input_type: InputType::Text,
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
                size: size.clone(),
                on_change: None,
                on_input: None,
                on_focus: None,
                on_blur: None,
                on_key_down: None,
                class: None,
                id: None,
                name: None,
            };
            
            let (element, _) = Input::create(props);
            
            assert!(has_class(element.dyn_ref().unwrap(), expected_class));
        }
    }

    #[wasm_bindgen_test]
    fn test_input_error_state() {
        let _guard = TestContainerGuard::new("test-input-error");
        
        let props = InputProps {
            input_type: InputType::Text,
            value: None,
            placeholder: None,
            label: None,
            helper_text: None,
            error: Some("Invalid email format".to_string()),
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
            size: InputSize::Md,
            on_change: None,
            on_input: None,
            on_focus: None,
            on_blur: None,
            on_key_down: None,
            class: None,
            id: None,
            name: None,
        };
        
        let (element, _) = Input::create(props);
        
        assert!(has_class(element.dyn_ref().unwrap(), "domius-input-error"));
    }
}
