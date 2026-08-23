//! Data display components for Domius UI.
//!
//! Components: DataTable, Charts, Carousel, Badge, AvatarGroup, Timeline, TreeView, Message, Comment, Statistic

pub mod avatar;
pub mod badge;
pub mod carousel;
pub mod charts;
pub mod comment;
pub mod message;
pub mod statistic;
pub mod table;
pub mod timeline;
pub mod tree_view;

// Re-exports
pub use avatar::{Avatar, AvatarGroup, AvatarGroupProps, AvatarProps};
pub use badge::{Badge, BadgeProps, BadgeVariant};
pub use comment::{comment, comment_list, CommentAction, CommentListProps, CommentProps};
pub use message::{chat, message, ChatProps, MessagePosition, MessageProps};
pub use statistic::{statistic, statistic_card, StatisticCardProps, StatisticProps, TrendType};
