//! Navbar component - Top navigation bar.

use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;

/// A navigation link.
#[derive(Clone)]
pub struct NavLink {
    pub label: String,
    pub href: Option<String>,
    pub active: bool,
    pub icon: Option<String>,
    pub children: Vec<NavLink>,
}

/// Props for the Navbar component.
pub struct NavbarProps {
    pub logo: Option<String>,
    pub logo_text: Option<String>,
    pub links: Vec<NavLink>,
    pub actions: Option<String>,
    pub fixed: bool,
    pub transparent: bool,
    pub on_link_click: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for NavbarProps {
    fn default() -> Self {
        Self {
            logo: None,
            logo_text: None,
            links: Vec::new(),
            actions: None,
            fixed: false,
            transparent: false,
            on_link_click: None,
            class: None,
        }
    }
}

/// Navbar component.
pub struct Navbar;

impl Navbar {
    /// Create a navbar element.
    pub fn create(props: NavbarProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let navbar = document.create_element("nav").expect("create navbar");
        navbar
            .set_attribute("aria-label", "Primary navigation")
            .expect("set navbar label");

        let mut classes = vec!["domius-navbar"];
        if props.fixed {
            classes.push("domius-navbar-fixed");
        }
        if props.transparent {
            classes.push("domius-navbar-transparent");
        }
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        navbar.set_class_name(&classes.join(" "));

        if props.logo.is_some() || props.logo_text.is_some() {
            let brand = document.create_element("a").expect("create brand link");
            brand.set_class_name("domius-navbar-brand");
            brand.set_attribute("href", "/").expect("set brand target");

            if let Some(source) = props.logo {
                let image = document.create_element("img").expect("create brand image");
                image
                    .set_attribute("src", &source)
                    .expect("set brand source");
                image
                    .set_attribute("alt", props.logo_text.as_deref().unwrap_or("Home"))
                    .expect("set brand alternative");
                brand.append_child(&image).expect("append brand image");
            }
            if let Some(text) = props.logo_text {
                let label = document.create_element("span").expect("create brand text");
                label.set_text_content(Some(&text));
                brand.append_child(&label).expect("append brand text");
            }
            navbar.append_child(&brand).expect("append brand");
        }

        let callback = props.on_link_click.map(Rc::<dyn Fn(String)>::from);
        let links = render_links(&document, props.links, callback);
        navbar
            .append_child(&links)
            .expect("append navigation links");

        if let Some(actions) = props.actions {
            let actions_element = document.create_element("div").expect("create actions");
            actions_element.set_class_name("domius-navbar-actions");
            actions_element.set_text_content(Some(&actions));
            navbar
                .append_child(&actions_element)
                .expect("append actions");
        }

        navbar
    }
}

fn render_links(
    document: &web_sys::Document,
    links: Vec<NavLink>,
    callback: Option<Rc<dyn Fn(String)>>,
) -> Element {
    let list = document
        .create_element("ul")
        .expect("create navigation list");
    list.set_class_name("domius-navbar-links");

    for link in links {
        let item = document
            .create_element("li")
            .expect("create navigation item");
        item.set_class_name("domius-navbar-item");
        let target = link.href.clone().unwrap_or_else(|| link.label.clone());

        let control = if let Some(href) = link.href {
            let anchor = document
                .create_element("a")
                .expect("create navigation link");
            anchor
                .set_attribute("href", &href)
                .expect("set link target");
            anchor
        } else {
            let button = document
                .create_element("button")
                .expect("create navigation button");
            button
                .set_attribute("type", "button")
                .expect("set button type");
            button
        };
        control.set_class_name("domius-navbar-link");
        if link.active {
            control
                .set_attribute("aria-current", "page")
                .expect("mark active navigation link");
        }

        if let Some(icon) = link.icon {
            let icon_element = document.create_element("span").expect("create link icon");
            icon_element.set_class_name("domius-navbar-link-icon");
            icon_element
                .set_attribute("aria-hidden", "true")
                .expect("hide decorative icon");
            icon_element.set_text_content(Some(&icon));
            control
                .append_child(&icon_element)
                .expect("append link icon");
        }

        let label = document.create_element("span").expect("create link label");
        label.set_text_content(Some(&link.label));
        control.append_child(&label).expect("append link label");

        if let Some(callback) = callback.clone() {
            let handler = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(
                move |event: web_sys::MouseEvent| {
                    event.prevent_default();
                    callback(target.clone());
                },
            );
            control
                .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                .expect("register navigation callback");
            handler.forget();
        }

        item.append_child(&control).expect("append link control");
        if !link.children.is_empty() {
            let children = render_links(document, link.children, callback.clone());
            children.set_class_name("domius-navbar-submenu");
            item.append_child(&children).expect("append submenu");
        }
        list.append_child(&item).expect("append navigation item");
    }

    list
}
