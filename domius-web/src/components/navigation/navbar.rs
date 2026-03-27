//! Navbar component - Top navigation bar.

use web_sys::Element;

/// A navigation link.
#[derive(Clone)]
pub struct NavLink {
    pub label: String,
    pub href: Option<String>,
    pub active: bool,
    pub icon: Option<String>,
    pub children: Vec<NavLink>,
}

/// Props for the Navbar component.
pub struct NavbarProps {
    pub logo: Option<String>,
    pub logo_text: Option<String>,
    pub links: Vec<NavLink>,
    pub actions: Option<String>,
    pub fixed: bool,
    pub transparent: bool,
    pub on_link_click: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for NavbarProps {
    fn default() -> Self {
        Self {
            logo: None,
            logo_text: None,
            links: Vec::new(),
            actions: None,
            fixed: false,
            transparent: false,
            on_link_click: None,
            class: None,
        }
    }
}

/// Navbar component.
pub struct Navbar;

impl Navbar {
    /// Create a navbar element.
    pub fn create(_props: NavbarProps) -> Element {
        // TODO: Implement navbar
        todo!("Navbar component implementation pending")
    }
}
