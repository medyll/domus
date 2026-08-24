#![cfg(target_arch = "wasm32")]

use domius_web::components::pro::scatter_plot::{ScatterPlot, ScatterPlotProps, ScatterPoint};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn point(x: f64, y: f64) -> ScatterPoint {
    ScatterPoint {
        x,
        y,
        label: None,
        color: None,
        size: None,
    }
}

fn correlation() -> Vec<ScatterPoint> {
    vec![
        ScatterPoint {
            label: Some("Gateway".into()),
            color: Some("critical".into()),
            size: Some(9.0),
            ..point(10.0, 1.0)
        },
        point(20.0, 2.0),
        ScatterPoint {
            label: Some("Billing".into()),
            ..point(30.0, 4.0)
        },
    ]
}

#[wasm_bindgen_test]
fn renders_marks_axes_grid_and_computed_domains() {
    let plot = ScatterPlot::create(ScatterPlotProps {
        points: correlation(),
        x_label: Some("Latency".into()),
        y_label: Some("Error rate".into()),
        show_labels: true,
        class: Some("correlation".into()),
        ..Default::default()
    });

    assert_eq!(plot.class_name(), "domius-scatter-plot correlation");
    assert_eq!(plot.get_attribute("data-points").as_deref(), Some("3"));
    assert!(plot.get_attribute("data-empty").is_none());

    let svg = plot.query_selector("svg").unwrap().unwrap();
    assert_eq!(svg.get_attribute("role").as_deref(), Some("img"));
    assert_eq!(svg.get_attribute("data-x-domain").as_deref(), Some("10 30"));
    assert_eq!(svg.get_attribute("data-y-domain").as_deref(), Some("1 4"));
    assert_eq!(
        svg.get_attribute("aria-label").as_deref(),
        Some("Scatter plot of 3 points, Latency from 10 to 30, Error rate from 1 to 4")
    );
    assert!(svg.get_attribute("data-constant-domain").is_none());

    assert_eq!(
        plot.query_selector_all("[data-role='marks'] circle")
            .unwrap()
            .length(),
        3
    );
    assert_eq!(
        plot.query_selector_all("[data-role='grid'] line")
            .unwrap()
            .length(),
        10
    );
    assert_eq!(
        plot.query_selector_all("[data-role='axes'] line")
            .unwrap()
            .length(),
        2
    );
    assert_eq!(
        plot.query_selector_all("[data-role='axis-label']")
            .unwrap()
            .length(),
        2
    );
    assert_eq!(
        plot.query_selector_all("[data-role='tick']")
            .unwrap()
            .length(),
        4
    );
    // Labels are opt-in and only rendered for points that carry one.
    assert_eq!(
        plot.query_selector_all("[data-role='point-label']")
            .unwrap()
            .length(),
        2
    );
}

#[wasm_bindgen_test]
fn marks_carry_values_colors_sizes_and_accessible_names() {
    let plot = ScatterPlot::create(ScatterPlotProps {
        points: correlation(),
        ..Default::default()
    });

    let first = plot.query_selector("[data-index='0']").unwrap().unwrap();
    assert_eq!(first.get_attribute("data-x").as_deref(), Some("10"));
    assert_eq!(first.get_attribute("data-y").as_deref(), Some("1"));
    assert_eq!(
        first.get_attribute("data-color").as_deref(),
        Some("critical")
    );
    assert_eq!(first.get_attribute("r").as_deref(), Some("9"));
    assert_eq!(
        first
            .query_selector("title")
            .unwrap()
            .unwrap()
            .text_content(),
        Some("Gateway: 10, 1".to_string())
    );

    let second = plot.query_selector("[data-index='1']").unwrap().unwrap();
    assert_eq!(
        second.get_attribute("data-color").as_deref(),
        Some("primary")
    );
    assert_eq!(second.get_attribute("r").as_deref(), Some("4"));
    assert_eq!(
        second
            .query_selector("title")
            .unwrap()
            .unwrap()
            .text_content(),
        Some("20, 2".to_string())
    );
}

#[wasm_bindgen_test]
fn explicit_domains_place_and_flag_points() {
    let plot = ScatterPlot::create(ScatterPlotProps {
        points: vec![point(0.0, 0.0), point(50.0, 5.0), point(120.0, 5.0)],
        x_min: Some(0.0),
        x_max: Some(100.0),
        y_min: Some(0.0),
        y_max: Some(10.0),
        width: Some(400),
        height: Some(300),
        ..Default::default()
    });

    let svg = plot.query_selector("svg").unwrap().unwrap();
    assert_eq!(svg.get_attribute("data-x-domain").as_deref(), Some("0 100"));
    assert_eq!(svg.get_attribute("data-y-domain").as_deref(), Some("0 10"));

    // Half of an explicit domain lands in the middle of the plot area.
    let middle = plot.query_selector("[data-index='1']").unwrap().unwrap();
    assert_eq!(middle.get_attribute("cx").as_deref(), Some("216"));
    assert_eq!(middle.get_attribute("cy").as_deref(), Some("142"));
    assert!(middle.get_attribute("data-outside").is_none());

    // A point beyond an explicit domain is clamped inside and flagged.
    let overflow = plot.query_selector("[data-index='2']").unwrap().unwrap();
    assert_eq!(
        overflow.get_attribute("data-outside").as_deref(),
        Some("true")
    );
    assert_eq!(overflow.get_attribute("cx").as_deref(), Some("388"));
}

#[wasm_bindgen_test]
fn constant_domains_centre_every_mark() {
    let plot = ScatterPlot::create(ScatterPlotProps {
        points: vec![point(7.0, 3.0), point(7.0, 3.0)],
        width: Some(400),
        height: Some(300),
        ..Default::default()
    });

    let svg = plot.query_selector("svg").unwrap().unwrap();
    assert_eq!(svg.get_attribute("data-x-domain").as_deref(), Some("7 7"));
    assert_eq!(
        svg.get_attribute("data-constant-domain").as_deref(),
        Some("true")
    );

    let marks = plot
        .query_selector_all("[data-role='marks'] circle")
        .unwrap();
    assert_eq!(marks.length(), 2);
    for index in 0..marks.length() {
        let mark = marks.item(index).unwrap();
        let mark: web_sys::Element = wasm_bindgen::JsCast::unchecked_into(mark);
        assert_eq!(mark.get_attribute("cx").as_deref(), Some("216"));
        assert_eq!(mark.get_attribute("cy").as_deref(), Some("142"));
        assert!(mark.get_attribute("data-outside").is_none());
    }
}

#[wasm_bindgen_test]
fn empty_data_renders_a_readable_empty_state() {
    let plot = ScatterPlot::create(ScatterPlotProps {
        points: vec![],
        x_label: Some("Latency".into()),
        ..Default::default()
    });

    assert_eq!(plot.get_attribute("data-points").as_deref(), Some("0"));
    assert_eq!(plot.get_attribute("data-empty").as_deref(), Some("true"));
    assert_eq!(
        plot.query_selector_all("[data-role='marks'] circle")
            .unwrap()
            .length(),
        0
    );

    let empty = plot.query_selector("[data-role='empty']").unwrap().unwrap();
    assert_eq!(empty.text_content(), Some("No data".to_string()));

    let svg = plot.query_selector("svg").unwrap().unwrap();
    assert_eq!(svg.get_attribute("data-x-domain").as_deref(), Some("0 0"));
    assert_eq!(
        svg.get_attribute("aria-label").as_deref(),
        Some("Scatter plot of 0 points, Latency from 0 to 0, y from 0 to 0")
    );
    // Axes still render so the empty state keeps its frame of reference.
    assert_eq!(
        plot.query_selector_all("[data-role='axes'] line")
            .unwrap()
            .length(),
        2
    );
}

#[wasm_bindgen_test]
fn grid_can_be_disabled() {
    let plot = ScatterPlot::create(ScatterPlotProps {
        points: correlation(),
        show_grid: false,
        ..Default::default()
    });

    assert!(plot.query_selector("[data-role='grid']").unwrap().is_none());
    assert_eq!(
        plot.query_selector_all("[data-role='axes'] line")
            .unwrap()
            .length(),
        2
    );
}
