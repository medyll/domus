#![cfg(target_arch = "wasm32")]

mod test_utils;

use std::cell::RefCell;
use std::rc::Rc;

use domius_core::signal::signal;
use domius_web::disposal::{init_disposal_observer, ViewScope, SCOPE_ATTRIBUTE};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Effects flush on an animation frame, so give the runtime one before looking.
async fn settle() {
    for _ in 0..3 {
        let promise = js_sys::Promise::new(&mut |resolve, _| {
            test_utils::window()
                .request_animation_frame(&resolve)
                .expect("animation frame should be scheduled");
        });
        wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .expect("animation frame should run");
    }
}

/// A scoped view that records every run of its effect.
fn scoped_view(
    id: &str,
) -> (
    web_sys::Element,
    ViewScope,
    Rc<RefCell<Vec<i32>>>,
    domius_core::Signal<i32>,
) {
    let root: web_sys::Element = test_utils::create_div().unchecked_into();
    root.set_id(id);
    let scope = ViewScope::attach(&root);

    let source = signal(0);
    let runs = Rc::new(RefCell::new(Vec::new()));
    let watched = source.clone();
    let recorded = Rc::clone(&runs);
    scope.effect(move || recorded.borrow_mut().push(watched.get()));

    (root, scope, runs, source)
}

#[wasm_bindgen_test]
fn a_view_scope_stamps_its_element() {
    let root: web_sys::Element = test_utils::create_div().unchecked_into();
    let scope = ViewScope::attach(&root);

    assert_eq!(
        root.get_attribute(SCOPE_ATTRIBUTE),
        Some(scope.id().value().to_string())
    );
}

#[wasm_bindgen_test]
async fn scoped_effects_run_until_the_scope_is_disposed() {
    let (_root, scope, runs, source) = scoped_view("scope-manual");
    assert_eq!(*runs.borrow(), vec![0]);

    source.set(1);
    settle().await;
    assert_eq!(
        *runs.borrow(),
        vec![0, 1],
        "the effect should follow its signal"
    );

    scope.dispose();
    source.set(2);
    settle().await;
    assert_eq!(
        *runs.borrow(),
        vec![0, 1],
        "a disposed scope should not run again"
    );
}

#[wasm_bindgen_test]
async fn removing_the_container_stops_its_effects() {
    init_disposal_observer();
    let (root, _scope, runs, source) = scoped_view("scope-observed");
    let body = test_utils::document().body().expect("no body");
    body.append_child(&root).expect("attach view");

    source.set(1);
    settle().await;
    assert_eq!(*runs.borrow(), vec![0, 1]);

    body.remove_child(&root).expect("detach view");
    settle().await;

    source.set(2);
    settle().await;
    assert_eq!(
        *runs.borrow(),
        vec![0, 1],
        "removing the container should have stopped the effect"
    );
}

#[wasm_bindgen_test]
async fn removing_an_ancestor_stops_a_nested_view() {
    init_disposal_observer();
    let page: web_sys::Element = test_utils::create_div().unchecked_into();
    page.set_id("scope-page");
    let (panel, _scope, runs, source) = scoped_view("scope-panel");
    page.append_child(&panel).expect("nest panel");

    let body = test_utils::document().body().expect("no body");
    body.append_child(&page).expect("attach page");

    source.set(1);
    settle().await;
    assert_eq!(*runs.borrow(), vec![0, 1]);

    // The panel is never removed itself; only the page above it is.
    body.remove_child(&page).expect("detach page");
    settle().await;

    source.set(2);
    settle().await;
    assert_eq!(
        *runs.borrow(),
        vec![0, 1],
        "a nested scope should die with the subtree it lives in"
    );
}

#[wasm_bindgen_test]
async fn a_sibling_view_keeps_running() {
    init_disposal_observer();
    let (doomed, _doomed_scope, doomed_runs, doomed_source) = scoped_view("scope-doomed");
    let (kept, _kept_scope, kept_runs, kept_source) = scoped_view("scope-kept");

    let body = test_utils::document().body().expect("no body");
    body.append_child(&doomed).expect("attach doomed view");
    body.append_child(&kept).expect("attach kept view");

    body.remove_child(&doomed).expect("detach doomed view");
    settle().await;

    doomed_source.set(1);
    kept_source.set(1);
    settle().await;

    assert_eq!(*doomed_runs.borrow(), vec![0]);
    assert_eq!(*kept_runs.borrow(), vec![0, 1]);

    body.remove_child(&kept).expect("clean up");
}
