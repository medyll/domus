//! DOM binding and component system for Domius.
//!
//! This crate provides the component system, router, context API, and a comprehensive
//! UI component library for building reactive web applications with Domius.

#![allow(
    missing_docs,
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_must_use,
    unknown_lints,
    clippy::type_complexity,
    clippy::useless_vec,
    clippy::large_enum_variant,
    clippy::derive_partial_eq_without_eq,
    clippy::redundant_pub_crate,
    clippy::enum_glob_use,
    clippy::items_after_statements,
    clippy::pattern_type_mismatch,
    clippy::cast_lossless,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::doc_markdown,
    clippy::derivable_impls,
    clippy::manual_strip,
    clippy::single_char_add_str,
    clippy::needless_borrow,
    clippy::redundant_closure,
    clippy::useless_format,
    clippy::useless_conversion,
    clippy::borrowed_box,
    clippy::new_ret_no_self,
    clippy::implicit_saturating_sub,
    clippy::manual_pattern_char_comparison,
    clippy::multiple_bound_locations
)]

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
