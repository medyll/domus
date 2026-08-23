//! WASM integration tests for DataTable.

#![cfg(target_arch = "wasm32")]

mod test_utils;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use domius_web::components::data::table::{
    Column, ColumnAlign, DataTable, DataTableProps, SortDirection,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlInputElement;

wasm_bindgen_test_configure!(run_in_browser);

fn columns() -> Vec<Column> {
    vec![
        Column {
            field: "service".into(),
            header: "Service".into(),
            sortable: true,
            filterable: true,
            width: Some("wide".into()),
            align: ColumnAlign::Left,
        },
        Column {
            field: "latency".into(),
            header: "Latency".into(),
            sortable: true,
            filterable: false,
            width: None,
            align: ColumnAlign::Right,
        },
    ]
}

fn row(service: &str, latency: &str) -> HashMap<String, String> {
    HashMap::from([
        ("service".into(), service.into()),
        ("latency".into(), latency.into()),
    ])
}

#[wasm_bindgen_test]
fn data_table_renders_semantic_selectable_rows() {
    let table = DataTable::create(DataTableProps {
        columns: columns(),
        data: vec![row("Search", "82"), row("Billing", "120")],
        selectable: true,
        ..Default::default()
    });
    assert_eq!(table.query_selector_all("table").unwrap().length(), 1);
    assert_eq!(
        table
            .query_selector_all("thead th[scope='col']")
            .unwrap()
            .length(),
        3
    );
    assert_eq!(table.query_selector_all("tbody tr").unwrap().length(), 2);
    let checkbox = table
        .query_selector("tbody input[type='checkbox']")
        .unwrap()
        .unwrap();
    test_utils::simulate_click(&checkbox);
    assert_eq!(
        table
            .query_selector("tbody tr")
            .unwrap()
            .unwrap()
            .get_attribute("aria-selected")
            .as_deref(),
        Some("true")
    );
}

#[wasm_bindgen_test]
fn data_table_sorts_and_reports_direction() {
    let sorts = Rc::new(RefCell::new(Vec::new()));
    let captured = Rc::clone(&sorts);
    let table = DataTable::create(DataTableProps {
        columns: columns(),
        data: vec![row("Search", "82"), row("Billing", "120")],
        on_sort: Some(Box::new(move |field, direction| {
            captured.borrow_mut().push((field, direction))
        })),
        ..Default::default()
    });
    let sort = table
        .query_selector("th[data-field='service'] button")
        .unwrap()
        .unwrap();
    test_utils::simulate_click(&sort);
    assert_eq!(
        table
            .query_selector("tbody tr td[data-field='service']")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("Billing")
    );
    assert_eq!(
        *sorts.borrow(),
        vec![("service".into(), SortDirection::Asc)]
    );
    assert_eq!(
        table
            .query_selector("th[data-field='service']")
            .unwrap()
            .unwrap()
            .get_attribute("aria-sort")
            .as_deref(),
        Some("ascending")
    );
}

#[wasm_bindgen_test]
fn data_table_filters_and_handles_empty_results() {
    let table = DataTable::create(DataTableProps {
        columns: columns(),
        data: vec![row("Search", "82"), row("Billing", "120")],
        filterable: true,
        ..Default::default()
    });
    let input: HtmlInputElement = table
        .query_selector("input[aria-label='Filter Service']")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    input.set_value("unknown");
    input
        .dispatch_event(&web_sys::Event::new("input").unwrap())
        .unwrap();
    assert_eq!(table.query_selector_all("tbody tr").unwrap().length(), 1);
    assert_eq!(
        table
            .query_selector(".table-empty")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("No results")
    );
}
