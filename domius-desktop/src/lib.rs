//! Independent desktop runtime foundation for Domius.
//!
//! This crate owns platform-neutral desktop application concepts:
//! - component-backed native windows;
//! - automatic scope disposal via window events;
//! - shared context;
//! - application events.
//!
//! Platform window and webview engines are implementation details and must not
//! leak through this public API.

#![allow(
    missing_docs,
    dead_code,
    unused_variables,
    unused_imports,
    static_mut_refs
)]

pub mod component;
pub mod context;
pub mod disposal;
pub mod event;

/// Initialize the Domius desktop runtime.
/// Call once from the Domius desktop application entry point.
pub fn init() {
    disposal::init_event_listeners();
}

// Re-exports
pub use component::{
    build_window_config, cleanup_component_scope, get_component_url, ComponentScope,
    DomiusDesktopComponent,
};
pub use context::{provide_context, use_context};
