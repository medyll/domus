//! Reading and driving scroll position, for the components that follow it.
//!
//! Every one of them can be pointed at a scrolling container instead of the
//! window, so the same primitive works inside a panel and inside a page.

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget};

/// The element a component scrolls against, or the window.
pub enum ScrollTarget {
    Window,
    Container(Element),
}

impl ScrollTarget {
    /// Resolve a selector to its container, falling back to the window.
    pub fn resolve(selector: Option<&str>) -> Self {
        selector
            .and_then(|selector| {
                web_sys::window()?
                    .document()?
                    .query_selector(selector)
                    .ok()
                    .flatten()
            })
            .map_or(Self::Window, Self::Container)
    }

    /// How far the target has been scrolled from its top, in pixels.
    pub fn offset(&self) -> f64 {
        match self {
            Self::Window => web_sys::window()
                .and_then(|window| window.scroll_y().ok())
                .unwrap_or_default(),
            Self::Container(element) => f64::from(element.scroll_top()),
        }
    }

    /// Where the scrollport starts, in viewport coordinates.
    ///
    /// Section positions come from `getBoundingClientRect`, which is measured
    /// against the viewport; this is what makes the two comparable whether the
    /// reader scrolls the window or a panel inside it.
    pub fn viewport_top(&self) -> f64 {
        match self {
            Self::Window => 0.0,
            Self::Container(element) => element.get_bounding_client_rect().top(),
        }
    }

    /// Send the target back to its top.
    pub fn to_top(&self) {
        match self {
            Self::Window => {
                if let Some(window) = web_sys::window() {
                    window.scroll_to_with_x_and_y(0.0, 0.0);
                }
            }
            Self::Container(element) => element.set_scroll_top(0),
        }
    }

    /// What emits the scroll events for this target.
    fn events(&self) -> Option<EventTarget> {
        match self {
            Self::Window => web_sys::window().map(EventTarget::from),
            Self::Container(element) => Some(EventTarget::from(element.clone())),
        }
    }
}

/// Run `on_scroll` now and on every scroll of `target`.
///
/// The listener is leaked deliberately: it lives as long as the document, which
/// is what a component attached to the page wants.
pub fn follow_scroll<F: FnMut(&ScrollTarget) + 'static>(selector: Option<&str>, mut on_scroll: F) {
    let target = ScrollTarget::resolve(selector);
    on_scroll(&target);
    let Some(events) = target.events() else {
        return;
    };
    let owned = selector.map(str::to_string);
    let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        on_scroll(&ScrollTarget::resolve(owned.as_deref()));
    });
    events
        .add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref())
        .expect("listen for scroll");
    handler.forget();
}
