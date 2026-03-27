//! Data display components for Domius UI.
//!
//! Components: DataTable, Charts, Carousel, Badge, AvatarGroup, Timeline, TreeView, Message, Comment, Statistic

pub mod badge;
pub mod avatar;
pub mod timeline;
pub mod table;
pub mod carousel;
pub mod tree_view;
pub mod charts;
pub mod message;
pub mod comment;
pub mod statistic;

// Re-exports
pub use badge::{Badge, BadgeProps, BadgeVariant};
pub use avatar::{Avatar, AvatarProps, AvatarGroup, AvatarGroupProps};
pub use message::{message, chat, MessageProps, MessagePosition, ChatProps};
pub use comment::{comment, comment_list, CommentProps, CommentAction, CommentListProps};
pub use statistic::{statistic, statistic_card, StatisticProps, StatisticCardProps, TrendType};
