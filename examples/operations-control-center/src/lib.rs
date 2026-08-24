//! Operations Control Center, a full Domius monitoring application.

pub mod app;
pub mod components;
pub mod data;
pub mod pages;
pub mod routes;
pub mod state;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Entry point for the WASM bundle.
///
/// Deliberately not called `main`: the test harness declares its own entry
/// symbol, and two of them will not link.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    domius_web::init();
    app::mount().expect("mount Operations Control Center");
}
