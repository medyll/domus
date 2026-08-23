//! Pagination component - Page navigation for data sets.

use std::collections::BTreeSet;
use std::rc::Rc;

use domius_core::effect::create_effect;
use domius_core::signal::{signal, Signal};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;

/// Props for the Pagination component.
pub struct PaginationProps {
    pub total_items: usize,
    pub page_size: usize,
    pub current_page: Option<usize>,
    pub sibling_count: usize,
    pub show_first_last: bool,
    pub show_prev_next: bool,
    pub on_page_change: Option<Box<dyn Fn(usize)>>,
    pub class: Option<String>,
}

impl Default for PaginationProps {
    fn default() -> Self {
        Self {
            total_items: 0,
            page_size: 10,
            current_page: Some(1),
            sibling_count: 1,
            show_first_last: true,
            show_prev_next: true,
            on_page_change: None,
            class: None,
        }
    }
}

/// Pagination component.
pub struct Pagination;

impl Pagination {
    /// Create a pagination element.
    pub fn create(props: PaginationProps) -> (Element, Signal<usize>) {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let navigation = document
            .create_element("nav")
            .expect("create pagination navigation");
        navigation.set_class_name(&props.class.as_deref().map_or_else(
            || "pagination".to_string(),
            |class| format!("pagination {class}"),
        ));
        navigation
            .set_attribute("aria-label", "Pagination")
            .expect("set pagination label");

        let page_size = props.page_size.max(1);
        let total_pages = props.total_items.div_ceil(page_size).max(1);
        let current_page = signal(props.current_page.unwrap_or(1).clamp(1, total_pages));
        navigation
            .set_attribute("data-total-pages", &total_pages.to_string())
            .expect("set page count");

        let callback = props.on_page_change.map(Rc::<dyn Fn(usize)>::from);
        let listener_page = current_page.clone();
        let listener_callback = callback.clone();
        let listener_navigation = navigation.clone();
        let listener_sibling_count = props.sibling_count;
        let listener_show_first_last = props.show_first_last;
        let listener_show_prev_next = props.show_prev_next;
        let handler =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                let Some(control) = event
                    .target()
                    .and_then(|target| target.dyn_into::<Element>().ok())
                else {
                    return;
                };
                if control.has_attribute("disabled") {
                    return;
                }

                let current = listener_page.get();
                let next = match control.get_attribute("data-action").as_deref() {
                    Some("first") => 1,
                    Some("previous") => current.saturating_sub(1).max(1),
                    Some("next") => current.saturating_add(1).min(total_pages),
                    Some("last") => total_pages,
                    Some("page") => control
                        .get_attribute("data-page")
                        .and_then(|page| page.parse().ok())
                        .unwrap_or(current),
                    _ => return,
                };

                if next != current {
                    listener_page.set(next);
                    render_controls(
                        &listener_navigation,
                        next,
                        total_pages,
                        listener_sibling_count,
                        listener_show_first_last,
                        listener_show_prev_next,
                    );
                    if let Some(callback) = listener_callback.as_ref() {
                        callback(next);
                    }
                }
            });
        navigation
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .expect("register pagination callback");
        handler.forget();

        let effect_page = current_page.clone();
        let effect_navigation = navigation.clone();
        create_effect(move || {
            render_controls(
                &effect_navigation,
                effect_page.get(),
                total_pages,
                props.sibling_count,
                props.show_first_last,
                props.show_prev_next,
            );
        });

        (navigation, current_page)
    }
}

fn render_controls(
    navigation: &Element,
    current_page: usize,
    total_pages: usize,
    sibling_count: usize,
    show_first_last: bool,
    show_prev_next: bool,
) {
    let document = navigation
        .owner_document()
        .expect("pagination owner document");
    navigation.set_text_content(None);
    navigation
        .set_attribute("data-current-page", &current_page.to_string())
        .expect("set current page");

    if show_first_last {
        append_control(navigation, "First", "first", 1, current_page == 1);
    }
    if show_prev_next {
        append_control(
            navigation,
            "Previous",
            "previous",
            current_page.saturating_sub(1).max(1),
            current_page == 1,
        );
    }

    let visible_pages = visible_pages(current_page, total_pages, sibling_count);
    let mut previous_page = None;
    for page in visible_pages {
        if previous_page.is_some_and(|previous| page > previous + 1) {
            let ellipsis = document
                .create_element("span")
                .expect("create pagination ellipsis");
            ellipsis.set_class_name("pagination-ellipsis");
            ellipsis
                .set_attribute("aria-hidden", "true")
                .expect("hide pagination ellipsis");
            ellipsis.set_text_content(Some("…"));
            navigation
                .append_child(&ellipsis)
                .expect("append pagination ellipsis");
        }
        append_page(navigation, page, page == current_page);
        previous_page = Some(page);
    }

    if show_prev_next {
        append_control(
            navigation,
            "Next",
            "next",
            current_page.saturating_add(1).min(total_pages),
            current_page == total_pages,
        );
    }
    if show_first_last {
        append_control(
            navigation,
            "Last",
            "last",
            total_pages,
            current_page == total_pages,
        );
    }
}

fn visible_pages(current_page: usize, total_pages: usize, sibling_count: usize) -> BTreeSet<usize> {
    let mut pages = BTreeSet::from([1, total_pages]);
    let start = current_page.saturating_sub(sibling_count).max(1);
    let end = current_page.saturating_add(sibling_count).min(total_pages);
    pages.extend(start..=end);
    pages
}

fn append_control(
    navigation: &Element,
    label: &str,
    action: &str,
    target_page: usize,
    disabled: bool,
) {
    let button = pagination_button(navigation, label);
    button
        .set_attribute("data-action", action)
        .expect("set pagination action");
    button
        .set_attribute("data-page", &target_page.to_string())
        .expect("set target page");
    button
        .set_attribute("aria-label", label)
        .expect("set pagination control label");
    if disabled {
        button
            .set_attribute("disabled", "")
            .expect("disable pagination control");
        button
            .set_attribute("aria-disabled", "true")
            .expect("expose disabled pagination control");
    }
    navigation
        .append_child(&button)
        .expect("append pagination control");
}

fn append_page(navigation: &Element, page: usize, current: bool) {
    let button = pagination_button(navigation, &page.to_string());
    button
        .set_attribute("data-action", "page")
        .expect("set page action");
    button
        .set_attribute("data-page", &page.to_string())
        .expect("set page number");
    button
        .set_attribute("aria-label", &format!("Page {page}"))
        .expect("set page label");
    if current {
        button
            .set_attribute("aria-current", "page")
            .expect("mark current page");
    }
    navigation
        .append_child(&button)
        .expect("append page control");
}

fn pagination_button(navigation: &Element, label: &str) -> Element {
    let button = navigation
        .owner_document()
        .expect("pagination owner document")
        .create_element("button")
        .expect("create pagination button");
    button.set_class_name("btn-sm");
    button
        .set_attribute("type", "button")
        .expect("set pagination button type");
    button.set_text_content(Some(label));
    button
}
