//! RichTextEditor component - WYSIWYG text editor.

use domius_core::signal::Signal;
use web_sys::Element;

/// Props for the RichTextEditor component.
pub struct RichTextEditorProps {
    pub value: Signal<String>,
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub toolbar_options: Vec<String>,
    pub height: Option<String>,
    pub on_change: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for RichTextEditorProps {
    fn default() -> Self {
        Self {
            value: domius_core::signal::signal(String::new()),
            placeholder: None,
            disabled: false,
            toolbar_options: vec!["bold".to_string(), "italic".to_string(), "underline".to_string()],
            height: None,
            on_change: None,
            class: None,
        }
    }
}

/// RichTextEditor component.
pub struct RichTextEditor;

impl RichTextEditor {
    /// Create a rich text editor element.
    pub fn create(_props: RichTextEditorProps) -> (Element, Signal<String>) {
        // TODO: Implement rich text editor
        todo!("RichTextEditor component implementation pending")
    }
}
