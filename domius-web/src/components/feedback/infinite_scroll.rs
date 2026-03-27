//! InfiniteScroll component - Auto-loading content on scroll.

use domius_core::signal::{signal, Signal};
use web_sys::Element;

/// Props for the InfiniteScroll component.
pub struct InfiniteScrollProps {
    pub children: Element,
    pub has_more: bool,
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
                .unwrap()
                .into(),
            has_more: true,
            loading: signal(false),
            threshold: 100,
            on_load_more: Box::new(|| {}),
            reverse: false,
            class: None,
        }
    }
}

/// InfiniteScroll component.
pub struct InfiniteScroll;

impl InfiniteScroll {
    /// Create an infinite scroll wrapper element.
    pub fn create(_props: InfiniteScrollProps) -> Element {
        // TODO: Implement infinite scroll with IntersectionObserver
        todo!("InfiniteScroll component implementation pending")
    }
}
