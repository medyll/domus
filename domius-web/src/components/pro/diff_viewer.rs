//! DiffViewer component - Code/text comparison.

use web_sys::Element;

/// Diff line type.
#[derive(Clone, PartialEq)]
pub enum DiffLineType {
    Unchanged,
    Added,
    Removed,
}

/// Diff line.
#[derive(Clone)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub old_line: Option<usize>,
    pub new_line: Option<usize>,
    pub content: String,
}

/// Diff output.
#[derive(Clone)]
pub struct DiffOutput {
    pub lines: Vec<DiffLine>,
    pub additions: usize,
    pub deletions: usize,
}

/// Props for the DiffViewer component.
#[derive(Clone)]
pub struct DiffViewerProps {
    pub old_text: String,
    pub new_text: String,
    pub mode: DiffMode,
    pub show_line_numbers: bool,
    pub language: Option<String>,
    pub class: Option<String>,
}

/// Diff display mode.
#[derive(Clone, PartialEq)]
pub enum DiffMode {
    Unified,
    Split,
    Word,
}

impl Default for DiffViewerProps {
    fn default() -> Self {
        Self {
            old_text: String::new(),
            new_text: String::new(),
            mode: DiffMode::Unified,
            show_line_numbers: true,
            language: None,
            class: None,
        }
    }
}

/// DiffViewer component.
pub struct DiffViewer;

impl DiffViewer {
    /// Create a diff viewer element.
    pub fn create(_props: DiffViewerProps) -> Element {
        // TODO: Implement diff viewer (would need a diff algorithm)
        todo!("DiffViewer component implementation pending")
    }
}
