//! Breadcrumbs component - Navigation hierarchy indicator.

use web_sys::Element;

/// A single breadcrumb item.
#[derive(Clone)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
    pub disabled: bool,
}

/// Props for the Breadcrumbs component.
#[derive(Clone)]
pub struct BreadcrumbsProps {
    pub items: Vec<BreadcrumbItem>,
    pub separator: Option<String>,
    pub class: Option<String>,
}

impl Default for BreadcrumbsProps {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            separator: Some("/".to_string()),
            class: None,
        }
    }
}

/// Breadcrumbs component.
pub struct Breadcrumbs;

impl Breadcrumbs {
    /// Create a breadcrumbs element.
    pub fn create(_props: BreadcrumbsProps) -> Element {
        // TODO: Implement breadcrumbs
        todo!("Breadcrumbs component implementation pending")
    }
}
