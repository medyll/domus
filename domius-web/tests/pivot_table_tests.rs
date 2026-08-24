//! WASM integration tests for PivotTable.

#![cfg(target_arch = "wasm32")]

use domius_web::components::pro::pivot_table::{
    Aggregator, PivotData, PivotTable, PivotTableProps,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn record(region: &str, quarter: &str, revenue: &str, label: &str) -> PivotData {
    [
        ("region", region),
        ("quarter", quarter),
        ("revenue", revenue),
        ("label", label),
    ]
    .into_iter()
    .map(|(key, value)| (key.into(), value.into()))
    .collect()
}

fn props(aggregator: Aggregator) -> PivotTableProps {
    PivotTableProps {
        data: vec![
            record("EU", "Q1", "10", "first"),
            record("EU", "Q1", "20", "second"),
            record("EU", "Q2", "30", "third"),
            record("US", "Q1", "5", "fourth"),
        ],
        rows: vec!["region".into()],
        columns: vec!["quarter".into()],
        values: vec!["revenue".into()],
        aggregator,
        show_totals: true,
        collapsible: true,
        class: Some("report-pivot".into()),
    }
}

fn first_value(aggregator: Aggregator) -> String {
    PivotTable::create(props(aggregator))
        .query_selector("tbody td")
        .unwrap()
        .unwrap()
        .text_content()
        .unwrap()
}

#[wasm_bindgen_test]
fn renders_dimensions_values_and_totals_deterministically() {
    let pivot = PivotTable::create(props(Aggregator::Sum));
    assert_eq!(pivot.class_name(), "table-container report-pivot");
    assert_eq!(
        pivot.get_attribute("data-aggregator").as_deref(),
        Some("sum")
    );
    assert_eq!(pivot.query_selector_all("tbody tr").unwrap().length(), 2);
    assert_eq!(pivot.query_selector_all("thead th").unwrap().length(), 4);
    assert_eq!(pivot.query_selector_all("tfoot td").unwrap().length(), 3);
    let cells = pivot
        .query_selector("tbody tr")
        .unwrap()
        .unwrap()
        .query_selector_all("td")
        .unwrap();
    assert_eq!(cells.item(0).unwrap().text_content().as_deref(), Some("30"));
    assert_eq!(cells.item(1).unwrap().text_content().as_deref(), Some("30"));
    assert_eq!(cells.item(2).unwrap().text_content().as_deref(), Some("60"));
}

#[wasm_bindgen_test]
fn supports_all_numeric_aggregators() {
    assert_eq!(first_value(Aggregator::Average), "15");
    assert_eq!(first_value(Aggregator::Min), "10");
    assert_eq!(first_value(Aggregator::Max), "20");
    assert_eq!(first_value(Aggregator::Count), "2");
}

#[wasm_bindgen_test]
fn supports_first_and_last_text_values() {
    let text = |aggregator| {
        let mut props = props(aggregator);
        props.values = vec!["label".into()];
        PivotTable::create(props)
            .query_selector("tbody td")
            .unwrap()
            .unwrap()
            .text_content()
            .unwrap()
    };
    assert_eq!(text(Aggregator::First), "first");
    assert_eq!(text(Aggregator::Last), "second");
}

#[wasm_bindgen_test]
fn collapse_control_toggles_accessible_state() {
    let pivot = PivotTable::create(props(Aggregator::Sum));
    let button = pivot.query_selector("tbody button").unwrap().unwrap();
    button
        .dispatch_event(&web_sys::Event::new("click").unwrap())
        .unwrap();
    assert_eq!(
        button.get_attribute("aria-expanded").as_deref(),
        Some("false")
    );
    assert_eq!(
        pivot
            .query_selector("tbody tr")
            .unwrap()
            .unwrap()
            .get_attribute("data-collapsed")
            .as_deref(),
        Some("true")
    );
    assert_eq!(
        pivot
            .query_selector("tbody td")
            .unwrap()
            .unwrap()
            .get_attribute("data-collapsed")
            .as_deref(),
        Some("true")
    );
}
