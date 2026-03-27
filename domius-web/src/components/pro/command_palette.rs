//! CommandPalette component - Quick action search (Cmd+K).

use domius_core::signal::Signal;
use web_sys::Element;

/// Command item.
pub struct Command {
    pub id: String,
    pub label: String,
    pub shortcut: Option<String>,
    pub icon: Option<String>,
    pub section: Option<String>,
    pub action: Box<dyn Fn()>,
}

/// Props for the CommandPalette component.
pub struct CommandPaletteProps {
    pub commands: Vec<Command>,
    pub open: Signal<bool>,
    pub placeholder: Option<String>,
    pub trigger_key: String,
    pub on_select: Option<Box<dyn Fn(String)>>,
    pub on_open_change: Option<Box<dyn Fn(bool)>>,
    pub class: Option<String>,
}

impl Default for CommandPaletteProps {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            open: domius_core::signal::signal(false),
            placeholder: Some("Type a command or search...".to_string()),
            trigger_key: "k".to_string(),
            on_select: None,
            on_open_change: None,
            class: None,
        }
    }
}

/// CommandPalette component.
pub struct CommandPalette;

impl CommandPalette {
    /// Create a command palette element.
    pub fn create(_props: CommandPaletteProps) -> Element {
        // TODO: Implement command palette with fuzzy search
        todo!("CommandPalette component implementation pending")
    }
}
