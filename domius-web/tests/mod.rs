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

// Each `*_tests.rs` file is an independent Cargo integration-test crate and
// includes `test_utils` directly.
