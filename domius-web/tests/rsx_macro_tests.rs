//! Browser-level regression tests for the public RSX macro.

#![cfg(target_arch = "wasm32")]

mod test_utils;

use domius_core::signal::signal;
use domius_web::domus;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

async fn next_animation_frame() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        test_utils::window()
            .request_animation_frame(&resolve)
            .expect("animation frame should be scheduled");
    });
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .expect("animation frame should run");
}

#[wasm_bindgen_test]
fn domus_macro_builds_nested_static_dom() {
    let _guard = test_utils::TestContainerGuard::new("test-rsx-static");
    let container = test_utils::get_element_by_id("test-rsx-static").unwrap();

    let card = domus! {
        article(class: "card") {
            h2 { "Status" }
            p(id: "status-copy") { "Ready" }
        }
    };
    container.append_child(&card).unwrap();

    assert_eq!(card.tag_name(), "ARTICLE");
    assert_eq!(card.class_name(), "card");
    assert_eq!(
        card.query_selector("h2")
            .unwrap()
            .unwrap()
            .text_content()
            .unwrap(),
        "Status"
    );
    assert_eq!(
        card.query_selector("#status-copy")
            .unwrap()
            .unwrap()
            .text_content()
            .unwrap(),
        "Ready"
    );
}

#[wasm_bindgen_test]
async fn domus_macro_updates_dynamic_text_after_event() {
    let _guard = test_utils::TestContainerGuard::new("test-rsx-reactive");
    let container = test_utils::get_element_by_id("test-rsx-reactive").unwrap();
    let count = signal(0i32);

    let counter = domus! {
        section {
            output(id: "rsx-count") { {count.get()} }
            button(on_click: {move |_| count.set(count.get() + 1)}) { "Increment" }
        }
    };
    container.append_child(&counter).unwrap();

    let output = counter.query_selector("#rsx-count").unwrap().unwrap();
    let button: web_sys::HtmlElement = counter
        .query_selector("button")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    assert_eq!(output.text_content().unwrap(), "0");
    assert_eq!(count.get(), 0);
    test_utils::simulate_click(&button);
    assert_eq!(count.get(), 1);
    next_animation_frame().await;
    assert_eq!(output.text_content().as_deref(), Some("1"));
}
