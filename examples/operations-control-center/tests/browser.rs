//! The eight acceptance tests from this example's README, run in a real browser.

#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::rc::Rc;

use domius_core::create_effect;
use domius_core::signal::signal;
use domius_web::context::clear_all_contexts;
use operations_control_center::app;
use operations_control_center::data::monitoring_fixture;
use operations_control_center::state::{Acknowledgement, FilterContext};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn document() -> web_sys::Document {
    web_sys::window()
        .expect("no window")
        .document()
        .expect("no document")
}

/// Effects flush on an animation frame; give the runtime a few before looking.
async fn settle() {
    for _ in 0..3 {
        let promise = js_sys::Promise::new(&mut |resolve, _| {
            web_sys::window()
                .expect("no window")
                .request_animation_frame(&resolve)
                .expect("animation frame should be scheduled");
        });
        wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .expect("animation frame should run");
    }
}

thread_local! {
    /// The disposal observer is installed once for the whole document.
    static RUNTIME_READY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Mount the application at `path`, on a document with nothing left over.
async fn boot(path: &str) {
    RUNTIME_READY.with(|ready| {
        if !ready.replace(true) {
            domius_web::init();
        }
    });
    clear_all_contexts();
    let document = document();
    if let Some(previous) = document.get_element_by_id("app") {
        previous.remove();
    }
    let host = document.create_element("div").expect("create app host");
    host.set_id("app");
    document
        .body()
        .expect("no body")
        .append_child(&host)
        .expect("attach app host");
    web_sys::window()
        .expect("no window")
        .history()
        .expect("no history")
        .replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(path))
        .expect("set the starting route");

    app::mount().expect("mount the application");
    settle().await;
}

/// Put the address bar back at the root the harness was loaded from.
fn reset_history() {
    web_sys::window()
        .expect("no window")
        .history()
        .expect("no history")
        .replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/"))
        .expect("restore the harness route");
}

fn query(selector: &str) -> Option<web_sys::Element> {
    document().query_selector(selector).expect("query document")
}

fn expect(selector: &str) -> web_sys::Element {
    query(selector).unwrap_or_else(|| panic!("{selector} should be on the page"))
}

fn count(selector: &str) -> usize {
    document()
        .query_selector_all(selector)
        .expect("query document")
        .length() as usize
}

fn click(element: &web_sys::Element) {
    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    let event =
        web_sys::MouseEvent::new_with_mouse_event_init_dict("click", &init).expect("create click");
    element.dispatch_event(&event).expect("dispatch click");
}

fn change(selector: &str, value: &str) {
    let control = expect(selector)
        .dyn_into::<web_sys::HtmlSelectElement>()
        .expect("control should be a select");
    control.set_value(value);
    let event = web_sys::Event::new("change").expect("create change");
    control.dispatch_event(&event).expect("dispatch change");
}

// 1. Following a link changes the route, the title and the content, without a
//    full reload.
//
#[wasm_bindgen_test]
async fn following_a_link_changes_the_view_in_place() {
    boot("/overview").await;
    assert_eq!(document().title(), "Operations overview | Domius");
    let overview = expect("main.operations-overview");

    click(&expect("#service-health a[href='/services/svc-01']"));
    settle().await;

    assert_eq!(
        web_sys::window().unwrap().location().pathname().unwrap(),
        "/services/svc-01"
    );
    assert_eq!(document().title(), "Gateway service | Domius");
    assert!(query("main.service-detail").is_some());
    assert!(query("main.operations-overview").is_none());
    // The old view left the document rather than the document being rebuilt.
    assert!(!overview.is_connected());
}

#[wasm_bindgen_test]
async fn browser_history_restores_the_previous_view() {
    boot("/overview").await;
    click(&expect("a[href='/incidents']"));
    settle().await;
    assert!(query("main.incidents-page").is_some());

    web_sys::window()
        .unwrap()
        .history()
        .unwrap()
        .back()
        .expect("go back");
    settle().await;

    assert_eq!(
        web_sys::window().unwrap().location().pathname().unwrap(),
        "/overview"
    );
    assert_eq!(document().title(), "Operations overview | Domius");
    assert!(query("main.operations-overview").is_some());
    assert!(query("main.incidents-page").is_none());
}

// 1b. A route parameter reaches the page that reads it.
#[wasm_bindgen_test]
async fn a_route_parameter_selects_the_service() {
    boot("/services/svc-01").await;
    assert_eq!(document().title(), "Gateway service | Domius");
    assert!(query("main.service-detail").is_some());
    assert_eq!(
        expect("main.service-detail h1").text_content().as_deref(),
        Some("Gateway")
    );

    boot("/services/svc-03").await;
    assert_eq!(document().title(), "Billing service | Domius");
    assert_eq!(
        expect("main.service-detail h1").text_content().as_deref(),
        Some("Billing")
    );

    // An id nobody serves still lands somewhere usable.
    boot("/services/svc-99").await;
    assert_eq!(document().title(), "Service not found | Domius");
    assert!(query("main.service-detail a[href='/overview']").is_some());
    reset_history();
}

// 2. Applying two filters inside a batch produces one observable update of the
//    aggregates.
#[wasm_bindgen_test]
async fn batched_filters_update_the_aggregates_once() {
    clear_all_contexts();
    let filters = FilterContext::over(signal(monitoring_fixture(0xD0_51_05)));
    let observed = Rc::new(RefCell::new(Vec::new()));

    let counted = filters.matching_count.clone();
    let recorded = Rc::clone(&observed);
    create_effect(move || recorded.borrow_mut().push(counted.get()));
    assert_eq!(*observed.borrow(), vec![48]);

    filters.apply(None, Some("svc-02".to_string()), Acknowledgement::Open);
    settle().await;

    let observed = observed.borrow();
    assert_eq!(observed.len(), 2, "one batch, one update: {observed:?}");
    assert!(observed[1] < 48);
}

// 3. Reordering then removing incidents keeps the nodes carrying the same key.
#[wasm_bindgen_test]
async fn reordering_then_acknowledging_keeps_the_surviving_nodes() {
    boot("/incidents").await;
    assert_eq!(count("#queue-list > li"), 48);

    let tracked = document()
        .query_selector_all("#queue-list > li")
        .expect("query queue");
    let mut before = Vec::new();
    for index in 0..tracked.length() {
        let node: web_sys::Element = tracked.item(index).unwrap().unchecked_into();
        before.push((node.get_attribute("data-key").unwrap(), node));
    }

    change("#queue-order", "severity");
    settle().await;
    assert_eq!(count("#queue-list > li"), 48);
    assert!(
        before.iter().all(|(key, node)| {
            expect(&format!("#queue-list > li[data-key='{key}']")).is_same_node(Some(node))
        }),
        "reordering should move nodes, not rebuild them"
    );

    change("#filter-state", "open");
    settle().await;
    assert_eq!(count("#queue-list > li"), 36);

    let doomed = expect("#queue-list > li");
    let key = doomed.get_attribute("data-key").unwrap();
    click(&expect("#queue-list > li button.acknowledge"));
    settle().await;

    assert_eq!(count("#queue-list > li"), 35);
    assert!(
        !doomed.is_connected(),
        "the acknowledged row should be gone"
    );
    assert!(
        query(&format!("#queue-list > li[data-key='{key}']")).is_none(),
        "and its key with it"
    );
    // Every remaining row is the node it already was.
    assert!(before
        .iter()
        .filter_map(|(key, node)| {
            query(&format!("#queue-list > li[data-key='{key}']")).map(|live| (live, node))
        })
        .all(|(live, node)| live.is_same_node(Some(node))));
}

// 4. Scrolling the feed loads the next page exactly once.
#[wasm_bindgen_test]
async fn the_feed_loads_one_page_at_a_time() {
    boot("/incidents").await;
    let feed = expect("#incident-feed");
    let rows = || feed.query_selector_all("[data-key]").unwrap().length();
    let first = rows();
    assert!(first > 0, "the feed should start with a page");

    let load = feed
        .query_selector("button")
        .expect("query feed control")
        .expect("the feed should offer a way to load more");
    click(&load);
    settle().await;
    let second = rows();
    assert!(second > first, "a page should have been added");
    assert_eq!(second - first, first, "exactly one page, not two");
}

// 5. The table, grid, pivot, heatmap and scatter read the same metric set.
#[wasm_bindgen_test]
async fn every_report_view_reads_the_same_window() {
    boot("/reports").await;

    // 360 measurements, 6 services over 6 ten-minute windows.
    assert_eq!(count("#metric-table tbody tr"), 6);
    assert_eq!(count("#metric-grid [role='row'], #metric-grid tr") - 1, 360);
    assert_eq!(count("#error-heatmap tbody td:not([data-empty])"), 36);
    assert_eq!(count("#correlation-scatter [data-role='marks'] circle"), 36);
    assert_eq!(count("#metric-pivot tbody tr"), 6);

    // The heatmap and the scatter agree cell for cell on the error rate.
    let cells = document()
        .query_selector_all("#error-heatmap tbody td[data-value]")
        .unwrap();
    let marks = document()
        .query_selector_all("#correlation-scatter [data-role='marks'] circle")
        .unwrap();
    assert_eq!(cells.length(), marks.length());
    for index in 0..cells.length() {
        let cell: web_sys::Element = cells.item(index).unwrap().unchecked_into();
        let mark: web_sys::Element = marks.item(index).unwrap().unchecked_into();
        assert_eq!(
            cell.get_attribute("data-value"),
            mark.get_attribute("data-y"),
            "cell {index} disagrees with its mark"
        );
    }
}

// 6. An absent route renders a usable result with a way back to the start.
#[wasm_bindgen_test]
async fn an_unknown_route_offers_a_way_home() {
    boot("/nowhere-at-all").await;

    let result = expect("[data-status='404']");
    assert_eq!(document().title(), "Page not found | Domius");
    assert_eq!(result.get_attribute("role").as_deref(), Some("status"));
    assert!(result
        .query_selector(".domius-result-description")
        .unwrap()
        .unwrap()
        .text_content()
        .unwrap()
        .contains("/nowhere-at-all"));

    let home = expect("[data-status='404'] a[href='/overview']");
    click(&home);
    settle().await;

    assert_eq!(
        web_sys::window().unwrap().location().pathname().unwrap(),
        "/overview"
    );
    assert!(query("main.operations-overview").is_some());
    assert!(query("[data-status='404']").is_none());
}

// 7. The report QR code decodes to the URL printed beside it.
#[wasm_bindgen_test]
async fn the_report_code_decodes_to_the_url_on_screen() {
    boot("/reports").await;

    let shown = expect("#report-url").text_content().unwrap();
    let code = expect("#report-qrcode .qrcode");
    assert_eq!(
        code.get_attribute("data-value").as_deref(),
        Some(shown.as_str())
    );

    let svg = code.query_selector("svg").unwrap().unwrap();
    let extent: usize = svg
        .get_attribute("viewBox")
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .parse()
        .unwrap();

    // Rasterize exactly what the page drew, then read it back with a decoder.
    const SCALE: usize = 8;
    let side = extent * SCALE;
    let mut pixels = vec![255u8; side * side];
    let modules = code
        .query_selector_all("[data-role='modules'] rect")
        .unwrap();
    for index in 0..modules.length() {
        let module: web_sys::Element = modules.item(index).unwrap().unchecked_into();
        let x: usize = module.get_attribute("x").unwrap().parse().unwrap();
        let y: usize = module.get_attribute("y").unwrap().parse().unwrap();
        for row in 0..SCALE {
            for column in 0..SCALE {
                pixels[(y * SCALE + row) * side + x * SCALE + column] = 0;
            }
        }
    }

    let mut image =
        rqrr::PreparedImage::prepare_from_greyscale(side, side, |x, y| pixels[y * side + x]);
    let grids = image.detect_grids();
    assert_eq!(grids.len(), 1, "the page should show exactly one QR code");
    assert_eq!(grids[0].decode().expect("the code should decode").1, shown);
}

// 8. Removing a page's container stops the effects that page created.
#[wasm_bindgen_test]
async fn removing_a_container_stops_its_effects() {
    boot("/incidents").await;

    let queue = expect(".incident-queue");
    assert!(
        queue.has_attribute("data-domius-scope"),
        "the queue should own a scope"
    );
    let count_before = expect("#queue-count").get_attribute("data-count").unwrap();

    // Leaving the page removes the container the scope is stamped on.
    click(&expect("a[href='/overview']"));
    settle().await;
    assert!(!queue.is_connected());

    // The filters are shared, so a change now would reach a live effect.
    FilterContext::current()
        .expect("the filters should still be published")
        .apply(None, None, Acknowledgement::Open);
    settle().await;

    assert_eq!(
        queue
            .query_selector("#queue-count")
            .unwrap()
            .unwrap()
            .get_attribute("data-count"),
        Some(count_before),
        "the detached queue should have stopped following the filters"
    );
}
