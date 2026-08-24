use std::rc::Rc;

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

    render_path(&host, &path)
}

fn render_path(host: &Element, path: &str) -> Result<(), JsValue> {
    host.set_text_content(None);
    let host_for_navigation = host.clone();
    let navigate: Rc<dyn Fn(String)> = Rc::new(move |target: String| {
        if let Some(window) = web_sys::window() {
            let _ = window
                .history()
                .and_then(|history| history.push_state_with_url(&JsValue::NULL, "", Some(&target)));
            let _ = render_path(&host_for_navigation, &target);
        }
    });
    let navigate_for_nav = Rc::clone(&navigate);
    host.append_child(&app_navigation(path, move |target| {
        navigate_for_nav(target);
    }))?;

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
    wire_route_links(&content, Rc::clone(&navigate));
    host.append_child(&content)?;
    Ok(())
}

fn wire_route_links(content: &Element, navigate: Rc<dyn Fn(String)>) {
    let links = content
        .query_selector_all("a[data-route]")
        .expect("query internal route links");
    for index in 0..links.length() {
        let Some(link) = links
            .item(index)
            .and_then(|node| node.dyn_into::<Element>().ok())
        else {
            continue;
        };
        let Some(target) = link.get_attribute("href") else {
            continue;
        };
        let callback = Rc::clone(&navigate);
        let handler =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                callback(target.clone());
            });
        link.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .expect("register internal route link");
        handler.forget();
    }
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
