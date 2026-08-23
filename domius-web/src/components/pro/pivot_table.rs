//! PivotTable component - Data aggregation and summarization.

use std::collections::HashMap;
use web_sys::Element;

/// Aggregation function.
#[derive(Clone)]
pub enum Aggregator {
    Sum,
    Count,
    Average,
    Min,
    Max,
    First,
    Last,
}

/// A row of data as key-value pairs.
pub type PivotData = HashMap<String, String>;

/// Props for the PivotTable component.
#[derive(Clone)]
pub struct PivotTableProps {
    pub data: Vec<PivotData>,
    pub rows: Vec<String>,
    pub columns: Vec<String>,
    pub values: Vec<String>,
    pub aggregator: Aggregator,
    pub show_totals: bool,
    pub collapsible: bool,
    pub class: Option<String>,
}

impl Default for PivotTableProps {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            rows: Vec::new(),
            columns: Vec::new(),
            values: Vec::new(),
            aggregator: Aggregator::Sum,
            show_totals: true,
            collapsible: true,
            class: None,
        }
    }
}

/// PivotTable component.
pub struct PivotTable;

impl PivotTable {
    /// Create a pivot table element.
    pub fn create(_props: PivotTableProps) -> Element {
        // TODO: Implement pivot table
        todo!("PivotTable component implementation pending")
    }
}
