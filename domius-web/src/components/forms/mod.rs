//! Form input components for Domius UI.
//!
//! Components: Autocomplete, DatePicker, MultiSelect, Slider, Switch, 
//! RichTextEditor, FileUploader, InputMask

pub mod switch;
pub mod slider;
pub mod select;
pub mod autocomplete;
pub mod date_picker;
pub mod multi_select;
pub mod file_uploader;
pub mod input_mask;
pub mod rich_text;

// Re-exports
pub use switch::{Switch, SwitchProps};
pub use slider::{Slider, SliderProps};
