//! Navigation components for Domius UI.
//!
//! Components: Accordion, Breadcrumbs, Drawer, Pagination, Stepper, Tabs, Navbar, Anchor

pub mod accordion;
pub mod anchor;
pub mod breadcrumbs;
pub mod drawer;
pub mod navbar;
pub mod pagination;
pub mod stepper;
pub mod tabs;

// Re-exports
pub use accordion::{Accordion, AccordionProps};
pub use anchor::{anchor, AnchorLink, AnchorProps};
pub use breadcrumbs::{Breadcrumbs, BreadcrumbsProps};
pub use drawer::{Drawer, DrawerProps};
pub use navbar::{Navbar, NavbarProps};
pub use pagination::{Pagination, PaginationProps};
pub use stepper::{Stepper, StepperProps};
pub use tabs::{Tabs, TabsProps};
