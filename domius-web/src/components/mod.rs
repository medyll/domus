//! Domius UI Component Library.
//!
//! This module contains reusable UI components organized by category:
//! - `primitives`: Basic building blocks (Button, Input, Text, Icon)
//! - `navigation`: Navigation components (Tabs, Accordion, Drawer, etc.)
//! - `forms`: Form input components (Select, Slider, Switch, etc.)
//! - `data`: Data display components (Table, Badge, Avatar, etc.)
//! - `feedback`: Feedback components (Modal, Toast, Tooltip, etc.)
//! - `pro`: Advanced professional components (DataGrid, Gantt, Kanban, etc.)

pub mod primitives;
pub mod navigation;
pub mod forms;
pub mod data;
pub mod feedback;
pub mod pro;

// Re-export commonly used components
pub use primitives::button::{Button, ButtonProps, ButtonVariant, ButtonSize};
pub use primitives::input::{Input, InputProps, InputType};
pub use primitives::text::{Text, TextProps, TextVariant};
pub use primitives::icon::{Icon, IconProps, IconName};
