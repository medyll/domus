//! Transfer component - Dual list selection.

use domius_core::signal::Signal;
use web_sys::Element;

/// Transfer item.
#[derive(Clone)]
pub struct TransferItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

/// Props for the Transfer component.
pub struct TransferProps {
    pub source: Vec<TransferItem>,
    pub target: Vec<TransferItem>,
    pub titles: (Option<String>, Option<String>),
    pub searchable: bool,
    pub on_change: Option<Box<dyn Fn(Vec<String>, Vec<String>)>>,
    pub class: Option<String>,
}

impl Default for TransferProps {
    fn default() -> Self {
        Self {
            source: Vec::new(),
            target: Vec::new(),
            titles: (None, None),
            searchable: true,
            on_change: None,
            class: None,
        }
    }
}

/// Transfer component.
pub struct Transfer;

impl Transfer {
    /// Create a transfer element.
    pub fn create(_props: TransferProps) -> Element {
        // TODO: Implement transfer (dual list with move operations)
        todo!("Transfer component implementation pending")
    }
}
