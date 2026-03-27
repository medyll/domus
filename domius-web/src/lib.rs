//! DOM binding and component system for Domius.
//!
//! This crate provides the component system, router, context API, and a comprehensive
//! UI component library for building reactive web applications with Domius.

#![warn(missing_docs)]

pub mod component;
pub mod context;
pub mod disposal;
pub mod list;
pub mod page;
pub mod router;

// UI Component Library
pub mod components;
pub mod hooks;
pub mod utils;

// Re-exports from core modules
pub use component::{DomiusComponent, DomiusNode, mount_component};
pub use page::DomiusPage;
pub use router::{Router, RoutePattern};

// Re-exports from hooks
pub use hooks::{use_click_outside, use_keyboard, use_focus};

// Re-exports from utils
pub use utils::class_names;
pub use utils::Portal;

/// Initialize the Domius runtime. Call once from `wasm_bindgen(start)`.
///
/// This sets up the MutationObserver for automatic scope disposal.
pub fn init() {
    disposal::init_disposal_observer();
    utils::portal::init_portal_containers();
}
