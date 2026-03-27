//! Core reactive runtime for Domius.
//!
//! This crate provides the low-level signal/effect runtime used by higher-level
//! crates such as `domius-web` and `domius-cli`.

#![warn(missing_docs)]

/// Reactive computed (derived/memo) values.
pub mod computed;
/// Reactive effects — auto-tracking closures that re-run on signal change.
pub mod effect;
/// Scheduler and batch execution runtime.
pub mod runtime;
/// Scope system for grouped effect disposal.
pub mod scope;
/// Reactive signal primitives.
pub mod signal;

pub use computed::{computed, Computed};
pub use effect::{create_effect, Effect};
pub use runtime::batch;
pub use scope::{create_scope, dispose_scope, ScopeId};
pub use signal::{signal, Signal};
