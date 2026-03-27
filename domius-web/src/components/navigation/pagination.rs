//! Pagination component - Page navigation for data sets.

use domius_core::signal::Signal;
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
    pub fn create(_props: PaginationProps) -> (Element, Signal<usize>) {
        // TODO: Implement pagination
        todo!("Pagination component implementation pending")
    }
}
