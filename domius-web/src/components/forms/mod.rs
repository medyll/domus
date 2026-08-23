//! Form input components for Domius UI.
//!
//! Components: Autocomplete, DatePicker, MultiSelect, Slider, Switch,
//! RichTextEditor, FileUploader, InputMask, Upload, TreeSelect

pub mod autocomplete;
pub mod date_picker;
pub mod file_uploader;
pub mod input_mask;
pub mod multi_select;
pub mod rich_text;
pub mod select;
pub mod slider;
pub mod switch;
pub mod treeselect;
pub mod upload;

// Re-exports
pub use autocomplete::{Autocomplete, AutocompleteProps};
pub use date_picker::{DatePicker, DatePickerProps};
pub use file_uploader::{FileUploader, FileUploaderProps};
pub use multi_select::{MultiSelect, MultiSelectProps};
pub use select::{Select, SelectProps};
pub use slider::{Slider, SliderProps};
pub use switch::{Switch, SwitchProps};
pub use treeselect::{treeselect, TreeNode, TreeSelectProps};
pub use upload::{upload, UploadProps};
