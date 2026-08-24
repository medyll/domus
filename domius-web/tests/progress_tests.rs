#![cfg(target_arch = "wasm32")]

mod test_utils;

use domius_core::signal::signal;
use domius_web::components::feedback::progress::{
    ProgressBar, ProgressProps, ProgressSize, ProgressVariant,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Effects flush on an animation frame, so give the runtime one before looking.
async fn settle() {
    for _ in 0..2 {
        let promise = js_sys::Promise::new(&mut |resolve, _| {
            test_utils::window()
                .request_animation_frame(&resolve)
                .expect("animation frame should be scheduled");
        });
        wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .expect("animation frame should run");
    }
}

fn label(bar: &web_sys::Element) -> Option<String> {
    bar.query_selector(".domius-progress-label")
        .unwrap()
        .and_then(|label| label.text_content())
}

#[wasm_bindgen_test]
fn the_linear_variant_is_a_native_progress_element() {
    let bar = ProgressBar::create(ProgressProps {
        value: signal(30),
        max: 60,
        size: ProgressSize::Lg,
        show_label: true,
        class: Some("resolution".into()),
        ..Default::default()
    });

    assert!(bar.class_name().contains("domius-progress-linear"));
    assert!(bar.class_name().contains("domius-progress-lg"));
    assert!(bar.class_name().contains("resolution"));
    assert_eq!(bar.get_attribute("data-variant").as_deref(), Some("linear"));

    let indicator = bar.query_selector("progress").unwrap().unwrap();
    assert_eq!(indicator.get_attribute("max").as_deref(), Some("60"));
    assert_eq!(indicator.get_attribute("value").as_deref(), Some("30"));
    assert_eq!(bar.get_attribute("data-percentage").as_deref(), Some("50"));
    assert_eq!(label(&bar).as_deref(), Some("50%"));
}

#[wasm_bindgen_test]
async fn the_bar_follows_its_value() {
    let value = signal(10);
    let bar = ProgressBar::create(ProgressProps {
        value: value.clone(),
        max: 100,
        show_label: true,
        ..Default::default()
    });
    let indicator = bar.query_selector("progress").unwrap().unwrap();
    assert_eq!(indicator.get_attribute("value").as_deref(), Some("10"));

    value.set(75);
    settle().await;

    assert_eq!(indicator.get_attribute("value").as_deref(), Some("75"));
    assert_eq!(bar.get_attribute("data-percentage").as_deref(), Some("75"));
    assert_eq!(label(&bar).as_deref(), Some("75%"));
}

#[wasm_bindgen_test]
async fn a_value_beyond_the_maximum_is_held_at_full() {
    let value = signal(0);
    let bar = ProgressBar::create(ProgressProps {
        value: value.clone(),
        max: 50,
        show_label: true,
        ..Default::default()
    });

    value.set(200);
    settle().await;

    let indicator = bar.query_selector("progress").unwrap().unwrap();
    assert_eq!(indicator.get_attribute("value").as_deref(), Some("50"));
    assert_eq!(bar.get_attribute("data-percentage").as_deref(), Some("100"));
    assert_eq!(label(&bar).as_deref(), Some("100%"));
}

#[wasm_bindgen_test]
fn a_custom_label_format_wins() {
    let bar = ProgressBar::create(ProgressProps {
        value: signal(13),
        max: 48,
        show_label: true,
        label_format: Some("{value} of {max} done".into()),
        ..Default::default()
    });

    assert_eq!(label(&bar).as_deref(), Some("13 of 48 done"));
}

#[wasm_bindgen_test]
fn no_label_is_rendered_unless_asked_for() {
    let bar = ProgressBar::create(ProgressProps {
        value: signal(13),
        ..Default::default()
    });

    assert!(bar
        .query_selector(".domius-progress-label")
        .unwrap()
        .is_none());
}

#[wasm_bindgen_test]
async fn the_circular_variant_draws_and_announces_its_arc() {
    let value = signal(25);
    let bar = ProgressBar::create(ProgressProps {
        value: value.clone(),
        max: 100,
        variant: ProgressVariant::Circular,
        ..Default::default()
    });

    let svg = bar.query_selector("svg").unwrap().unwrap();
    assert_eq!(svg.get_attribute("role").as_deref(), Some("progressbar"));
    assert_eq!(svg.get_attribute("aria-valuemax").as_deref(), Some("100"));
    assert_eq!(svg.get_attribute("aria-valuenow").as_deref(), Some("25"));
    assert_eq!(svg.get_attribute("aria-valuetext").as_deref(), Some("25%"));

    let arc = bar
        .query_selector("circle.domius-progress-bar")
        .unwrap()
        .unwrap();
    let quarter = arc.get_attribute("stroke-dasharray").unwrap();
    assert!(!quarter.is_empty());

    value.set(75);
    settle().await;
    assert_eq!(svg.get_attribute("aria-valuenow").as_deref(), Some("75"));
    assert_ne!(arc.get_attribute("stroke-dasharray").unwrap(), quarter);
}

#[wasm_bindgen_test]
fn an_indeterminate_bar_reports_no_value() {
    let bar = ProgressBar::create(ProgressProps {
        value: signal(40),
        indeterminate: true,
        ..Default::default()
    });

    assert!(bar.class_name().contains("domius-progress-indeterminate"));
    let indicator = bar.query_selector("progress").unwrap().unwrap();
    assert!(
        indicator.get_attribute("value").is_none(),
        "a progress element without a value is the indeterminate one"
    );
    assert!(bar.get_attribute("data-percentage").is_none());
}

#[wasm_bindgen_test]
fn the_colour_travels_as_a_token_not_a_style() {
    let bar = ProgressBar::create(ProgressProps {
        value: signal(40),
        color: Some("critical".into()),
        ..Default::default()
    });

    assert_eq!(bar.get_attribute("data-color").as_deref(), Some("critical"));
    assert_eq!(bar.query_selector_all("[style]").unwrap().length(), 0);
}
