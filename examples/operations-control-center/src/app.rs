use std::cell::Cell;

use domius_web::{domus, DomiusComponent, DomiusPage};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;

use domius_web::components::pro::result::{
    Result as ResultView, ResultAction, ResultProps, ResultStatus,
};

use crate::components::{app_navigation, mark_route_links};
use crate::pages::{
    IncidentsPage, OverviewPage, ReportsPage, ServiceDetailPage, ServiceDetailProps,
};
use crate::routes::{router, AppRoute};
use crate::state::MonitoringContext;

pub fn mount() -> Result<(), JsValue> {
    MonitoringContext::seeded(0xD0_51_05).provide();
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window unavailable"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document unavailable"))?;
    let host = document
        .get_element_by_id("app")
        .ok_or_else(|| JsValue::from_str("#app not found"))?;
    let path = match window.location().pathname()?.as_str() {
        "/" | "" => "/overview".to_string(),
        path => path.to_string(),
    };
    if window.location().pathname()? != path {
        window
            .history()?
            .replace_state_with_url(&JsValue::NULL, "", Some(&path))?;
    }

    install_navigation(&document)?;
    render_path(&host, &path)
}

fn render_path(host: &Element, path: &str) -> Result<(), JsValue> {
    host.set_text_content(None);
    host.append_child(&app_navigation(path, move |target| navigate_to(&target, true)))?;

    let routes = router();
    let (route, params) = routes
        .match_route(path)
        .expect("wildcard route must always match");
    let document = web_sys::window().unwrap().document().unwrap();
    let content = match route {
        AppRoute::Overview => {
            let state = OverviewPage::setup(());
            document.set_title(&OverviewPage::title(&state));
            OverviewPage::render(&state)
        }
        AppRoute::ServiceDetail => {
            let state = ServiceDetailPage::setup(ServiceDetailProps {
                service_id: params
                    .get("id")
                    .cloned()
                    .unwrap_or_else(|| "unknown".to_string()),
            });
            document.set_title(&ServiceDetailPage::title(&state));
            ServiceDetailPage::render(&state)
        }
        AppRoute::Incidents => {
            let state = IncidentsPage::setup(());
            document.set_title(&IncidentsPage::title(&state));
            IncidentsPage::render(&state)
        }
        AppRoute::Reports => {
            let state = ReportsPage::setup(());
            document.set_title(&ReportsPage::title(&state));
            ReportsPage::render(&state)
        }
        AppRoute::NotFound => {
            document.set_title("Page not found | Domius");
            not_found_view(path)
        }
    };
    host.append_child(&content)?;
    Ok(())
}

thread_local! {
    static NAVIGATION_INSTALLED: Cell<bool> = const { Cell::new(false) };
}

/// Install one delegated listener for the application lifetime. This catches
/// links produced by asynchronous effects as well as links present at mount.
fn install_navigation(document: &web_sys::Document) -> Result<(), JsValue> {
    if NAVIGATION_INSTALLED.with(Cell::get) {
        return Ok(());
    }

    let click = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(|event: web_sys::MouseEvent| {
        if event.button() != 0
            || event.alt_key()
            || event.ctrl_key()
            || event.meta_key()
            || event.shift_key()
        {
            return;
        }
        let Some(link) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .and_then(|target| target.closest("a[data-route]").ok().flatten())
        else {
            return;
        };
        let Some(host) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id("app"))
        else {
            return;
        };
        let Some(target) = link.get_attribute("href") else {
            return;
        };
        if host.contains(Some(&link)) && target.starts_with('/') {
            event.prevent_default();
            navigate_to(&target, true);
        }
    });
    document.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;
    click.forget();

    let popstate = Closure::<dyn FnMut(web_sys::Event)>::new(|_| {
        if let Some(window) = web_sys::window() {
            if let Ok(path) = window.location().pathname() {
                navigate_to(&path, false);
            }
        }
    });
    web_sys::window()
        .ok_or_else(|| JsValue::from_str("window unavailable"))?
        .add_event_listener_with_callback("popstate", popstate.as_ref().unchecked_ref())?;
    popstate.forget();
    NAVIGATION_INSTALLED.with(|installed| installed.set(true));
    Ok(())
}

fn navigate_to(target: &str, push: bool) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(host) = window
        .document()
        .and_then(|document| document.get_element_by_id("app"))
    else {
        return;
    };
    if push {
        let _ = window
            .history()
            .and_then(|history| history.push_state_with_url(&JsValue::NULL, "", Some(target)));
    }
    let _ = render_path(&host, target);
}

/// Render an unknown route as a result the reader can act on, not a dead end.
fn not_found_view(path: &str) -> Element {
    let root = domus! {
        main(class: "route-placeholder") {
            div(id: "route-state") { }
        }
    };
    let result = ResultView::create(ResultProps {
        status: ResultStatus::Custom("404".to_string()),
        title: "Page not found".to_string(),
        description: Some(format!("No view is registered for {path}.")),
        actions: vec![
            ResultAction::new("Back to overview", "/overview").primary(),
            ResultAction::new("Open incidents", "/incidents"),
            ResultAction::new("Open reports", "/reports"),
        ],
        class: Some("route-not-found".to_string()),
        ..Default::default()
    });
    mark_route_links(&result, ".domius-result-action");
    root.query_selector("#route-state")
        .expect("query route state")
        .expect("route state host")
        .append_child(&result)
        .expect("append route state");
    root
}
