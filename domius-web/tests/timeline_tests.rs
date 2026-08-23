//! WASM integration tests for Timeline.

#![cfg(target_arch = "wasm32")]

use domius_web::components::data::timeline::{
    Timeline, TimelineEvent, TimelineOrientation, TimelineProps,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn timeline_renders_chronological_events() {
    let timeline = Timeline::create(TimelineProps {
        events: vec![
            TimelineEvent {
                title: "Deployment completed".to_string(),
                description: Some("Version 1.4.2 reached production".to_string()),
                timestamp: Some("10:14 UTC".to_string()),
                icon: Some("D".to_string()),
                color: Some("success".to_string()),
            },
            TimelineEvent {
                title: "Latency alert".to_string(),
                description: None,
                timestamp: Some("10:19 UTC".to_string()),
                icon: None,
                color: Some("critical".to_string()),
            },
        ],
        orientation: TimelineOrientation::Vertical,
        alternate: false,
        class: None,
    });

    assert_eq!(timeline.tag_name(), "OL");
    assert_eq!(timeline.query_selector_all("li").unwrap().length(), 2);
    assert_eq!(timeline.query_selector_all("time").unwrap().length(), 2);
    assert_eq!(
        timeline
            .query_selector("h3")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("Deployment completed")
    );
    assert_eq!(
        timeline
            .query_selector("[data-color='critical']")
            .unwrap()
            .unwrap()
            .get_attribute("aria-hidden")
            .as_deref(),
        Some("true")
    );
}

#[wasm_bindgen_test]
fn timeline_exposes_horizontal_alternating_mode() {
    let timeline = Timeline::create(TimelineProps {
        orientation: TimelineOrientation::Horizontal,
        alternate: true,
        ..Default::default()
    });

    assert_eq!(
        timeline.get_attribute("data-orientation").as_deref(),
        Some("horizontal")
    );
    assert!(timeline.class_list().contains("domius-timeline-alternate"));
}
