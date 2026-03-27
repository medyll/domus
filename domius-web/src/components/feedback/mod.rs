//! Feedback components for Domius UI.
//!
//! Components: Modal, Toast, Tooltip, Popover, Skeleton, ProgressBar, InfiniteScroll

pub mod modal;
pub mod toast;
pub mod tooltip;
pub mod popover;
pub mod skeleton;
pub mod progress;
pub mod infinite_scroll;

// Re-exports
pub use modal::{Modal, ModalProps};
pub use toast::{Toast, ToastProps, ToastVariant, ToastManager};
pub use tooltip::{Tooltip, TooltipProps};
pub use skeleton::{Skeleton, SkeletonProps, SkeletonVariant};
pub use progress::{ProgressBar, ProgressProps, ProgressVariant};
