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
    pub fn create(props: BreadcrumbsProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let navigation = document
            .create_element("nav")
            .expect("create breadcrumbs navigation");
        navigation
            .set_attribute("aria-label", "Breadcrumb")
            .expect("set breadcrumbs label");
        let mut classes = vec!["domius-breadcrumbs"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        navigation.set_class_name(&classes.join(" "));

        let list = document
            .create_element("ol")
            .expect("create breadcrumbs list");
        let separator = props.separator.as_deref().unwrap_or("/");
        let item_count = props.items.len();

        for (index, item) in props.items.into_iter().enumerate() {
            let list_item = document
                .create_element("li")
                .expect("create breadcrumb item");
            list_item.set_class_name("domius-breadcrumbs-item");
            let is_current = index + 1 == item_count;
            let content = if let Some(href) = item.href.filter(|_| !item.disabled && !is_current) {
                let link = document
                    .create_element("a")
                    .expect("create breadcrumb link");
                link.set_attribute("href", &href)
                    .expect("set breadcrumb target");
                link
            } else {
                document
                    .create_element("span")
                    .expect("create breadcrumb label")
            };
            content.set_class_name("domius-breadcrumbs-link");
            content.set_text_content(Some(&item.label));
            if item.disabled {
                content
                    .set_attribute("aria-disabled", "true")
                    .expect("disable breadcrumb");
            }
            if is_current {
                content
                    .set_attribute("aria-current", "page")
                    .expect("mark current breadcrumb");
            }
            list_item
                .append_child(&content)
                .expect("append breadcrumb content");

            if !is_current {
                let separator_element = document
                    .create_element("span")
                    .expect("create breadcrumb separator");
                separator_element.set_class_name("domius-breadcrumbs-separator");
                separator_element
                    .set_attribute("aria-hidden", "true")
                    .expect("hide breadcrumb separator");
                separator_element.set_text_content(Some(separator));
                list_item
                    .append_child(&separator_element)
                    .expect("append breadcrumb separator");
            }
            list.append_child(&list_item)
                .expect("append breadcrumb item");
        }

        navigation
            .append_child(&list)
            .expect("append breadcrumbs list");
        navigation
    }
}
