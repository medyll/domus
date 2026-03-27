//! Form input components for Domius UI.
//!
//! Components: Autocomplete, DatePicker, MultiSelect, Slider, Switch,
//! RichTextEditor, FileUploader, InputMask, Upload, TreeSelect

pub mod switch;
pub mod slider;
pub mod select;
pub mod autocomplete;
pub mod date_picker;
pub mod multi_select;
pub mod file_uploader;
pub mod input_mask;
pub mod rich_text;
pub mod upload;
pub mod treeselect;

// Re-exports
pub use switch::{Switch, SwitchProps};
pub use slider::{Slider, SliderProps};
pub use select::{Select, SelectProps};
pub use autocomplete::{Autocomplete, AutocompleteProps};
pub use date_picker::{DatePicker, DatePickerProps};
pub use multi_select::{MultiSelect, MultiSelectProps};
pub use file_uploader::{FileUploader, FileUploaderProps};
pub use upload::{upload, UploadProps};
pub use treeselect::{treeselect, TreeSelectProps, TreeNode};
