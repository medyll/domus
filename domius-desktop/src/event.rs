//! Event handling for Tauri desktop applications.
//!
//! Bridges Rust events to Tauri commands and vice versa.

use domius_core::signal::Signal;
use serde::{Deserialize, Serialize};

/// Desktop event types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DesktopEvent {
    /// Window closed
    WindowClose { scope_id: usize },
    /// Window focused
    WindowFocus,
    /// Window blurred
    WindowBlur,
    /// Custom event with payload
    Custom { name: String, payload: String },
}

/// Event handler type.
pub type EventHandler = Box<dyn Fn(DesktopEvent) + Send + 'static>;

/// Event bridge for communicating between Rust and Tauri.
pub struct EventBridge {
    handlers: Vec<EventHandler>,
}

impl EventBridge {
    /// Create a new event bridge.
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Register an event handler.
    pub fn on<F: Fn(DesktopEvent) + Send + 'static>(&mut self, handler: F) {
        self.handlers.push(Box::new(handler));
    }

    /// Emit an event to all handlers.
    pub fn emit(&self, event: DesktopEvent) {
        for handler in &self.handlers {
            handler(event.clone());
        }
    }
}

impl Default for EventBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a signal-based event listener.
///
/// Returns a signal that updates when the event fires.
pub fn use_event_signal<T: Clone + 'static>(
    _event_name: &str,
    initial: T,
) -> Signal<T> {
    // TODO: Implement proper event subscription
    // For now, return a simple signal
    signal(initial)
}

fn signal<T: Clone + 'static>(value: T) -> Signal<T> {
    domius_core::signal::signal(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn event_bridge_emit_and_handle() {
        let mut bridge = EventBridge::new();
        let received = Arc::new(Mutex::new(Vec::new()));

        let received_clone = Arc::clone(&received);
        bridge.on(move |event| {
            received_clone
                .lock()
                .expect("received event lock should not be poisoned")
                .push(format!("{:?}", event));
        });

        bridge.emit(DesktopEvent::WindowFocus);
        bridge.emit(DesktopEvent::WindowBlur);

        assert_eq!(
            received
                .lock()
                .expect("received event lock should not be poisoned")
                .len(),
            2
        );
    }

    #[test]
    fn event_bridge_custom_event() {
        let mut bridge = EventBridge::new();
        let received = Arc::new(Mutex::new(None));

        let received_clone = Arc::clone(&received);
        bridge.on(move |event| {
            if let DesktopEvent::Custom { name, payload } = event {
                *received_clone
                    .lock()
                    .expect("received event lock should not be poisoned") = Some((name, payload));
            }
        });

        bridge.emit(DesktopEvent::Custom {
            name: "test".into(),
            payload: "data".into(),
        });

        assert_eq!(
            *received
                .lock()
                .expect("received event lock should not be poisoned"),
            Some(("test".into(), "data".into()))
        );
    }
}
