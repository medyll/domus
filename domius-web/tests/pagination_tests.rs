//! WASM integration tests for Pagination.

#![cfg(target_arch = "wasm32")]

mod test_utils;

use std::cell::RefCell;
use std::rc::Rc;

use domius_web::components::navigation::pagination::{Pagination, PaginationProps};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pagination_renders_a_bounded_accessible_page_range() {
    let (navigation, page) = Pagination::create(PaginationProps {
        total_items: 200,
        page_size: 10,
        current_page: Some(10),
        sibling_count: 1,
        ..Default::default()
    });

    assert_eq!(navigation.tag_name(), "NAV");
    assert_eq!(
        navigation.get_attribute("aria-label").as_deref(),
        Some("Pagination")
    );
    assert_eq!(
        navigation.get_attribute("data-total-pages").as_deref(),
        Some("20")
    );
    assert_eq!(page.get(), 10);
    assert_eq!(
        navigation
            .query_selector_all("[data-action='page']")
            .unwrap()
            .length(),
        5
    );
    assert_eq!(
        navigation
            .query_selector_all(".pagination-ellipsis")
            .unwrap()
            .length(),
        2
    );
    assert_eq!(
        navigation
            .query_selector("[aria-current='page']")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("10")
    );
}

#[wasm_bindgen_test]
fn pagination_updates_signal_markup_and_callback() {
    let changes = Rc::new(RefCell::new(Vec::new()));
    let captured_changes = Rc::clone(&changes);
    let (navigation, page) = Pagination::create(PaginationProps {
        total_items: 42,
        current_page: Some(1),
        on_page_change: Some(Box::new(move |next| {
            captured_changes.borrow_mut().push(next)
        })),
        ..Default::default()
    });

    let next = navigation
        .query_selector("[data-action='next']")
        .unwrap()
        .unwrap();
    test_utils::simulate_click(&next);

    assert_eq!(page.get(), 2);
    assert_eq!(*changes.borrow(), vec![2]);
    assert_eq!(
        navigation.get_attribute("data-current-page").as_deref(),
        Some("2")
    );
    assert_eq!(
        navigation
            .query_selector("[aria-current='page']")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("2")
    );
}

#[wasm_bindgen_test]
fn pagination_clamps_invalid_configuration() {
    let (navigation, page) = Pagination::create(PaginationProps {
        total_items: 0,
        page_size: 0,
        current_page: Some(999),
        ..Default::default()
    });

    assert_eq!(page.get(), 1);
    assert_eq!(
        navigation.get_attribute("data-total-pages").as_deref(),
        Some("1")
    );
    assert!(navigation
        .query_selector("[data-action='previous']")
        .unwrap()
        .unwrap()
        .has_attribute("disabled"));
    assert!(navigation
        .query_selector("[data-action='next']")
        .unwrap()
        .unwrap()
        .has_attribute("disabled"));
}
