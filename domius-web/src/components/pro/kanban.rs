//! KanbanBoard component - Task management board.

use domius_core::signal::Signal;
use web_sys::Element;

/// Kanban card.
#[derive(Clone)]
pub struct KanbanCard {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub assignees: Vec<String>,
    pub priority: KanbanPriority,
    pub tags: Vec<String>,
}

/// Priority level.
#[derive(Clone, PartialEq)]
pub enum KanbanPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Kanban column.
#[derive(Clone)]
pub struct KanbanColumn {
    pub id: String,
    pub title: String,
    pub cards: Vec<KanbanCard>,
    pub color: Option<String>,
}

/// Props for the KanbanBoard component.
pub struct KanbanBoardProps {
    pub columns: Vec<KanbanColumn>,
    pub draggable: bool,
    pub on_card_move: Option<Box<dyn Fn(String, String, usize)>>,
    pub on_card_click: Option<Box<dyn Fn(String)>>,
    pub class: Option<String>,
}

impl Default for KanbanBoardProps {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            draggable: true,
            on_card_move: None,
            on_card_click: None,
            class: None,
        }
    }
}

/// KanbanBoard component.
pub struct KanbanBoard;

impl KanbanBoard {
    /// Create a Kanban board element.
    pub fn create(_props: KanbanBoardProps) -> Element {
        // TODO: Implement Kanban board with drag-and-drop
        todo!("KanbanBoard component implementation pending")
    }
}
