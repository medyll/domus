//! Core reactive runtime for Domus.
//!
//! This crate provides the low-level signal/effect runtime used by higher-level
//! crates such as `domus-web` and `domus-cli`.

#![warn(missing_docs)]

pub mod effect;
pub mod runtime;
pub mod scope;
pub mod signal;

pub use effect::{create_effect, Effect};
pub use runtime::batch;
pub use scope::{create_scope, dispose_scope, ScopeId};
pub use signal::{signal, Signal};
