//! WASM integration tests for Charts.

#![cfg(target_arch = "wasm32")]

use domius_web::components::data::charts::{ChartDataPoint, ChartType, Charts, ChartsProps};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn data() -> Vec<ChartDataPoint> {
    vec![
        ChartDataPoint {
            label: "Gateway".into(),
            value: 120.0,
        },
        ChartDataPoint {
            label: "Billing".into(),
            value: 80.0,
        },
        ChartDataPoint {
            label: "Search".into(),
            value: 40.0,
        },
    ]
}

#[wasm_bindgen_test]
fn bar_chart_is_accessible_and_has_a_legend() {
    let chart = Charts::create(ChartsProps {
        data: data(),
        ..Default::default()
    });
    assert_eq!(chart.tag_name(), "FIGURE");
    assert_eq!(
        chart.get_attribute("data-chart-type").as_deref(),
        Some("bar")
    );
    assert_eq!(chart.query_selector_all("svg rect").unwrap().length(), 3);
    assert_eq!(
        chart
            .query_selector_all(".domius-chart-legend li")
            .unwrap()
            .length(),
        3
    );
    assert_eq!(
        chart
            .query_selector("svg")
            .unwrap()
            .unwrap()
            .get_attribute("role")
            .as_deref(),
        Some("img")
    );
}

#[wasm_bindgen_test]
fn line_chart_exposes_points_and_native_tooltips() {
    let chart = Charts::create(ChartsProps {
        chart_type: ChartType::Line,
        data: data(),
        show_legend: false,
        ..Default::default()
    });
    assert_eq!(chart.query_selector_all("polyline").unwrap().length(), 1);
    assert_eq!(
        chart.query_selector_all("circle title").unwrap().length(),
        3
    );
    assert_eq!(
        chart
            .query_selector_all(".domius-chart-legend")
            .unwrap()
            .length(),
        0
    );
}

#[wasm_bindgen_test]
fn donut_chart_preserves_render_configuration() {
    let chart = Charts::create(ChartsProps {
        chart_type: ChartType::Donut,
        data: data(),
        animated: false,
        colors: vec!["primary".into(), "warning".into(), "critical".into()],
        ..Default::default()
    });
    assert_eq!(
        chart.get_attribute("data-animated").as_deref(),
        Some("false")
    );
    assert_eq!(
        chart
            .query_selector_all("svg circle[stroke-dasharray]")
            .unwrap()
            .length(),
        3
    );
    assert_eq!(
        chart
            .query_selector("[data-color='critical']")
            .unwrap()
            .unwrap()
            .get_attribute("data-series")
            .as_deref(),
        Some("2")
    );
}
