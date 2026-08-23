//! Automatic scope disposal via MutationObserver.
//!
//! Watches the document for removed DOM nodes. When a node with a
//! `data-domius-scope` attribute is removed, the corresponding scope is
//! disposed, unsubscribing all reactive effects and freeing memory.
//!
//! # Usage
//!
//! Call [`init_disposal_observer`] once from your `wasm_bindgen(start)` entry:
//!
//! ```ignore
//! #[wasm_bindgen(start)]
//! pub fn main() {
//!     domius_web::init(); // calls init_disposal_observer internally
//! }
//! ```
//!
//! # DOM contract
//!
//! Components that create a scope must set the `data-domius-scope` attribute on
//! their root element to the scope's numeric ID. The observer reads this
//! attribute on removal and calls `dispose_scope`.

#[cfg(target_arch = "wasm32")]
mod wasm {
    use domius_core::scope::{dispose_scope, ScopeId};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{MutationObserver, MutationObserverInit, MutationRecord, NodeList};

    /// Initialise the document-level MutationObserver.
    ///
    /// Safe to call multiple times (each call installs one observer).
    pub fn init_disposal_observer() {
        let callback = Closure::wrap(Box::new(
            move |mutations: js_sys::Array, _observer: MutationObserver| {
                for item in mutations.iter() {
                    let record: MutationRecord = item.unchecked_into();
                    let removed: NodeList = record.removed_nodes();
                    for i in 0..removed.length() {
                        if let Some(node) = removed.get(i) {
                            if let Some(element) = node.dyn_ref::<web_sys::Element>() {
                                try_dispose_element(element);
                                // Also check descendants with data-domius-scope
                                if let Ok(scoped) =
                                    element.query_selector_all("[data-domius-scope]")
                                {
                                    for j in 0..scoped.length() {
                                        if let Some(child) = scoped.get(j) {
                                            if let Some(el) = child.dyn_ref::<web_sys::Element>() {
                                                try_dispose_element(el);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
        ) as Box<dyn Fn(js_sys::Array, MutationObserver)>);

        let observer = MutationObserver::new(callback.as_ref().unchecked_ref())
            .expect("MutationObserver::new failed");
        callback.forget(); // keep alive for document lifetime

        let options = MutationObserverInit::new();
        options.set_subtree(true);
        options.set_child_list(true);

        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        observer
            .observe_with_options(&document, &options)
            .expect("observer.observe failed");
    }

    fn try_dispose_element(element: &web_sys::Element) {
        if let Some(scope_str) = element.get_attribute("data-domius-scope") {
            if let Ok(scope_id) = scope_str.parse::<usize>() {
                dispose_scope(ScopeId::from_numeric(scope_id));
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::init_disposal_observer;

/// No-op stub for non-WASM targets (native tests, CI).
#[cfg(not(target_arch = "wasm32"))]
pub fn init_disposal_observer() {}
