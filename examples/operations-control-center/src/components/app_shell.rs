use domius_web::components::navigation::navbar::NavLink;
use domius_web::components::navigation::{Navbar, NavbarProps};
use web_sys::Element;

pub fn app_navigation(active_path: &str, on_navigate: impl Fn(String) + 'static) -> Element {
    let links = [
        ("Overview", "/overview"),
        ("Incidents", "/incidents"),
        ("Reports", "/reports"),
    ]
    .into_iter()
    .map(|(label, href)| NavLink {
        label: label.to_string(),
        href: Some(href.to_string()),
        active: active_path == href,
        icon: None,
        children: Vec::new(),
    })
    .collect();

    Navbar::create(NavbarProps {
        logo_text: Some("Operations Control Center".to_string()),
        links,
        on_link_click: Some(Box::new(on_navigate)),
        ..Default::default()
    })
}

/// Mark links the shell should intercept instead of letting the browser reload.
///
/// Components from the library render plain anchors; the shell only wires the
/// ones carrying this marker, so views opt in explicitly.
pub fn mark_route_links(root: &Element, selector: &str) {
    let links = root
        .query_selector_all(selector)
        .expect("query internal links");
    for index in 0..links.length() {
        if let Some(link) = links
            .item(index)
            .and_then(|node| wasm_bindgen::JsCast::dyn_into::<Element>(node).ok())
        {
            link.set_attribute("data-route", "")
                .expect("mark internal link");
        }
    }
}
