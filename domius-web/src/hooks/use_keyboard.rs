//! Hook to listen for keyboard events.
//!
//! Useful for shortcuts, escape key handling, and keyboard navigation.

use domius_core::effect::create_effect;
use domius_core::signal::Signal;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

/// Configuration for keyboard hook.
#[derive(Clone)]
pub struct KeyboardConfig {
    /// Key to listen for (e.g., "Escape", "Enter", "k")
    pub key: String,
    /// Whether to listen for keydown (true) or keyup (false)
    pub keydown: bool,
    /// Whether Ctrl/Cmd must be pressed
    pub ctrl: Option<bool>,
    /// Whether Shift must be pressed
    pub shift: Option<bool>,
    /// Whether Alt must be pressed
    pub alt: Option<bool>,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self {
            key: String::new(),
            keydown: true,
            ctrl: None,
            shift: None,
            alt: None,
        }
    }
}

impl KeyboardConfig {
    /// Create a config for a single key press.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            ..Default::default()
        }
    }

    /// Require Ctrl/Cmd to be pressed.
    pub fn with_ctrl(mut self) -> Self {
        self.ctrl = Some(true);
        self
    }

    /// Require Shift to be pressed.
    pub fn with_shift(mut self) -> Self {
        self.shift = Some(true);
        self
    }

    /// Require Alt to be pressed.
    pub fn with_alt(mut self) -> Self {
        self.alt = Some(true);
        self
    }

    /// Listen for keyup instead of keydown.
    pub fn on_keyup(mut self) -> Self {
        self.keydown = false;
        self
    }

    /// Check if a keyboard event matches this config.
    pub fn matches(&self, event: &KeyboardEvent) -> bool {
        // Check key
        if !self.key.is_empty() && event.key() != self.key {
            return false;
        }

        // Check modifiers
        if let Some(ctrl) = self.ctrl {
            if event.ctrl_key() != ctrl {
                return false;
            }
        }
        if let Some(shift) = self.shift {
            if event.shift_key() != shift {
                return false;
            }
        }
        if let Some(alt) = self.alt {
            if event.alt_key() != alt {
                return false;
            }
        }

        true
    }
}

/// Returns a signal that is true when the configured key combination is pressed.
///
/// # Example
/// ```ignore
/// let escape_pressed = use_keyboard(KeyboardConfig::new("Escape"));
/// create_effect(move || {
///     if escape_pressed.get() {
///         // Close modal
///     }
/// });
/// ```
pub fn use_keyboard(config: KeyboardConfig) -> Signal<bool> {
    use domius_core::signal::signal;

    let pressed = signal(false);
    let config_clone = config.clone();
    let pressed_clone = pressed.clone();

    let event_name = if config.keydown { "keydown" } else { "keyup" };

    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        if config_clone.matches(&event) {
            pressed_clone.set(true);
            // Reset after a short delay for keydown
            if config_clone.keydown {
                let pressed_reset = pressed_clone.clone();
                let timeout_closure = Closure::once(move || {
                    pressed_reset.set(false);
                });
                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        timeout_closure.as_ref().unchecked_ref(),
                        100,
                    );
                    timeout_closure.forget();
                }
            }
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);

    if let Some(window) = web_sys::window() {
        window
            .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
            .expect("Failed to add keyboard listener");
        closure.forget();
    }

    pressed
}

/// Register a keyboard shortcut that triggers a callback.
///
/// # Example
/// ```ignore
/// use_keyboard_with_callback(KeyboardConfig::new("Escape").with_ctrl(), || {
///     // Save document
/// });
/// ```
pub fn use_keyboard_with_callback<F>(config: KeyboardConfig, callback: F)
where
    F: Fn() + 'static,
{
    let pressed = use_keyboard(config.clone());

    create_effect(move || {
        if pressed.get() {
            callback();
        }
    });
}

/// Common keyboard shortcuts.
pub mod shortcuts {
    use super::*;

    /// Escape key config.
    pub fn escape() -> KeyboardConfig {
        KeyboardConfig::new("Escape")
    }

    /// Enter key config.
    pub fn enter() -> KeyboardConfig {
        KeyboardConfig::new("Enter")
    }

    /// Tab key config.
    pub fn tab() -> KeyboardConfig {
        KeyboardConfig::new("Tab")
    }

    /// Ctrl+S (save) config.
    pub fn save() -> KeyboardConfig {
        KeyboardConfig::new("s").with_ctrl()
    }

    /// Ctrl+Z (undo) config.
    pub fn undo() -> KeyboardConfig {
        KeyboardConfig::new("z").with_ctrl()
    }

    /// Ctrl+Y (redo) config.
    pub fn redo() -> KeyboardConfig {
        KeyboardConfig::new("y").with_ctrl()
    }

    /// Ctrl+K (command palette) config.
    pub fn command_palette() -> KeyboardConfig {
        KeyboardConfig::new("k").with_ctrl()
    }
}
