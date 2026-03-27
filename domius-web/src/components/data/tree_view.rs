//! TreeView component - Hierarchical data display.

use domius_core::signal::Signal;
use web_sys::Element;

/// A tree node.
#[derive(Clone)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeNode>,
    pub expanded: bool,
    pub selectable: bool,
    pub icon: Option<String>,
    pub disabled: bool,
}

/// Props for the TreeView component.
pub struct TreeViewProps {
    pub nodes: Vec<TreeNode>,
    pub selectable: bool,
    pub multi_select: bool,
    pub expandable: bool,
    pub default_expanded: bool,
    pub selected_ids: Option<Signal<Vec<String>>>,
    pub on_select: Option<Box<dyn Fn(String)>>,
    pub on_expand: Option<Box<dyn Fn(String, bool)>>,
    pub class: Option<String>,
}

impl Default for TreeViewProps {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            selectable: true,
            multi_select: false,
            expandable: true,
            default_expanded: false,
            selected_ids: None,
            on_select: None,
            on_expand: None,
            class: None,
        }
    }
}

/// TreeView component.
pub struct TreeView;

impl TreeView {
    /// Create a tree view element.
    pub fn create(_props: TreeViewProps) -> (Element, Option<Signal<Vec<String>>>) {
        // TODO: Implement tree view
        todo!("TreeView component implementation pending")
    }
}
