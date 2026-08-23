use std::rc::Rc;

use domius_web::{domus, DomiusComponent, DomiusPage};
use wasm_bindgen::JsValue;
use web_sys::Element;

use crate::components::app_navigation;
use crate::pages::OverviewPage;
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
    let navigate = Rc::new(move |target: String| {
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
            document.set_title("Service detail | Domius");
            placeholder(
                "Service detail",
                params.get("id").map(String::as_str).unwrap_or("unknown"),
            )
        }
        AppRoute::Incidents => {
            document.set_title("Incidents | Domius");
            placeholder("Incidents", "Implementation follows overview")
        }
        AppRoute::Reports => {
            document.set_title("Reports | Domius");
            placeholder("Reports", "Implementation follows incidents")
        }
        AppRoute::NotFound => {
            document.set_title("Page not found | Domius");
            placeholder("Page not found", "Return to /overview")
        }
    };
    host.append_child(&content)?;
    Ok(())
}

fn placeholder(title: &str, description: &str) -> Element {
    let title = title.to_string();
    let description = description.to_string();

    domus! {
        main(class: "route-placeholder") {
            h1 { {title} }
            p { {description} }
        }
    }
}
