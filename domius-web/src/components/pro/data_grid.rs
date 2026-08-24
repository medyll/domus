//! Accessible editable data grid.

use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};

#[derive(Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub value: String,
    pub editable: bool,
}

#[derive(Clone)]
pub struct GridColumn {
    pub field: String,
    pub header: String,
    pub width: Option<u32>,
    pub editable: bool,
    pub cell_renderer: Option<String>,
}

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

pub struct DataGrid;

impl DataGrid {
    pub fn create(props: DataGridProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let container = document
            .create_element("div")
            .expect("create data grid container");
        let mut classes = vec!["table-container"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        container.set_class_name(&classes.join(" "));
        container
            .set_attribute("data-virtualized", &props.virtualized.to_string())
            .expect("set grid virtualization mode");
        container
            .set_attribute("data-row-height", &props.row_height.to_string())
            .expect("set grid row height");
        container
            .set_attribute("data-column-width", &props.column_width.to_string())
            .expect("set grid column width");

        let table = document.create_element("table").expect("create data grid");
        table.set_class_name("table");
        table.set_attribute("role", "grid").expect("set grid role");
        table
            .set_attribute("aria-rowcount", &props.data.len().to_string())
            .expect("set grid row count");
        table
            .set_attribute("aria-colcount", &props.columns.len().to_string())
            .expect("set grid column count");

        let head = document.create_element("thead").expect("create grid head");
        let header_row = document
            .create_element("tr")
            .expect("create grid header row");
        for (column_index, column) in props.columns.iter().enumerate() {
            let header = document.create_element("th").expect("create grid header");
            header
                .set_attribute("scope", "col")
                .expect("set grid header scope");
            header
                .set_attribute("data-field", &column.field)
                .expect("set grid field");
            header
                .set_attribute(
                    "data-width",
                    &column.width.unwrap_or(props.column_width).to_string(),
                )
                .expect("set grid column width");
            if column_index < props.frozen_columns {
                header
                    .set_attribute("data-frozen", "true")
                    .expect("freeze grid header");
            }
            header.set_text_content(Some(&column.header));
            header_row
                .append_child(&header)
                .expect("append grid header");
        }
        head.append_child(&header_row)
            .expect("append grid header row");
        table.append_child(&head).expect("append grid head");

        let callback = props
            .on_cell_change
            .map(Rc::<dyn Fn(usize, usize, String)>::from);
        let body = document.create_element("tbody").expect("create grid body");
        for (row_index, values) in props.data.into_iter().enumerate() {
            let row = document.create_element("tr").expect("create grid row");
            row.set_attribute("aria-rowindex", &(row_index + 1).to_string())
                .expect("set grid row index");
            if row_index < props.frozen_rows {
                row.set_attribute("data-frozen", "true")
                    .expect("freeze grid row");
            }
            for (column_index, column) in props.columns.iter().enumerate() {
                let value = values.get(column_index).cloned().unwrap_or_default();
                let cell = document.create_element("td").expect("create grid cell");
                cell.set_attribute("role", "gridcell")
                    .expect("set grid cell role");
                cell.set_attribute("aria-colindex", &(column_index + 1).to_string())
                    .expect("set grid column index");
                cell.set_attribute("data-row", &row_index.to_string())
                    .expect("set cell row");
                cell.set_attribute("data-column", &column_index.to_string())
                    .expect("set cell column");
                cell.set_attribute("data-value", &value)
                    .expect("set cell value");
                if column_index < props.frozen_columns {
                    cell.set_attribute("data-frozen", "true")
                        .expect("freeze grid cell");
                }

                if props.editable && column.editable {
                    let input = document
                        .create_element("input")
                        .expect("create grid cell editor");
                    input
                        .set_attribute("type", "text")
                        .expect("set editor type");
                    input
                        .set_attribute("value", &value)
                        .expect("set editor value");
                    input
                        .set_attribute(
                            "aria-label",
                            &format!("{} row {}", column.header, row_index + 1),
                        )
                        .expect("label cell editor");
                    if let Some(renderer) = column.cell_renderer.as_deref() {
                        input
                            .set_attribute("data-renderer", renderer)
                            .expect("set cell renderer");
                    }
                    let handler_callback = callback.clone();
                    let handler_cell = cell.clone();
                    let handler =
                        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                            let value = event
                                .target()
                                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                                .map(|input| input.value())
                                .unwrap_or_default();
                            handler_cell
                                .set_attribute("data-value", &value)
                                .expect("update cell value");
                            if let Some(callback) = handler_callback.as_ref() {
                                callback(row_index, column_index, value);
                            }
                        });
                    input
                        .add_event_listener_with_callback(
                            "change",
                            handler.as_ref().unchecked_ref(),
                        )
                        .expect("register cell editor");
                    handler.forget();
                    cell.append_child(&input).expect("append cell editor");
                } else {
                    cell.set_text_content(Some(&value));
                }
                row.append_child(&cell).expect("append grid cell");
            }
            body.append_child(&row).expect("append grid row");
        }
        table.append_child(&body).expect("append grid body");
        container.append_child(&table).expect("append data grid");
        container
    }
}
