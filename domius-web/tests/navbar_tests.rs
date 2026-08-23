//! WASM integration tests for Navbar.

#![cfg(target_arch = "wasm32")]

mod test_utils;

use std::cell::RefCell;
use std::rc::Rc;

use domius_web::components::navigation::navbar::NavLink;
use domius_web::components::navigation::{Navbar, NavbarProps};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn links() -> Vec<NavLink> {
    vec![
        NavLink {
            label: "Overview".to_string(),
            href: Some("/overview".to_string()),
            active: true,
            icon: None,
            children: Vec::new(),
        },
        NavLink {
            label: "Reports".to_string(),
            href: Some("/reports".to_string()),
            active: false,
            icon: None,
            children: Vec::new(),
        },
    ]
}

#[wasm_bindgen_test]
fn navbar_renders_landmarks_links_and_active_state() {
    let navbar = Navbar::create(NavbarProps {
        logo_text: Some("Operations".to_string()),
        links: links(),
        ..Default::default()
    });

    assert_eq!(navbar.tag_name(), "NAV");
    assert_eq!(
        navbar.get_attribute("aria-label").as_deref(),
        Some("Primary navigation")
    );
    assert_eq!(navbar.query_selector_all("a").unwrap().length(), 3);
    assert_eq!(
        navbar
            .query_selector("[aria-current='page']")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("Overview")
    );
}

#[wasm_bindgen_test]
fn navbar_reports_the_selected_target() {
    let selected = Rc::new(RefCell::new(None));
    let selected_for_callback = Rc::clone(&selected);
    let navbar = Navbar::create(NavbarProps {
        links: links(),
        on_link_click: Some(Box::new(move |target| {
            *selected_for_callback.borrow_mut() = Some(target);
        })),
        ..Default::default()
    });
    let reports = navbar
        .query_selector("a[href='/reports']")
        .unwrap()
        .unwrap();

    test_utils::simulate_click(&reports);

    assert_eq!(selected.borrow().as_deref(), Some("/reports"));
}
