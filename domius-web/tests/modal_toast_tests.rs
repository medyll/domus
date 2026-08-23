//! WASM integration tests for Modal and Toast components.

#![cfg(target_arch = "wasm32")]

mod test_utils;

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use domius_core::signal::signal;

wasm_bindgen_test_configure!(run_in_browser);

mod modal_tests {
    use super::*;
    use crate::test_utils::*;
    use domius_web::components::{Modal, ModalProps, ModalSize};

    #[wasm_bindgen_test]
    fn test_modal_creates_backdrop() {
        let _guard = TestContainerGuard::new("test-modal");
        
        let open = signal(true);
        
        let props = ModalProps {
            open: open.clone(),
            title: Some("Test Modal".to_string()),
            content: "This is modal content".to_string(),
            closable: true,
            close_on_overlay: true,
            close_on_escape: true,
            size: ModalSize::Md,
            show_footer: false,
            confirm_text: None,
            cancel_text: None,
            on_close: None,
            on_confirm: None,
            class: None,
        };
        
        let element = Modal::create(props);
        
        // Modal should have backdrop class
        assert!(has_class(element.dyn_ref().unwrap(), "domius-modal-backdrop"));
    }

    #[wasm_bindgen_test]
    fn test_modal_has_title() {
        let _guard = TestContainerGuard::new("test-modal-title");
        
        let open = signal(true);
        
        let props = ModalProps {
            open: open.clone(),
            title: Some("Important Notice".to_string()),
            content: "Content here".to_string(),
            closable: true,
            close_on_overlay: true,
            close_on_escape: true,
            size: ModalSize::Md,
            show_footer: false,
            confirm_text: None,
            cancel_text: None,
            on_close: None,
            on_confirm: None,
            class: None,
        };
        
        let element = Modal::create(props);
        
        // Check that title text is present
        let text = get_text_content(element.dyn_ref().unwrap());
        assert!(text.contains("Important Notice"));
    }

    #[wasm_bindgen_test]
    fn test_modal_sizes() {
        let _guard = TestContainerGuard::new("test-modal-sizes");
        
        let sizes = [
            ModalSize::Sm,
            ModalSize::Md,
            ModalSize::Lg,
            ModalSize::Xl,
            ModalSize::Full,
        ];
        
        let size_classes = [
            "domius-modal-sm",
            "domius-modal-md",
            "domius-modal-lg",
            "domius-modal-xl",
            "domius-modal-full",
        ];
        
        for (size, expected_class) in sizes.iter().zip(size_classes.iter()) {
            let open = signal(true);
            
            let props = ModalProps {
                open: open.clone(),
                title: None,
                content: "Test".to_string(),
                closable: true,
                close_on_overlay: true,
                close_on_escape: true,
                size: size.clone(),
                show_footer: false,
                confirm_text: None,
                cancel_text: None,
                on_close: None,
                on_confirm: None,
                class: None,
            };
            
            let element = Modal::create(props);
            
            // Find the modal element inside backdrop
            let modal = element.query_selector(".domius-modal").ok().flatten().unwrap();
            assert!(has_class(modal.dyn_ref().unwrap(), expected_class));
        }
    }
}

mod toast_tests {
    use super::*;
    use domius_web::components::{ToastManager, ToastVariant, ToastData};

    #[wasm_bindgen_test]
    fn test_toast_manager_creation() {
        let manager = ToastManager::new();
        
        // Initially no toasts
        assert!(manager.toasts.get().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_toast_manager_add_toast() {
        let manager = ToastManager::new();
        
        manager.add(ToastData {
            id: "test-1".to_string(),
            message: "Test message".to_string(),
            title: Some("Test".to_string()),
            variant: ToastVariant::Info,
            duration: Some(5000),
            dismissible: true,
        });
        
        let toasts = manager.toasts.get();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].id, "test-1");
    }

    #[wasm_bindgen_test]
    fn test_toast_manager_remove_toast() {
        let manager = ToastManager::new();
        
        manager.add(ToastData {
            id: "test-1".to_string(),
            message: "Test message".to_string(),
            title: None,
            variant: ToastVariant::Info,
            duration: Some(5000),
            dismissible: true,
        });
        
        manager.add(ToastData {
            id: "test-2".to_string(),
            message: "Another message".to_string(),
            title: None,
            variant: ToastVariant::Success,
            duration: Some(3000),
            dismissible: true,
        });
        
        assert_eq!(manager.toasts.get().len(), 2);
        
        manager.remove("test-1");
        
        let toasts = manager.toasts.get();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].id, "test-2");
    }

    #[wasm_bindgen_test]
    fn test_toast_manager_info() {
        let manager = ToastManager::new();
        
        manager.info("Information message");
        
        let toasts = manager.toasts.get();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].variant, ToastVariant::Info);
        assert!(toasts[0].message.contains("Information"));
    }

    #[wasm_bindgen_test]
    fn test_toast_manager_success() {
        let manager = ToastManager::new();
        
        manager.success("Operation completed!");
        
        let toasts = manager.toasts.get();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].variant, ToastVariant::Success);
    }

    #[wasm_bindgen_test]
    fn test_toast_manager_error() {
        let manager = ToastManager::new();
        
        manager.error("Something went wrong");
        
        let toasts = manager.toasts.get();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].variant, ToastVariant::Error);
    }

    #[wasm_bindgen_test]
    fn test_toast_manager_multiple_toasts() {
        let manager = ToastManager::new();
        
        manager.info("Info");
        manager.success("Success");
        manager.warning("Warning");
        manager.error("Error");
        
        let toasts = manager.toasts.get();
        assert_eq!(toasts.len(), 4);
        
        let variants: Vec<_> = toasts.iter().map(|t| t.variant.clone()).collect();
        assert!(variants.contains(&ToastVariant::Info));
        assert!(variants.contains(&ToastVariant::Success));
        assert!(variants.contains(&ToastVariant::Warning));
        assert!(variants.contains(&ToastVariant::Error));
    }
}
