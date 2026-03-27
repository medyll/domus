//! Navigation components for Domius UI.
//!
//! Components: Accordion, Breadcrumbs, Drawer, Pagination, Stepper, Tabs, Navbar, Anchor

pub mod tabs;
pub mod accordion;
pub mod breadcrumbs;
pub mod drawer;
pub mod pagination;
pub mod stepper;
pub mod navbar;
pub mod anchor;

// Re-exports
pub use tabs::{Tabs, TabsProps};
pub use accordion::{Accordion, AccordionProps};
pub use breadcrumbs::{Breadcrumbs, BreadcrumbsProps};
pub use drawer::{Drawer, DrawerProps};
pub use pagination::{Pagination, PaginationProps};
pub use stepper::{Stepper, StepperProps};
pub use navbar::{Navbar, NavbarProps};
pub use anchor::{anchor, AnchorProps, AnchorLink};
