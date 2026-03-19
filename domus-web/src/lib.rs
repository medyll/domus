//! DOM binding and component system for Domus.

#![warn(missing_docs)]

pub mod component;
pub use component::{DomusComponent, DomusNode, mount_component};

/// Initialize the Domus runtime. Call once from `wasm_bindgen(start)`.
pub fn init() {
    // Initialization hook — reserved for future console_error_panic_hook setup.
}
