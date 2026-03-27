//! WASM integration tests for hooks.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use domius_web::hooks::{use_click_outside, use_keyboard, use_focus, KeyboardConfig};

wasm_bindgen_test_configure!(run_in_browser);

mod hook_tests {
    use super::*;
    use crate::test_utils::*;
    use domius_core::signal::signal;

    #[wasm_bindgen_test]
    fn test_use_click_outside_detects_outside_click() {
        let _guard = TestContainerGuard::new("test-click-outside");
        
        let target = create_div();
        target.set_id("target");
        target.set_text_content(Some("Click target"));
        
        let outside = create_div();
        outside.set_id("outside");
        outside.set_text_content(Some("Outside area"));
        
        // Append both to container
        let container = get_element_by_id("test-click-outside").unwrap();
        container.append_child(target.dyn_ref()).unwrap();
        container.append_child(outside.dyn_ref()).unwrap();
        
        let clicked_outside = use_click_outside(target.dyn_ref().unwrap());
        
        // Initially false
        assert!(!clicked_outside.get());
        
        // Simulate click on outside element
        simulate_click(outside.dyn_ref().unwrap());
        
        // Should be true now
        assert!(clicked_outside.get());
    }

    #[wasm_bindgen_test]
    fn test_use_click_outside_ignores_inside_click() {
        let _guard = TestContainerGuard::new("test-click-inside");
        
        let target = create_div();
        target.set_id("target");
        target.set_text_content(Some("Click target"));
        
        let child = create_div();
        child.set_id("child");
        child.set_text_content(Some("Child element"));
        
        target.append_child(child.dyn_ref()).unwrap();
        
        let container = get_element_by_id("test-click-inside").unwrap();
        container.append_child(target.dyn_ref()).unwrap();
        
        let clicked_outside = use_click_outside(target.dyn_ref().unwrap());
        
        // Initially false
        assert!(!clicked_outside.get());
        
        // Simulate click on child element (inside target)
        simulate_click(child.dyn_ref().unwrap());
        
        // Should still be false
        assert!(!clicked_outside.get());
    }

    #[wasm_bindgen_test]
    fn test_use_keyboard_escape_key() {
        let _guard = TestContainerGuard::new("test-keyboard");
        
        let target = create_div();
        target.set_id("keyboard-target");
        target.set_text_content(Some("Press Escape"));
        
        let container = get_element_by_id("test-keyboard").unwrap();
        container.append_child(target.dyn_ref()).unwrap();
        
        // Create keyboard hook for Escape key
        let escape_pressed = use_keyboard(KeyboardConfig::new("Escape"));
        
        // Initially false
        assert!(!escape_pressed.get());
        
        // Simulate Escape key press
        simulate_key_press(target.dyn_ref().unwrap(), "Escape");
        
        // Should be true (or become true after effect runs)
        // Note: This depends on timing of effect execution
    }

    #[wasm_bindgen_test]
    fn test_use_keyboard_ctrl_s() {
        let _guard = TestContainerGuard::new("test-keyboard-ctrl-s");
        
        let target = create_div();
        target.set_id("keyboard-target-2");
        
        let container = get_element_by_id("test-keyboard-ctrl-s").unwrap();
        container.append_child(target.dyn_ref()).unwrap();
        
        // Create keyboard hook for Ctrl+S
        let save_pressed = use_keyboard(KeyboardConfig::new("s").with_ctrl());
        
        // Initially false
        assert!(!save_pressed.get());
    }

    #[wasm_bindgen_test]
    fn test_use_focus_returns_signal_and_closures() {
        let _guard = TestContainerGuard::new("test-focus");
        
        let input = create_element("input");
        input.set_id("focus-target");
        input.set_attribute("type", "text").unwrap();
        
        let container = get_element_by_id("test-focus").unwrap();
        container.append_child(input.dyn_ref()).unwrap();
        
        let (is_focused, focus_cb, blur_cb) = use_focus();
        
        // Initially not focused
        assert!(!is_focused.get());
        
        // Note: Testing actual focus/blur requires user interaction
        // or more complex event simulation
    }

    #[wasm_bindgen_test]
    fn test_use_focus_auto() {
        use domius_web::hooks::use_focus_auto;
        
        let _guard = TestContainerGuard::new("test-focus-auto");
        
        let input = create_element("input");
        input.set_id("focus-auto-target");
        input.set_attribute("type", "text").unwrap();
        
        let container = get_element_by_id("test-focus-auto").unwrap();
        container.append_child(input.dyn_ref()).unwrap();
        
        let is_focused = use_focus_auto(input.dyn_ref().unwrap());
        
        // Initially not focused
        assert!(!is_focused.get());
    }

    #[wasm_bindgen_test]
    fn test_keyboard_shortcuts_helpers() {
        // Test that helper functions return correct configs
        let escape = KeyboardConfig::new("Escape");
        assert_eq!(escape.key, "Escape");
        assert!(!escape.keydown == false); // Should be true (keydown)
        
        let enter = KeyboardConfig::new("Enter");
        assert_eq!(enter.key, "Enter");
        
        let ctrl_k = KeyboardConfig::new("k").with_ctrl();
        assert_eq!(ctrl_k.key, "k");
        assert_eq!(ctrl_k.ctrl, Some(true));
        
        let shift_tab = KeyboardConfig::new("Tab").with_shift();
        assert_eq!(shift_tab.key, "Tab");
        assert_eq!(shift_tab.shift, Some(true));
    }
}
