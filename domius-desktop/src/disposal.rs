//! Automatic disposal via Tauri window events.
//!
//! Listens to window close events and disposes associated scopes.

use domius_core::{dispose_scope, ScopeId};

/// Initialize event listeners for automatic scope disposal.
///
/// This should be called once during app initialization.
pub fn init_event_listeners() {
    // TODO: Set up global window event listeners
    // For now, this is a placeholder that will be implemented
    // when integrating with actual Tauri windows
}

/// Dispose a scope when a window is closed.
///
/// This should be called from the window's close event handler.
pub fn on_window_close(scope_id: ScopeId) {
    dispose_scope(scope_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use domius_core::scope::create_scope;

    #[test]
    fn init_does_not_panic() {
        init_event_listeners();
    }

    #[test]
    fn on_window_close_disposes_scope() {
        let scope = create_scope(None);
        on_window_close(scope);
        // Scope should be disposed without panicking
    }
}
