//! FileUploader component - Drag and drop file upload.

use domius_core::signal::Signal;
use web_sys::Element;

/// Props for the FileUploader component.
pub struct FileUploaderProps {
    pub accept: Option<String>,
    pub multiple: bool,
    pub max_size: Option<u64>,
    pub max_files: Option<usize>,
    pub drag_drop: bool,
    pub show_preview: bool,
    pub on_upload: Option<Box<dyn Fn(Vec<String>)>>,
    pub class: Option<String>,
}

impl Default for FileUploaderProps {
    fn default() -> Self {
        Self {
            accept: None,
            multiple: false,
            max_size: None,
            max_files: None,
            drag_drop: true,
            show_preview: true,
            on_upload: None,
            class: None,
        }
    }
}

/// FileUploader component.
pub struct FileUploader;

impl FileUploader {
    /// Create a file uploader element.
    pub fn create(_props: FileUploaderProps) -> Element {
        // TODO: Implement file uploader
        todo!("FileUploader component implementation pending")
    }
}
