//! DataTable component - Advanced data table with sorting and filtering.

use domius_core::signal::Signal;
use web_sys::Element;

/// Column definition.
#[derive(Clone)]
pub struct Column {
    pub field: String,
    pub header: String,
    pub sortable: bool,
    pub filterable: bool,
    pub width: Option<String>,
    pub align: ColumnAlign,
}

/// Column alignment.
#[derive(Clone, PartialEq)]
pub enum ColumnAlign {
    Left,
    Center,
    Right,
}

impl Default for ColumnAlign {
    fn default() -> Self {
        Self::Left
    }
}

/// Sort direction.
#[derive(Clone, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
    None,
}

/// A row of data as key-value pairs.
pub type RowData = std::collections::HashMap<String, String>;

/// Props for the DataTable component.
pub struct DataTableProps {
    pub columns: Vec<Column>,
    pub data: Vec<RowData>,
    pub sortable: bool,
    pub filterable: bool,
    pub selectable: bool,
    pub striped: bool,
    pub hoverable: bool,
    pub virtualized: bool,
    pub row_height: Option<u32>,
    pub on_row_click: Option<Box<dyn Fn(usize)>>,
    pub on_sort: Option<Box<dyn Fn(String, SortDirection)>>,
    pub class: Option<String>,
}

impl Default for DataTableProps {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            data: Vec::new(),
            sortable: true,
            filterable: false,
            selectable: false,
            striped: true,
            hoverable: true,
            virtualized: false,
            row_height: None,
            on_row_click: None,
            on_sort: None,
            class: None,
        }
    }
}

/// DataTable component.
pub struct DataTable;

impl DataTable {
    /// Create a data table element.
    pub fn create(_props: DataTableProps) -> Element {
        // TODO: Implement data table
        todo!("DataTable component implementation pending")
    }
}
