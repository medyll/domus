//! Operations Control Center, a full Domius monitoring application.

pub mod app;
pub mod components;
pub mod data;
pub mod pages;
pub mod routes;
pub mod state;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    domius_web::init();
    app::mount().expect("mount Operations Control Center");
}
