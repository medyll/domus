//! Feedback components for Domius UI.
//!
//! Components: Modal, Toast, Tooltip, Popover, Skeleton, ProgressBar, InfiniteScroll, Spinner

pub mod infinite_scroll;
pub mod modal;
pub mod popover;
pub mod progress;
pub mod skeleton;
pub mod spinner;
pub mod toast;
pub mod tooltip;

// Re-exports
pub use modal::{Modal, ModalProps, ModalSize};
pub use progress::{ProgressBar, ProgressProps, ProgressVariant};
pub use skeleton::{Skeleton, SkeletonProps, SkeletonVariant};
pub use spinner::{spinner, SpinnerProps, SpinnerSize, SpinnerType};
pub use toast::{Toast, ToastData, ToastManager, ToastProps, ToastVariant};
pub use tooltip::{Tooltip, TooltipProps};
