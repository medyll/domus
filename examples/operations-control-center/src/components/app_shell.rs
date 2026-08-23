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
