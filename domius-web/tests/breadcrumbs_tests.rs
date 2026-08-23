//! WASM integration tests for Breadcrumbs.

#![cfg(target_arch = "wasm32")]

use domius_web::components::navigation::breadcrumbs::{
    BreadcrumbItem, Breadcrumbs, BreadcrumbsProps,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn breadcrumbs_render_links_separators_and_current_page() {
    let breadcrumbs = Breadcrumbs::create(BreadcrumbsProps {
        items: vec![
            BreadcrumbItem {
                label: "Overview".to_string(),
                href: Some("/overview".to_string()),
                disabled: false,
            },
            BreadcrumbItem {
                label: "Services".to_string(),
                href: Some("/services".to_string()),
                disabled: false,
            },
            BreadcrumbItem {
                label: "Billing".to_string(),
                href: None,
                disabled: false,
            },
        ],
        separator: Some("›".to_string()),
        class: None,
    });

    assert_eq!(breadcrumbs.tag_name(), "NAV");
    assert_eq!(
        breadcrumbs.get_attribute("aria-label").as_deref(),
        Some("Breadcrumb")
    );
    assert_eq!(breadcrumbs.query_selector_all("li").unwrap().length(), 3);
    assert_eq!(breadcrumbs.query_selector_all("a").unwrap().length(), 2);
    assert_eq!(
        breadcrumbs
            .query_selector("[aria-current='page']")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("Billing")
    );
    assert_eq!(
        breadcrumbs
            .query_selector_all(".domius-breadcrumbs-separator")
            .unwrap()
            .length(),
        2
    );
}

#[wasm_bindgen_test]
fn disabled_breadcrumb_is_not_a_link() {
    let breadcrumbs = Breadcrumbs::create(BreadcrumbsProps {
        items: vec![BreadcrumbItem {
            label: "Restricted".to_string(),
            href: Some("/restricted".to_string()),
            disabled: true,
        }],
        ..Default::default()
    });

    assert!(breadcrumbs.query_selector("a").unwrap().is_none());
    assert_eq!(
        breadcrumbs
            .query_selector("[aria-disabled='true']")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("Restricted")
    );
}
