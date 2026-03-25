//! DOM binding and component system for Domius.

#![warn(missing_docs)]

pub mod component;
pub mod context;
pub mod disposal;
pub mod list;
pub mod page;
pub mod router;
pub use component::{DomiusComponent, DomiusNode, mount_component};
pub use page::DomiusPage;
pub use router::{Router, RoutePattern};

/// Initialize the Domius runtime. Call once from `wasm_bindgen(start)`.
pub fn init() {
    disposal::init_disposal_observer();
}
