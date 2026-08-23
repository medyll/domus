//! TreeSelect component for Domius.
//!
//! A select dropdown with tree structure.

use web_sys::Element;

/// Tree node.
#[derive(Clone)]
pub struct TreeNode {
    pub title: String,
    pub value: String,
    pub children: Option<Vec<TreeNode>>,
    pub disabled: bool,
}

/// TreeSelect props.
#[derive(Clone, Default)]
pub struct TreeSelectProps {
    /// Tree data
    pub tree_data: Option<Vec<TreeNode>>,
    /// CSS class
    pub class: Option<String>,
    /// Placeholder text
    pub placeholder: Option<String>,
    /// Multiple selection
    pub multiple: bool,
    /// Show search box
    pub show_search: bool,
    /// Disabled state
    pub disabled: bool,
    /// Default expanded keys
    pub default_expanded_keys: Option<Vec<String>>,
    /// Show checked strategy
    pub show_checked_strategy: Option<String>,
}

/// Build a TreeSelect component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::treeselect::{treeselect, TreeSelectProps, TreeNode};
///
/// let tree_data = vec![
///     TreeNode {
///         title: "Parent 1".to_string(),
///         value: "parent1".to_string(),
///         children: Some(vec![
///             TreeNode {
///                 title: "Child 1".to_string(),
///                 value: "child1".to_string(),
///                 children: None,
///                 disabled: false,
///             },
///         ]),
///         disabled: false,
///     },
/// ];
///
/// let treeselect_node = treeselect(TreeSelectProps {
///     tree_data: Some(tree_data),
///     placeholder: Some("Select an option".to_string()),
///     ..Default::default()
/// });
/// ```
pub fn treeselect(props: TreeSelectProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("treeselect");
    if props.disabled {
        classes.push_str(" treeselect-disabled");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Select trigger
    let trigger: Element = document.create_element("div").unwrap();
    trigger.set_class_name("treeselect-trigger");

    // Placeholder or selected value
    let value_display: Element = document.create_element("span").unwrap();
    value_display.set_class_name("treeselect-value");
    value_display.set_text_content(Some(
        &props.placeholder.unwrap_or_else(|| "Select...".to_string()),
    ));
    trigger.append_child(&value_display).unwrap();

    // Arrow icon
    let arrow: Element = document.create_element("span").unwrap();
    arrow.set_class_name("treeselect-arrow");
    arrow.set_inner_html("&#9662;"); // ▼ symbol
    trigger.append_child(&arrow).unwrap();

    container.append_child(&trigger).unwrap();

    // Dropdown panel
    let dropdown: Element = document.create_element("div").unwrap();
    dropdown.set_class_name("treeselect-dropdown");
    dropdown.set_attribute("hidden", "").ok();

    // Search box
    if props.show_search {
        let search: Element = document.create_element("input").unwrap();
        search.set_class_name("treeselect-search");
        search.set_attribute("type", "text").ok();
        search.set_attribute("placeholder", "Search...").ok();
        dropdown.append_child(&search).unwrap();
    }

    // Tree content
    let tree: Element = document.create_element("div").unwrap();
    tree.set_class_name("treeselect-tree");

    if let Some(tree_data) = &props.tree_data {
        for node in tree_data {
            let node_el = render_tree_node(&document, node, 0);
            tree.append_child(&node_el).unwrap();
        }
    }

    dropdown.append_child(&tree).unwrap();
    container.append_child(&dropdown).unwrap();

    container
}

fn render_tree_node(document: &web_sys::Document, node: &TreeNode, level: u32) -> Element {
    let node_container: Element = document.create_element("div").unwrap();
    node_container.set_class_name("treeselect-node");
    node_container.set_attribute("data-value", &node.value).ok();

    // Indentation
    let indent: Element = document.create_element("span").unwrap();
    indent.set_class_name("treeselect-indent");
    indent
        .set_attribute("style", &format!("width: {}px;", level * 20))
        .ok();
    node_container.append_child(&indent).unwrap();

    // Expand/collapse icon (if has children)
    if node.children.is_some() {
        let expand_icon: Element = document.create_element("span").unwrap();
        expand_icon.set_class_name("treeselect-expand");
        expand_icon.set_inner_html("&#9658;"); // ► symbol
        node_container.append_child(&expand_icon).unwrap();
    } else {
        let spacer: Element = document.create_element("span").unwrap();
        spacer.set_class_name("treeselect-spacer");
        spacer.set_attribute("style", "width: 16px;").ok();
        node_container.append_child(&spacer).unwrap();
    }

    // Checkbox (if multiple)
    let checkbox: Element = document.create_element("input").unwrap();
    checkbox.set_attribute("type", "checkbox").ok();
    checkbox.set_class_name("treeselect-checkbox");
    if node.disabled {
        checkbox.set_attribute("disabled", "").ok();
    }
    node_container.append_child(&checkbox).unwrap();

    // Node title
    let title: Element = document.create_element("span").unwrap();
    title.set_class_name("treeselect-title");
    title.set_text_content(Some(&node.title));
    if node.disabled {
        title.set_attribute("style", "opacity: 0.5;").ok();
    }
    node_container.append_child(&title).unwrap();

    // Children
    if let Some(children) = &node.children {
        let children_container: Element = document.create_element("div").unwrap();
        children_container.set_class_name("treeselect-children");
        children_container.set_attribute("hidden", "").ok();

        for child in children {
            let child_el = render_tree_node(document, child, level + 1);
            children_container.append_child(&child_el).unwrap();
        }

        node_container.append_child(&children_container).unwrap();
    }

    node_container
}
