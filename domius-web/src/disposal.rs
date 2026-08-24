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
//! attribute on removal and calls `dispose_scope`. Use [`ViewScope`] rather
//! than writing the attribute by hand: it holds the two halves together.

use domius_core::scope::{create_effect_in_scope, create_scope, dispose_scope, ScopeId};
use web_sys::Element;

/// Attribute the observer reads to find the scope a removed element owned.
pub const SCOPE_ATTRIBUTE: &str = "data-domius-scope";

/// A scope tied to the element that owns it.
///
/// Creating effects through a `ViewScope` rather than `create_effect` is what
/// makes them stop when their element leaves the document: the scope id is
/// stamped on the element, and the disposal observer reads it back on removal.
///
/// ```ignore
/// let scope = ViewScope::attach(&root);
/// scope.effect(move || render(count.get()));
/// ```
pub struct ViewScope {
    id: ScopeId,
}

impl ViewScope {
    /// Create a scope and mark `root` as its owner.
    pub fn attach(root: &Element) -> Self {
        Self::attach_within(root, None)
    }

    /// Create a scope nested inside `parent`, and mark `root` as its owner.
    pub fn attach_within(root: &Element, parent: Option<ScopeId>) -> Self {
        let id = create_scope(parent);
        root.set_attribute(SCOPE_ATTRIBUTE, &id.value().to_string())
            .expect("stamp scope on view root");
        Self { id }
    }

    /// This scope's id, as stamped on its element.
    pub fn id(&self) -> ScopeId {
        self.id
    }

    /// Run `f` now and on every change, until the scope is disposed.
    pub fn effect<F: FnMut() + 'static>(&self, f: F) {
        create_effect_in_scope(self.id, f).expect("scope should still be alive");
    }

    /// Stop every effect in this scope without waiting for the observer.
    pub fn dispose(self) {
        dispose_scope(self.id);
    }
}

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
        if let Some(scope_str) = element.get_attribute(super::SCOPE_ATTRIBUTE) {
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
