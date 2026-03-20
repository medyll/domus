//! DOM binding and component system for Domus.

#![warn(missing_docs)]

pub mod component;
pub mod context;
pub mod disposal;
pub mod list;
pub mod page;
pub mod router;
pub use component::{DomusComponent, DomusNode, mount_component};
pub use page::DomusPage;
pub use router::{Router, RoutePattern};

/// Initialize the Domus runtime. Call once from `wasm_bindgen(start)`.
pub fn init() {
    disposal::init_disposal_observer();
}
