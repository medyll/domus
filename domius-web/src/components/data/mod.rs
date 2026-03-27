//! Data display components for Domius UI.
//!
//! Components: DataTable, Charts, Carousel, Badge, AvatarGroup, Timeline, TreeView

pub mod badge;
pub mod avatar;
pub mod timeline;
pub mod table;
pub mod carousel;
pub mod tree_view;
pub mod charts;

// Re-exports
pub use badge::{Badge, BadgeProps, BadgeVariant};
pub use avatar::{Avatar, AvatarProps, AvatarGroup, AvatarGroupProps};
