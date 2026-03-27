//! DataGrid component - High-performance editable data grid.
//!
//! Advanced table with Excel-like editing capabilities.

use web_sys::Element;

/// DataGrid cell.
#[derive(Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub value: String,
    pub editable: bool,
}

/// DataGrid column.
#[derive(Clone)]
pub struct GridColumn {
    pub field: String,
    pub header: String,
    pub width: Option<u32>,
    pub editable: bool,
    pub cell_renderer: Option<String>,
}

/// Props for the DataGrid component.
pub struct DataGridProps {
    pub columns: Vec<GridColumn>,
    pub data: Vec<Vec<String>>,
    pub editable: bool,
    pub virtualized: bool,
    pub row_height: u32,
    pub column_width: u32,
    pub frozen_rows: usize,
    pub frozen_columns: usize,
    pub on_cell_change: Option<Box<dyn Fn(usize, usize, String)>>,
    pub class: Option<String>,
}

impl Default for DataGridProps {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            data: Vec::new(),
            editable: true,
            virtualized: true,
            row_height: 32,
            column_width: 100,
            frozen_rows: 1,
            frozen_columns: 1,
            on_cell_change: None,
            class: None,
        }
    }
}

/// DataGrid component.
pub struct DataGrid;

impl DataGrid {
    /// Create a data grid element.
    pub fn create(_props: DataGridProps) -> Element {
        // TODO: Implement data grid with virtualization
        todo!("DataGrid component implementation pending")
    }
}
