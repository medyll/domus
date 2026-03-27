//! Integration tests for domius-web components.
//!
//! These tests run in a browser environment using wasm-pack test.
//!
//! # Running Tests
//!
//! ```bash
//! # Run all WASM tests in headless browser
//! wasm-pack test --headless --firefox domius-web
//!
//! # Run tests in Chrome
//! wasm-pack test --headless --chrome domius-web
//!
//! # Run specific test file
//! wasm-pack test --headless --firefox domius-web -- button_tests
//! ```

pub mod test_utils;

#[cfg(test)]
mod button_tests;

#[cfg(test)]
mod input_tests;

#[cfg(test)]
mod hooks_tests;

#[cfg(test)]
mod modal_toast_tests;
