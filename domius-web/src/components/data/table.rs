//! DataTable component - Advanced data table with sorting and filtering.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlInputElement};

#[derive(Clone)]
pub struct Column {
    pub field: String,
    pub header: String,
    pub sortable: bool,
    pub filterable: bool,
    pub width: Option<String>,
    pub align: ColumnAlign,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl ColumnAlign {
    fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
    None,
}

pub type RowData = HashMap<String, String>;

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

struct TableState {
    data: Vec<RowData>,
    filters: HashMap<String, String>,
    sort: Option<(String, SortDirection)>,
    selected: HashSet<usize>,
}

pub struct DataTable;

impl DataTable {
    pub fn create(props: DataTableProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let container = document
            .create_element("div")
            .expect("create table container");
        let mut container_classes = vec!["table-container"];
        if let Some(class) = props.class.as_deref() {
            container_classes.push(class);
        }
        container.set_class_name(&container_classes.join(" "));
        container
            .set_attribute("data-virtualized", &props.virtualized.to_string())
            .expect("set virtualization mode");
        if let Some(height) = props.row_height {
            container
                .set_attribute("data-row-height", &height.to_string())
                .expect("set row height");
        }

        let table = document.create_element("table").expect("create data table");
        let mut classes = vec!["table"];
        if props.striped {
            classes.push("table-striped");
        }
        if props.hoverable {
            classes.push("table-hoverable");
        }
        table.set_class_name(&classes.join(" "));

        let columns = Rc::new(props.columns);
        let state = Rc::new(RefCell::new(TableState {
            data: props.data,
            filters: HashMap::new(),
            sort: None,
            selected: HashSet::new(),
        }));
        let row_callback = props.on_row_click.map(Rc::<dyn Fn(usize)>::from);
        let sort_callback = props.on_sort.map(Rc::<dyn Fn(String, SortDirection)>::from);
        let body = document.create_element("tbody").expect("create table body");
        let head = create_head(
            &document,
            &table,
            &body,
            Rc::clone(&columns),
            Rc::clone(&state),
            row_callback.clone(),
            sort_callback,
            props.sortable,
            props.filterable,
            props.selectable,
        );
        table.append_child(&head).expect("append table head");
        table.append_child(&body).expect("append table body");
        render_rows(
            &body,
            columns.as_ref(),
            &state,
            props.selectable,
            row_callback,
        );
        container.append_child(&table).expect("append data table");
        container
    }
}

#[allow(clippy::too_many_arguments)]
fn create_head(
    document: &Document,
    table: &Element,
    body: &Element,
    columns: Rc<Vec<Column>>,
    state: Rc<RefCell<TableState>>,
    row_callback: Option<Rc<dyn Fn(usize)>>,
    sort_callback: Option<Rc<dyn Fn(String, SortDirection)>>,
    sortable: bool,
    filterable: bool,
    selectable: bool,
) -> Element {
    let head = document.create_element("thead").expect("create table head");
    let header_row = document.create_element("tr").expect("create header row");
    if selectable {
        let header = document
            .create_element("th")
            .expect("create selection header");
        header
            .set_attribute("scope", "col")
            .expect("set header scope");
        header
            .set_attribute("aria-label", "Selection")
            .expect("label selection column");
        header_row
            .append_child(&header)
            .expect("append selection header");
    }
    for column in columns.iter() {
        let header = document.create_element("th").expect("create column header");
        header
            .set_attribute("scope", "col")
            .expect("set column scope");
        header
            .set_attribute("data-field", &column.field)
            .expect("set column field");
        header
            .set_attribute("data-align", column.align.as_str())
            .expect("set alignment");
        if let Some(width) = column.width.as_deref() {
            header
                .set_attribute("data-width", width)
                .expect("set column width");
        }
        if sortable && column.sortable {
            let button = document
                .create_element("button")
                .expect("create sort button");
            button
                .set_attribute("type", "button")
                .expect("set button type");
            button.set_class_name("table-sort");
            button.set_text_content(Some(&column.header));
            let field = column.field.clone();
            let handler_state = Rc::clone(&state);
            let handler_body = body.clone();
            let handler_table = table.clone();
            let handler_columns = Rc::clone(&columns);
            let handler_rows = row_callback.clone();
            let handler_sort = sort_callback.clone();
            let handler = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_| {
                let direction = {
                    let mut state = handler_state.borrow_mut();
                    let next = match state.sort.as_ref() {
                        Some((active, SortDirection::Asc)) if active == &field => {
                            SortDirection::Desc
                        }
                        Some((active, SortDirection::Desc)) if active == &field => {
                            SortDirection::None
                        }
                        _ => SortDirection::Asc,
                    };
                    state.sort = (next != SortDirection::None).then(|| (field.clone(), next));
                    next
                };
                update_sort_headers(&handler_table, &field, direction);
                render_rows(
                    &handler_body,
                    handler_columns.as_ref(),
                    &handler_state,
                    selectable,
                    handler_rows.clone(),
                );
                if let Some(callback) = handler_sort.as_ref() {
                    callback(field.clone(), direction);
                }
            });
            button
                .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                .expect("register sort callback");
            handler.forget();
            header
                .set_attribute("aria-sort", "none")
                .expect("set initial sort state");
            header.append_child(&button).expect("append sort button");
        } else {
            header.set_text_content(Some(&column.header));
        }
        header_row
            .append_child(&header)
            .expect("append column header");
    }
    head.append_child(&header_row).expect("append header row");

    if filterable && columns.iter().any(|column| column.filterable) {
        let filter_row = document.create_element("tr").expect("create filter row");
        filter_row.set_class_name("table-filters");
        if selectable {
            filter_row
                .append_child(&document.create_element("th").expect("create filter spacer"))
                .expect("append filter spacer");
        }
        for column in columns.iter() {
            let cell = document.create_element("th").expect("create filter cell");
            if column.filterable {
                let input = document
                    .create_element("input")
                    .expect("create column filter");
                input
                    .set_attribute("type", "search")
                    .expect("set filter type");
                input
                    .set_attribute("aria-label", &format!("Filter {}", column.header))
                    .expect("label filter");
                input
                    .set_attribute("data-field", &column.field)
                    .expect("set filter field");
                let field = column.field.clone();
                let handler_state = Rc::clone(&state);
                let handler_body = body.clone();
                let handler_columns = Rc::clone(&columns);
                let handler_rows = row_callback.clone();
                let handler = Closure::<dyn FnMut(web_sys::InputEvent)>::new(
                    move |event: web_sys::InputEvent| {
                        let value = event
                            .target()
                            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                            .map(|input| input.value())
                            .unwrap_or_default();
                        let mut state = handler_state.borrow_mut();
                        if value.is_empty() {
                            state.filters.remove(&field);
                        } else {
                            state.filters.insert(field.clone(), value);
                        }
                        drop(state);
                        render_rows(
                            &handler_body,
                            handler_columns.as_ref(),
                            &handler_state,
                            selectable,
                            handler_rows.clone(),
                        );
                    },
                );
                input
                    .add_event_listener_with_callback("input", handler.as_ref().unchecked_ref())
                    .expect("register filter callback");
                handler.forget();
                cell.append_child(&input).expect("append filter");
            }
            filter_row.append_child(&cell).expect("append filter cell");
        }
        head.append_child(&filter_row).expect("append filter row");
    }
    head
}

fn render_rows(
    body: &Element,
    columns: &[Column],
    state: &Rc<RefCell<TableState>>,
    selectable: bool,
    row_callback: Option<Rc<dyn Fn(usize)>>,
) {
    body.set_text_content(None);
    let document = body.owner_document().expect("table owner document");
    let rows = visible_rows(&state.borrow());
    if rows.is_empty() {
        let row = document.create_element("tr").expect("create empty row");
        let cell = document.create_element("td").expect("create empty cell");
        cell.set_class_name("table-empty");
        cell.set_attribute(
            "colspan",
            &(columns.len() + usize::from(selectable)).to_string(),
        )
        .expect("span empty cell");
        cell.set_text_content(Some("No results"));
        row.append_child(&cell).expect("append empty cell");
        body.append_child(&row).expect("append empty row");
        return;
    }
    for (source_index, row_data) in rows {
        let row = document.create_element("tr").expect("create table row");
        row.set_attribute("data-row-index", &source_index.to_string())
            .expect("set row index");
        if state.borrow().selected.contains(&source_index) {
            row.set_attribute("aria-selected", "true")
                .expect("mark selected row");
        }
        if let Some(callback) = row_callback.clone() {
            let handler =
                Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_| callback(source_index));
            row.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                .expect("register row callback");
            handler.forget();
        }
        if selectable {
            let cell = document
                .create_element("td")
                .expect("create selection cell");
            let checkbox = document
                .create_element("input")
                .expect("create row checkbox");
            checkbox
                .set_attribute("type", "checkbox")
                .expect("set checkbox type");
            checkbox
                .set_attribute("aria-label", &format!("Select row {}", source_index + 1))
                .expect("label checkbox");
            if state.borrow().selected.contains(&source_index) {
                checkbox
                    .set_attribute("checked", "")
                    .expect("check selected row");
            }
            let handler_state = Rc::clone(state);
            let handler_row = row.clone();
            let handler = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(
                move |event: web_sys::MouseEvent| {
                    event.stop_propagation();
                    let selected = {
                        let mut state = handler_state.borrow_mut();
                        if state.selected.remove(&source_index) {
                            false
                        } else {
                            state.selected.insert(source_index);
                            true
                        }
                    };
                    if selected {
                        handler_row
                            .set_attribute("aria-selected", "true")
                            .expect("mark selected row");
                    } else {
                        handler_row
                            .remove_attribute("aria-selected")
                            .expect("clear selected row");
                    }
                },
            );
            checkbox
                .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                .expect("register row selection");
            handler.forget();
            cell.append_child(&checkbox).expect("append checkbox");
            row.append_child(&cell).expect("append selection cell");
        }
        for column in columns {
            let cell = document.create_element("td").expect("create table cell");
            cell.set_attribute("data-field", &column.field)
                .expect("set cell field");
            cell.set_attribute("data-align", column.align.as_str())
                .expect("set cell alignment");
            cell.set_text_content(Some(
                row_data
                    .get(&column.field)
                    .map(String::as_str)
                    .unwrap_or(""),
            ));
            row.append_child(&cell).expect("append table cell");
        }
        body.append_child(&row).expect("append table row");
    }
}

fn visible_rows(state: &TableState) -> Vec<(usize, RowData)> {
    let mut rows: Vec<_> = state
        .data
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, row)| {
            state.filters.iter().all(|(field, filter)| {
                row.get(field)
                    .is_some_and(|value| value.to_lowercase().contains(&filter.to_lowercase()))
            })
        })
        .collect();
    if let Some((field, direction)) = state.sort.as_ref() {
        rows.sort_by(|(_, left), (_, right)| {
            let order = left
                .get(field)
                .map(|value| value.to_lowercase())
                .cmp(&right.get(field).map(|value| value.to_lowercase()));
            if *direction == SortDirection::Desc {
                order.reverse()
            } else {
                order
            }
        });
    }
    rows
}

fn update_sort_headers(table: &Element, field: &str, direction: SortDirection) {
    let headers = table
        .query_selector_all("thead th[data-field]")
        .expect("query sort headers");
    for index in 0..headers.length() {
        let Some(header) = headers
            .item(index)
            .and_then(|node| node.dyn_into::<Element>().ok())
        else {
            continue;
        };
        let aria_sort = if header.get_attribute("data-field").as_deref() == Some(field) {
            match direction {
                SortDirection::Asc => "ascending",
                SortDirection::Desc => "descending",
                SortDirection::None => "none",
            }
        } else {
            "none"
        };
        if header.has_attribute("aria-sort") {
            header
                .set_attribute("aria-sort", aria_sort)
                .expect("update sort state");
        }
    }
}
