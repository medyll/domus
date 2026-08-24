#![cfg(target_arch = "wasm32")]

use domius_web::components::pro::heatmap::{Heatmap, HeatmapCell, HeatmapColorScale, HeatmapProps};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn cells() -> Vec<HeatmapCell> {
    vec![
        HeatmapCell {
            x: 0,
            y: 0,
            value: 10.0,
        },
        HeatmapCell {
            x: 1,
            y: 0,
            value: 30.0,
        },
        HeatmapCell {
            x: 0,
            y: 1,
            value: 20.0,
        },
    ]
}

#[wasm_bindgen_test]
fn renders_axes_values_colors_and_missing_cells() {
    let heatmap = Heatmap::create(HeatmapProps {
        data: cells(),
        x_labels: vec!["Morning".into(), "Evening".into()],
        y_labels: vec!["API".into(), "Worker".into()],
        color_scale: HeatmapColorScale::Sequential(vec!["low".into(), "high".into()]),
        show_values: true,
        on_cell_click: None,
        class: Some("activity".into()),
    });
    assert_eq!(heatmap.class_name(), "table-container heatmap activity");
    assert_eq!(heatmap.query_selector_all("tbody tr").unwrap().length(), 2);
    assert_eq!(heatmap.query_selector_all("tbody td").unwrap().length(), 4);
    assert_eq!(
        heatmap
            .query_selector("[data-x='1'][data-y='0']")
            .unwrap()
            .unwrap()
            .get_attribute("data-color")
            .as_deref(),
        Some("high")
    );
    assert_eq!(
        heatmap
            .query_selector("[data-x='1'][data-y='1']")
            .unwrap()
            .unwrap()
            .get_attribute("data-empty")
            .as_deref(),
        Some("true")
    );
}

#[wasm_bindgen_test]
fn interactive_cells_report_coordinates() {
    let selected = Rc::new(RefCell::new(None));
    let captured = Rc::clone(&selected);
    let heatmap = Heatmap::create(HeatmapProps {
        data: cells(),
        x_labels: vec!["Morning".into(), "Evening".into()],
        y_labels: vec!["API".into(), "Worker".into()],
        on_cell_click: Some(Box::new(move |x, y| *captured.borrow_mut() = Some((x, y)))),
        ..Default::default()
    });
    let button = heatmap
        .query_selector("[data-x='1'][data-y='0'] button")
        .unwrap()
        .unwrap();
    assert_eq!(
        button.get_attribute("aria-label").as_deref(),
        Some("API, Evening: 30")
    );
    button
        .dispatch_event(&web_sys::Event::new("click").unwrap())
        .unwrap();
    assert_eq!(*selected.borrow(), Some((1, 0)));
}

#[wasm_bindgen_test]
fn supports_diverging_and_categorical_scales() {
    for scale in [
        HeatmapColorScale::Diverging(vec!["cold".into(), "hot".into()]),
        HeatmapColorScale::Categorical(vec!["a".into(), "b".into()]),
    ] {
        let heatmap = Heatmap::create(HeatmapProps {
            data: cells(),
            x_labels: vec!["x".into(), "y".into()],
            y_labels: vec!["z".into(), "w".into()],
            color_scale: scale,
            ..Default::default()
        });
        assert!(heatmap.query_selector("[data-color]").unwrap().is_some());
    }
}
