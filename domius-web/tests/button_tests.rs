//! WASM integration tests for Button component.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use domius_web::components::{Button, ButtonProps, ButtonVariant, ButtonSize};

wasm_bindgen_test_configure!(run_in_browser);

mod button_tests {
    use super::*;
    use crate::test_utils::*;

    #[wasm_bindgen_test]
    fn test_button_creates_element() {
        let _guard = TestContainerGuard::new("test-button-container");
        
        let props = ButtonProps {
            children: "Click me".to_string(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            disabled: false,
            loading: false,
            loading_text: None,
            full_width: false,
            left_icon: None,
            right_icon: None,
            on_click: None,
            class: None,
            button_type: domius_web::components::ButtonType::Button,
        };
        
        let (element, _) = Button::create(props);
        
        assert!(element.dyn_ref::<web_sys::HtmlButtonElement>().is_some());
    }

    #[wasm_bindgen_test]
    fn test_button_has_correct_class() {
        let _guard = TestContainerGuard::new("test-button-class");
        
        let props = ButtonProps {
            children: "Test".to_string(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            disabled: false,
            loading: false,
            loading_text: None,
            full_width: false,
            left_icon: None,
            right_icon: None,
            on_click: None,
            class: Some("custom-class".to_string()),
            button_type: domius_web::components::ButtonType::Button,
        };
        
        let (element, _) = Button::create(props);
        
        assert!(has_class(element.dyn_ref().unwrap(), "domius-btn"));
        assert!(has_class(element.dyn_ref().unwrap(), "domius-btn-primary"));
        assert!(has_class(element.dyn_ref().unwrap(), "custom-class"));
    }

    #[wasm_bindgen_test]
    fn test_button_text_content() {
        let _guard = TestContainerGuard::new("test-button-text");
        
        let props = ButtonProps {
            children: "Submit Form".to_string(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            disabled: false,
            loading: false,
            loading_text: None,
            full_width: false,
            left_icon: None,
            right_icon: None,
            on_click: None,
            class: None,
            button_type: domius_web::components::ButtonType::Button,
        };
        
        let (element, _) = Button::create(props);
        
        assert_eq!(get_text_content(element.dyn_ref().unwrap()), "Submit Form");
    }

    #[wasm_bindgen_test]
    fn test_button_disabled() {
        let _guard = TestContainerGuard::new("test-button-disabled");
        
        let props = ButtonProps {
            children: "Disabled".to_string(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            disabled: true,
            loading: false,
            loading_text: None,
            full_width: false,
            left_icon: None,
            right_icon: None,
            on_click: None,
            class: None,
            button_type: domius_web::components::ButtonType::Button,
        };
        
        let (element, _) = Button::create(props);
        let button: web_sys::HtmlButtonElement = element.dyn_into().unwrap();
        
        assert!(button.disabled());
    }

    #[wasm_bindgen_test]
    fn test_button_click_handler() {
        use std::cell::RefCell;
        use std::rc::Rc;
        
        let _guard = TestContainerGuard::new("test-button-click");
        
        let clicked = Rc::new(RefCell::new(false));
        let clicked_clone = Rc::clone(&clicked);
        
        let props = ButtonProps {
            children: "Click me".to_string(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            disabled: false,
            loading: false,
            loading_text: None,
            full_width: false,
            left_icon: None,
            right_icon: None,
            on_click: Some(Box::new(move || {
                *clicked_clone.borrow_mut() = true;
            })),
            class: None,
            button_type: domius_web::components::ButtonType::Button,
        };
        
        let (element, _) = Button::create(props);
        
        // Simulate click
        simulate_click(element.dyn_ref().unwrap());
        
        // Check that click handler was called
        assert!(*clicked.borrow());
    }

    #[wasm_bindgen_test]
    fn test_button_variants() {
        let _guard = TestContainerGuard::new("test-button-variants");
        
        let variants = [
            ButtonVariant::Primary,
            ButtonVariant::Secondary,
            ButtonVariant::Text,
            ButtonVariant::Danger,
            ButtonVariant::Ghost,
        ];
        
        let variant_classes = [
            "domius-btn-primary",
            "domius-btn-secondary",
            "domius-btn-text",
            "domius-btn-danger",
            "domius-btn-ghost",
        ];
        
        for (variant, expected_class) in variants.iter().zip(variant_classes.iter()) {
            let props = ButtonProps {
                children: "Test".to_string(),
                variant: variant.clone(),
                size: ButtonSize::Md,
                disabled: false,
                loading: false,
                loading_text: None,
                full_width: false,
                left_icon: None,
                right_icon: None,
                on_click: None,
                class: None,
                button_type: domius_web::components::ButtonType::Button,
            };
            
            let (element, _) = Button::create(props);
            
            assert!(has_class(element.dyn_ref().unwrap(), expected_class));
        }
    }

    #[wasm_bindgen_test]
    fn test_button_sizes() {
        let _guard = TestContainerGuard::new("test-button-sizes");
        
        let sizes = [
            ButtonSize::Sm,
            ButtonSize::Md,
            ButtonSize::Lg,
        ];
        
        let size_classes = [
            "domius-btn-sm",
            "domius-btn-md",
            "domius-btn-lg",
        ];
        
        for (size, expected_class) in sizes.iter().zip(size_classes.iter()) {
            let props = ButtonProps {
                children: "Test".to_string(),
                variant: ButtonVariant::Primary,
                size: size.clone(),
                disabled: false,
                loading: false,
                loading_text: None,
                full_width: false,
                left_icon: None,
                right_icon: None,
                on_click: None,
                class: None,
                button_type: domius_web::components::ButtonType::Button,
            };
            
            let (element, _) = Button::create(props);
            
            assert!(has_class(element.dyn_ref().unwrap(), expected_class));
        }
    }
}
