//! Drawer component - Sliding side panel.

use domius_core::signal::Signal;
use web_sys::Element;

/// Drawer position.
#[derive(Clone, PartialEq)]
pub enum DrawerPosition {
    Left,
    Right,
    Top,
    Bottom,
}

impl Default for DrawerPosition {
    fn default() -> Self {
        Self::Left
    }
}

/// Props for the Drawer component.
pub struct DrawerProps {
    pub open: Signal<bool>,
    pub position: DrawerPosition,
    pub title: Option<String>,
    pub closable: bool,
    pub close_on_overlay: bool,
    pub width: Option<String>,
    pub on_close: Option<Box<dyn Fn()>>,
    pub class: Option<String>,
}

impl Default for DrawerProps {
    fn default() -> Self {
        Self {
            open: domius_core::signal::signal(false),
            position: DrawerPosition::default(),
            title: None,
            closable: true,
            close_on_overlay: true,
            width: None,
            on_close: None,
            class: None,
        }
    }
}

/// Drawer component.
pub struct Drawer;

impl Drawer {
    /// Create a drawer element.
    pub fn create(_props: DrawerProps) -> (Element, Signal<bool>) {
        // TODO: Implement drawer
        todo!("Drawer component implementation pending")
    }
}
