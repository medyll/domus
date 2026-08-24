//! WASM integration tests for InfiniteScroll.

#![cfg(target_arch = "wasm32")]

mod test_utils;

use std::cell::Cell;
use std::rc::Rc;

use domius_core::signal::signal;
use domius_web::components::feedback::infinite_scroll::{InfiniteScroll, InfiniteScrollProps};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn infinite_scroll_renders_content_sentinel_and_status() {
    let document = web_sys::window().unwrap().document().unwrap();
    let content = document.create_element("ol").unwrap();
    content.set_id("feed");
    let root = InfiniteScroll::create(InfiniteScrollProps {
        children: content,
        threshold: 240,
        ..Default::default()
    });

    assert_eq!(root.tag_name(), "SECTION");
    assert_eq!(root.first_element_child().unwrap().id(), "feed");
    assert_eq!(root.get_attribute("data-threshold").as_deref(), Some("240"));
    assert_eq!(root.get_attribute("aria-busy").as_deref(), Some("false"));
    assert_eq!(
        root.query_selector_all("[role='status']").unwrap().length(),
        1
    );
    assert_eq!(
        root.query_selector_all(".domius-infinite-scroll-sentinel")
            .unwrap()
            .length(),
        1
    );
}

#[wasm_bindgen_test]
fn manual_fallback_requests_only_once_while_loading() {
    let document = web_sys::window().unwrap().document().unwrap();
    let loading = signal(false);
    let requests = Rc::new(Cell::new(0));
    let captured_requests = Rc::clone(&requests);
    let root = InfiniteScroll::create(InfiniteScrollProps {
        children: document.create_element("div").unwrap(),
        loading: loading.clone(),
        on_load_more: Box::new(move || captured_requests.set(captured_requests.get() + 1)),
        ..Default::default()
    });
    let button = root.query_selector("button").unwrap().unwrap();

    test_utils::simulate_click(&button);
    test_utils::simulate_click(&button);
    assert_eq!(requests.get(), 1);
    assert!(loading.get());
    assert!(button.has_attribute("disabled"));

    loading.set(false);
    test_utils::simulate_click(&button);
    assert_eq!(requests.get(), 2);
}

#[wasm_bindgen_test]
fn completed_reverse_feed_has_no_load_control() {
    let document = web_sys::window().unwrap().document().unwrap();
    let root = InfiniteScroll::create(InfiniteScrollProps {
        children: document.create_element("ol").unwrap(),
        has_more: false,
        reverse: true,
        ..Default::default()
    });

    assert_eq!(
        root.get_attribute("data-direction").as_deref(),
        Some("reverse")
    );
    assert_eq!(root.query_selector_all("button").unwrap().length(), 0);
    assert_eq!(
        root.query_selector("[role='status']")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("All items loaded")
    );
}
