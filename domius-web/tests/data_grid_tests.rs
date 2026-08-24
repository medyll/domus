//! WASM integration tests for DataGrid.

#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::rc::Rc;

use domius_web::components::pro::data_grid::{DataGrid, DataGridProps, GridColumn};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlInputElement;

wasm_bindgen_test_configure!(run_in_browser);

fn columns() -> Vec<GridColumn> {
    vec![
        GridColumn {
            field: "service".into(),
            header: "Service".into(),
            width: Some(180),
            editable: false,
            cell_renderer: None,
        },
        GridColumn {
            field: "latency".into(),
            header: "Latency".into(),
            width: None,
            editable: true,
            cell_renderer: Some("numeric".into()),
        },
    ]
}

#[wasm_bindgen_test]
fn grid_renders_accessible_frozen_coordinates() {
    let grid = DataGrid::create(DataGridProps {
        columns: columns(),
        data: vec![
            vec!["Gateway".into(), "82".into()],
            vec!["Billing".into(), "120".into()],
        ],
        ..Default::default()
    });
    let table = grid.query_selector("table").unwrap().unwrap();
    assert_eq!(table.get_attribute("role").as_deref(), Some("grid"));
    assert_eq!(table.get_attribute("aria-rowcount").as_deref(), Some("2"));
    assert_eq!(grid.query_selector_all("tbody td").unwrap().length(), 4);
    assert_eq!(
        grid.query_selector_all("[data-frozen='true']")
            .unwrap()
            .length(),
        4
    );
}

#[wasm_bindgen_test]
fn editable_cell_updates_value_and_reports_coordinates() {
    let changes = Rc::new(RefCell::new(Vec::new()));
    let captured = Rc::clone(&changes);
    let grid = DataGrid::create(DataGridProps {
        columns: columns(),
        data: vec![vec!["Gateway".into(), "82".into()]],
        on_cell_change: Some(Box::new(move |row, column, value| {
            captured.borrow_mut().push((row, column, value))
        })),
        ..Default::default()
    });
    let input: HtmlInputElement = grid
        .query_selector("input")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    input.set_value("91");
    input
        .dispatch_event(&web_sys::Event::new("change").unwrap())
        .unwrap();
    assert_eq!(*changes.borrow(), vec![(0, 1, "91".into())]);
    assert_eq!(
        grid.query_selector("td[data-column='1']")
            .unwrap()
            .unwrap()
            .get_attribute("data-value")
            .as_deref(),
        Some("91")
    );
}

#[wasm_bindgen_test]
fn global_read_only_mode_contains_no_editors() {
    let grid = DataGrid::create(DataGridProps {
        columns: columns(),
        data: vec![vec!["Gateway".into(), "82".into()]],
        editable: false,
        ..Default::default()
    });
    assert_eq!(grid.query_selector_all("input").unwrap().length(), 0);
    assert_eq!(
        grid.query_selector("td[data-column='1']")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("82")
    );
}
