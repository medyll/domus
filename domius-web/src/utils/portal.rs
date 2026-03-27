//! Portal utility for rendering components outside the normal DOM hierarchy.
//!
//! Useful for modals, tooltips, and popovers that need to escape overflow:hidden containers.

use web_sys::{Document, Element, Node};

/// A portal that renders content to a target element (default: document body).
///
/// # Example
/// ```ignore
/// let portal = Portal::new();
/// let content = document.create_element("div").unwrap();
/// portal.mount(&content);
///
/// // Later, to unmount:
/// portal.unmount();
/// ```
pub struct Portal {
    target: Element,
    content: Option<Node>,
}

impl Portal {
    /// Create a new portal that renders to the document body.
    pub fn new() -> Self {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        
        let target = document
            .query_selector("body")
            .ok()
            .flatten()
            .unwrap_or_else(|| {
                document.create_element("body").unwrap()
            });
        
        Self {
            target,
            content: None,
        }
    }

    /// Create a new portal that renders to a specific target element.
    pub fn with_target(target: Element) -> Self {
        Self {
            target,
            content: None,
        }
    }

    /// Create a new portal that renders to an element by ID.
    pub fn by_id(id: &str) -> Option<Self> {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        
        document.get_element_by_id(id).map(|target| Self {
            target,
            content: None,
        })
    }

    /// Mount content to the portal target.
    pub fn mount(&mut self, content: &Node) {
        // Unmount existing content first
        self.unmount();
        
        let cloned = content.clone_node_with_deep(true).ok().unwrap();
        self.target.append_child(&cloned).ok();
        self.content = Some(cloned);
    }

    /// Unmount content from the portal target.
    pub fn unmount(&mut self) {
        if let Some(content) = self.content.take() {
            let _ = self.target.remove_child(&content);
        }
    }

    /// Get the target element.
    pub fn target(&self) -> &Element {
        &self.target
    }

    /// Get the mounted content.
    pub fn content(&self) -> Option<&Node> {
        self.content.as_ref()
    }
}

impl Default for Portal {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Portal {
    fn drop(&mut self) {
        self.unmount();
    }
}

/// Get or create a portal container element by ID.
///
/// This is useful for creating a dedicated portal root that persists
/// across multiple portal instances.
pub fn get_portal_container(id: &str) -> Element {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    
    if let Some(existing) = document.get_element_by_id(id) {
        return existing;
    }
    
    // Create new container
    let container = document.create_element("div").unwrap();
    container.set_id(id);
    
    // Style the container
    container.set_attribute("style", "
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;
        z-index: 9999;
    ").ok();
    
    // Make children interactive
    container.set_attribute("data-portal", "true").ok();
    
    document
        .query_selector("body")
        .ok()
        .flatten()
        .expect("no body")
        .append_child(&container)
        .expect("failed to append portal container");
    
    container
}

/// Ensure portal containers exist in the document.
pub fn init_portal_containers() {
    let _ = get_portal_container("domius-portal-root");
    let _ = get_portal_container("domius-modal-root");
    let _ = get_portal_container("domius-toast-root");
}
