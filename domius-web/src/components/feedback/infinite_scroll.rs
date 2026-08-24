//! Infinite scroll with an accessible manual fallback.

use std::rc::Rc;

use domius_core::effect::create_effect;
use domius_core::signal::{signal, Signal};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

pub struct InfiniteScrollProps {
    pub children: Element,
    pub has_more: Signal<bool>,
    pub loading: Signal<bool>,
    pub threshold: usize,
    pub on_load_more: Box<dyn Fn()>,
    pub reverse: bool,
    pub class: Option<String>,
}

impl Default for InfiniteScrollProps {
    fn default() -> Self {
        Self {
            children: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap(),
            has_more: signal(true),
            loading: signal(false),
            threshold: 100,
            on_load_more: Box::new(|| {}),
            reverse: false,
            class: None,
        }
    }
}

pub struct InfiniteScroll;

impl InfiniteScroll {
    pub fn create(props: InfiniteScrollProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let root = document
            .create_element("section")
            .expect("create infinite scroll");
        let mut classes = vec!["domius-infinite-scroll"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        root.set_class_name(&classes.join(" "));
        root.set_attribute(
            "data-direction",
            if props.reverse { "reverse" } else { "forward" },
        )
        .expect("set scroll direction");
        root.set_attribute("data-threshold", &props.threshold.to_string())
            .expect("set scroll threshold");

        let sentinel = document
            .create_element("div")
            .expect("create infinite scroll sentinel");
        sentinel.set_class_name("domius-infinite-scroll-sentinel");
        sentinel
            .set_attribute("aria-hidden", "true")
            .expect("hide scroll sentinel");

        if props.reverse {
            root.append_child(&sentinel)
                .expect("append scroll sentinel");
            root.append_child(&props.children)
                .expect("append infinite scroll content");
        } else {
            root.append_child(&props.children)
                .expect("append infinite scroll content");
            root.append_child(&sentinel)
                .expect("append scroll sentinel");
        }

        let status = document
            .create_element("p")
            .expect("create infinite scroll status");
        status.set_class_name("domius-infinite-scroll-status");
        status
            .set_attribute("role", "status")
            .expect("set status role");
        status
            .set_attribute("aria-live", "polite")
            .expect("set status live region");

        let load_button = document
            .create_element("button")
            .expect("create load more button");
        load_button.set_class_name("btn-sm");
        load_button
            .set_attribute("type", "button")
            .expect("set load button type");
        load_button.set_text_content(Some("Load more"));
        root.append_child(&load_button).expect("append load button");
        root.append_child(&status).expect("append scroll status");

        let callback = Rc::<dyn Fn()>::from(props.on_load_more);
        let request_loading = props.loading.clone();
        let request_has_more = props.has_more.clone();
        let request_callback = Rc::clone(&callback);
        let request_root = root.clone();
        let request_status = status.clone();
        let request_button = load_button.clone();
        let request_load: Rc<dyn Fn()> = Rc::new(move || {
            if request_has_more.get() && !request_loading.get() {
                request_loading.set(true);
                apply_loading_state(&request_root, &request_status, &request_button, true, true);
                request_callback();
            }
        });

        let click_request = Rc::clone(&request_load);
        let handler = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_| {
            click_request();
        });
        load_button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .expect("register load button");
        handler.forget();

        observe_sentinel(&sentinel, props.threshold, Rc::clone(&request_load));

        let effect_loading = props.loading.clone();
        let effect_root = root.clone();
        let effect_status = status.clone();
        let effect_button = load_button.clone();
        let effect_has_more = props.has_more.clone();
        create_effect(move || {
            let loading = effect_loading.get();
            let has_more = effect_has_more.get();
            apply_loading_state(
                &effect_root,
                &effect_status,
                &effect_button,
                loading,
                has_more,
            );
        });

        root
    }
}

fn apply_loading_state(
    root: &Element,
    status: &Element,
    button: &Element,
    loading: bool,
    has_more: bool,
) {
    root.set_attribute("aria-busy", &loading.to_string())
        .expect("set infinite scroll busy state");
    root.set_attribute("data-loading", &loading.to_string())
        .expect("set infinite scroll loading state");
    status.set_text_content(Some(if loading {
        "Loading more items"
    } else if has_more {
        "More items load automatically"
    } else {
        "All items loaded"
    }));
    if !has_more {
        button
            .set_attribute("hidden", "")
            .expect("hide completed load button");
        button
            .set_attribute("disabled", "")
            .expect("disable completed load button");
        button
            .set_attribute("aria-disabled", "true")
            .expect("expose completed load button");
    } else {
        button.remove_attribute("hidden").expect("show load button");
        if loading {
            button
                .set_attribute("disabled", "")
                .expect("disable load button");
            button
                .set_attribute("aria-disabled", "true")
                .expect("expose disabled load button");
        } else {
            button
                .remove_attribute("disabled")
                .expect("enable load button");
            button
                .remove_attribute("aria-disabled")
                .expect("expose enabled load button");
        }
    }
}

fn observe_sentinel(sentinel: &Element, threshold: usize, request_load: Rc<dyn Fn()>) {
    let callback = Closure::<dyn FnMut(js_sys::Array, IntersectionObserver)>::new(
        move |entries: js_sys::Array, _observer: IntersectionObserver| {
            let intersects = entries.iter().any(|entry| {
                entry
                    .dyn_into::<IntersectionObserverEntry>()
                    .is_ok_and(|entry| entry.is_intersecting())
            });
            if intersects {
                request_load();
            }
        },
    );
    let options = IntersectionObserverInit::new();
    options.set_root_margin(&format!("0px 0px {threshold}px 0px"));
    let observer =
        IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
            .expect("create intersection observer");
    observer.observe(sentinel);
    callback.forget();
    std::mem::forget(observer);
}
