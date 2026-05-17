//! Desktop backend for Domus using Tauri.
//!
//! This crate provides Tauri-specific implementations of:
//! - Component system (native windows)
//! - Automatic disposal via window events
//! - Context API
//! - Event handling

#![allow(missing_docs, dead_code, unused_variables, unused_imports, static_mut_refs)]

pub mod component;
pub mod context;
pub mod disposal;
pub mod event;

/// Initialize the Domius desktop runtime.
/// Call once from your Tauri setup hook.
pub fn init() {
    disposal::init_event_listeners();
}

// Re-exports
pub use component::{
    build_window_config, cleanup_component_scope, DomiusDesktopComponent,
    get_component_url, ComponentScope,
};
pub use context::{provide_context, use_context};
