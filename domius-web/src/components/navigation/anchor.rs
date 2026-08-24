//! Anchor component for Domius.
//!
//! A table of contents that follows the section the reader is looking at.

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::utils::scroll::{follow_scroll, ScrollTarget};

/// One entry of the anchor list.
#[derive(Clone, Default)]
pub struct AnchorLink {
    pub href: String,
    pub title: String,
}

/// Anchor props.
#[derive(Clone, Default)]
pub struct AnchorProps {
    /// Anchor links
    pub links: Vec<AnchorLink>,
    /// CSS class
    pub class: Option<String>,
    /// Offset from top (in px)
    pub offset_top: u32,
    /// Show boundary indicator line
    pub show_boundary: bool,
    /// Scroll container target
    pub target_container: Option<String>,
}

/// Build an Anchor component.
///
/// The list is a navigation landmark of real links, so it works before any
/// script does. On top of that it marks the entry whose section the reader has
/// reached with `aria-current` and `data-active`, and activating an entry
/// scrolls its section into view rather than jumping the document.
pub fn anchor(props: AnchorProps) -> Element {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let container = document
        .create_element("nav")
        .expect("create anchor container");
    let mut classes = vec!["anchor"];
    if let Some(class) = props.class.as_deref() {
        classes.push(class);
    }
    container.set_class_name(&classes.join(" "));
    container
        .set_attribute("aria-label", "On this page")
        .expect("label anchor navigation");
    container
        .set_attribute("data-offset-top", &props.offset_top.to_string())
        .expect("set anchor offset");

    if props.show_boundary {
        let line = document.create_element("div").expect("create anchor line");
        line.set_class_name("anchor-line");
        line.set_attribute("aria-hidden", "true")
            .expect("hide anchor line");
        container.append_child(&line).expect("append anchor line");
    }

    let list = document.create_element("ol").expect("create anchor list");
    list.set_class_name("anchor-list");
    for link in &props.links {
        let item = document.create_element("li").expect("create anchor item");
        item.set_class_name("anchor-link-item");
        let entry = document.create_element("a").expect("create anchor link");
        entry.set_class_name("anchor-link");
        entry
            .set_attribute("href", &link.href)
            .expect("target anchor link");
        entry.set_text_content(Some(&link.title));
        listen_for_activation(&entry, link.href.clone());
        item.append_child(&entry).expect("append anchor link");
        list.append_child(&item).expect("append anchor item");
    }
    container.append_child(&list).expect("append anchor list");

    follow_current_section(&container, &props);
    container
}

/// Scroll to the section rather than letting the document jump to it.
fn listen_for_activation(entry: &Element, href: String) {
    let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let Some(section) = section_for(&href) else {
            return;
        };
        event.prevent_default();
        section.scroll_into_view();
        // Keep the keyboard where the reader was sent.
        if !is_focusable(&section) {
            section
                .set_attribute("tabindex", "-1")
                .expect("make section focusable");
        }
        if let Some(target) = section.dyn_ref::<web_sys::HtmlElement>() {
            target.focus().ok();
        }
    });
    entry
        .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        .expect("listen for anchor activation");
    handler.forget();
}

/// Mark the entry whose section the reader has reached.
fn follow_current_section(container: &Element, props: &AnchorProps) {
    let hrefs = props
        .links
        .iter()
        .map(|link| link.href.clone())
        .collect::<Vec<_>>();
    if hrefs.is_empty() {
        return;
    }
    let watched = container.clone();
    let offset = f64::from(props.offset_top);
    follow_scroll(props.target_container.as_deref(), move |target| {
        let current = current_index(&hrefs, target, offset);
        let entries = watched
            .query_selector_all(".anchor-link")
            .expect("query anchor links");
        for index in 0..entries.length() {
            let Some(entry) = entries
                .item(index)
                .and_then(|node| node.dyn_into::<Element>().ok())
            else {
                continue;
            };
            if Some(index as usize) == current {
                entry
                    .set_attribute("data-active", "true")
                    .expect("mark active anchor");
                entry
                    .set_attribute("aria-current", "location")
                    .expect("announce active anchor");
            } else {
                entry
                    .remove_attribute("data-active")
                    .expect("clear active anchor");
                entry
                    .remove_attribute("aria-current")
                    .expect("clear anchor announcement");
            }
        }
    });
}

/// The last section whose top the reader has passed.
///
/// Both sides are measured against the viewport, so this reads the same whether
/// the reader scrolls the window or a panel inside it.
fn current_index(hrefs: &[String], target: &ScrollTarget, offset: f64) -> Option<usize> {
    let line = target.viewport_top() + offset;
    let mut current = None;
    for (index, href) in hrefs.iter().enumerate() {
        let Some(section) = section_for(href) else {
            continue;
        };
        if section.get_bounding_client_rect().top() <= line + 1.0 {
            current = Some(index);
        }
    }
    // Before the first section, the first entry is still the one being read.
    current.or(Some(0))
}

fn section_for(href: &str) -> Option<Element> {
    let id = href.strip_prefix('#')?;
    web_sys::window()?.document()?.get_element_by_id(id)
}

fn is_focusable(element: &Element) -> bool {
    element.has_attribute("tabindex")
        || matches!(
            element.tag_name().to_ascii_lowercase().as_str(),
            "a" | "button" | "input" | "select" | "textarea" | "summary"
        )
}
