//! Domius UI Component Library.
//!
//! This module contains reusable UI components organized by category:
//! - `primitives`: Basic building blocks (Button, Input, Text, Icon, Card, Grid, etc.)
//! - `navigation`: Navigation components (Tabs, Accordion, Drawer, etc.)
//! - `forms`: Form input components (Select, Slider, Switch, etc.)
//! - `data`: Data display components (Table, Badge, Avatar, etc.)
//! - `feedback`: Feedback components (Modal, Toast, Tooltip, Spinner, etc.)
//! - `media`: Media components (VideoPlayer, AudioPlayer)
//! - `pro`: Advanced professional components (DataGrid, Gantt, Kanban, etc.)

pub mod data;
pub mod feedback;
pub mod forms;
pub mod media;
pub mod navigation;
pub mod primitives;
pub mod pro;

// Re-export commonly used components
pub use feedback::modal::{Modal, ModalProps, ModalSize};
pub use feedback::spinner::{spinner, SpinnerProps, SpinnerSize, SpinnerType};
pub use feedback::toast::{Toast, ToastData, ToastManager, ToastProps, ToastVariant};
pub use media::audio_player::{audio_player, AudioPlayerProps};
pub use media::video_player::{video_player, VideoPlayerProps};
pub use primitives::button::{Button, ButtonProps, ButtonSize, ButtonType, ButtonVariant};
pub use primitives::card::{card, CardProps};
pub use primitives::divider::{divider, DividerOrientation, DividerProps};
pub use primitives::grid::{col, row, ColProps, RowProps};
pub use primitives::icon::{Icon, IconName, IconProps};
pub use primitives::input::{Input, InputProps, InputSize, InputType};
pub use primitives::tag::{tag, TagColor, TagProps};
pub use primitives::text::{Text, TextVariant};
pub use primitives::typography::{link, paragraph, text, title, TitleLevel};
