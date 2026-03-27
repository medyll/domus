//! Mention component - Auto-complete with trigger character (@, #).

use domius_core::signal::Signal;
use web_sys::Element;

/// Mention item.
#[derive(Clone)]
pub struct MentionItem {
    pub value: String,
    pub label: String,
    pub avatar: Option<String>,
}

/// Props for the Mention component.
pub struct MentionProps {
    pub trigger: char,
    pub items: Vec<MentionItem>,
    pub value: Signal<String>,
    pub placeholder: Option<String>,
    pub on_select: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for MentionProps {
    fn default() -> Self {
        Self {
            trigger: '@',
            items: Vec::new(),
            value: domius_core::signal::signal(String::new()),
            placeholder: None,
            on_select: None,
            class: None,
        }
    }
}

/// Mention component.
pub struct Mention;

impl Mention {
    /// Create a mention input element.
    pub fn create(_props: MentionProps) -> (Element, Signal<String>) {
        // TODO: Implement mention with popup suggestions
        todo!("Mention component implementation pending")
    }
}
