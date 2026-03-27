//! Upload component for Domius.
//!
//! File upload with drag and drop support.

use web_sys::Element;

/// Upload props.
#[derive(Clone, Default)]
pub struct UploadProps {
    /// Upload URL
    pub action: Option<String>,
    /// CSS class
    pub class: Option<String>,
    /// Multiple file selection
    pub multiple: bool,
    /// Accept file types
    pub accept: Option<String>,
    /// Show drag area
    pub drag: bool,
    /// Max file size in bytes
    pub max_size: Option<u32>,
    /// File list
    pub file_list: Option<String>,
}

/// Build an Upload component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::upload::{upload, UploadProps};
///
/// let upload_node = upload(UploadProps {
///     action: Some("/api/upload".to_string()),
///     multiple: true,
///     drag: true,
///     ..Default::default()
/// });
/// ```
pub fn upload(props: UploadProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let container: Element = document.create_element("div").unwrap();
    
    let mut classes = String::from("upload");
    if props.drag {
        classes.push_str(" upload-drag");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Input element
    let input: Element = document.create_element("input").unwrap();
    input.set_attribute("type", "file").ok();
    input.set_class_name("upload-input");
    
    if props.multiple {
        input.set_attribute("multiple", "").ok();
    }
    if let Some(accept) = &props.accept {
        input.set_attribute("accept", accept).ok();
    }
    
    container.append_child(&input).unwrap();

    // Drag area
    if props.drag {
        let drag_area: Element = document.create_element("div").unwrap();
        drag_area.set_class_name("upload-drag-area");
        
        let icon: Element = document.create_element("div").unwrap();
        icon.set_class_name("upload-icon");
        icon.set_inner_html("&#8593;"); // ↑ symbol
        drag_area.append_child(&icon).unwrap();
        
        let text: Element = document.create_element("p").unwrap();
        text.set_class_name("upload-text");
        text.set_text_content(Some("Click or drag file to this area to upload"));
        drag_area.append_child(&text).unwrap();
        
        container.append_child(&drag_area).unwrap();
    }

    // File list placeholder
    let file_list: Element = document.create_element("div").unwrap();
    file_list.set_class_name("upload-file-list");
    container.append_child(&file_list).unwrap();

    container
}
