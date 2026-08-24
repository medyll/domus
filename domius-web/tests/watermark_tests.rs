#![cfg(target_arch = "wasm32")]

use domius_web::components::pro::watermark::{Watermark, WatermarkProps};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn pattern(overlay: &web_sys::Element) -> web_sys::Element {
    overlay.query_selector("pattern").unwrap().unwrap()
}

#[wasm_bindgen_test]
fn tiles_text_with_gap_offset_rotation_and_opacity() {
    let overlay = Watermark::create(WatermarkProps {
        text: Some("INTERNAL COPY".into()),
        opacity: 0.25,
        rotation: -30.0,
        gap: (120, 90),
        offset: (60, 45),
        font_size: 18,
        font_color: "#123456".into(),
        class: Some("report".into()),
        ..Default::default()
    });

    assert_eq!(overlay.class_name(), "domius-watermark report");
    assert_eq!(
        overlay.get_attribute("data-role").as_deref(),
        Some("watermark")
    );
    assert_eq!(
        overlay.get_attribute("data-watermark").as_deref(),
        Some("INTERNAL COPY")
    );
    assert_eq!(
        overlay.get_attribute("data-opacity").as_deref(),
        Some("0.25")
    );
    assert_eq!(
        overlay.get_attribute("data-rotation").as_deref(),
        Some("-30")
    );

    let tile = pattern(&overlay);
    assert_eq!(tile.get_attribute("width").as_deref(), Some("120"));
    assert_eq!(tile.get_attribute("height").as_deref(), Some("90"));
    assert_eq!(
        tile.get_attribute("patternUnits").as_deref(),
        Some("userSpaceOnUse")
    );
    assert_eq!(
        tile.get_attribute("patternTransform").as_deref(),
        Some("rotate(-30)")
    );

    let text = overlay.query_selector("pattern text").unwrap().unwrap();
    assert_eq!(text.text_content(), Some("INTERNAL COPY".to_string()));
    assert_eq!(text.get_attribute("x").as_deref(), Some("60"));
    assert_eq!(text.get_attribute("y").as_deref(), Some("45"));
    assert_eq!(text.get_attribute("font-size").as_deref(), Some("18"));
    assert_eq!(text.get_attribute("fill").as_deref(), Some("#123456"));
    assert_eq!(text.get_attribute("opacity").as_deref(), Some("0.25"));

    // The overlay carries no inline styles; the sheet owns placement.
    assert_eq!(overlay.query_selector_all("[style]").unwrap().length(), 0);
}

#[wasm_bindgen_test]
fn the_fill_references_its_own_pattern() {
    let first = Watermark::create(WatermarkProps::default());
    let second = Watermark::create(WatermarkProps::default());

    let first_id = pattern(&first).get_attribute("id").unwrap();
    let second_id = pattern(&second).get_attribute("id").unwrap();
    assert_ne!(first_id, second_id);

    for (overlay, id) in [(&first, &first_id), (&second, &second_id)] {
        let fill = overlay.query_selector("rect").unwrap().unwrap();
        assert_eq!(fill.get_attribute("fill"), Some(format!("url(#{id})")));
        assert_eq!(fill.get_attribute("width").as_deref(), Some("100%"));
        assert_eq!(fill.get_attribute("height").as_deref(), Some("100%"));
    }
}

#[wasm_bindgen_test]
fn the_layer_announces_the_mark() {
    let overlay = Watermark::create(WatermarkProps {
        text: Some("DO NOT SHARE".into()),
        ..Default::default()
    });

    let layer = overlay.query_selector("svg").unwrap().unwrap();
    assert_eq!(layer.get_attribute("role").as_deref(), Some("img"));
    assert_eq!(
        layer.get_attribute("aria-label").as_deref(),
        Some("Watermark: DO NOT SHARE")
    );
}

#[wasm_bindgen_test]
fn an_image_mark_replaces_the_caption() {
    let overlay = Watermark::create(WatermarkProps {
        text: None,
        image: Some("/assets/seal.svg".into()),
        gap: (80, 80),
        offset: (10, 10),
        opacity: 0.5,
        ..Default::default()
    });

    assert_eq!(
        overlay.get_attribute("data-watermark").as_deref(),
        Some("/assets/seal.svg")
    );
    assert!(overlay.query_selector("pattern text").unwrap().is_none());

    let image = overlay.query_selector("pattern image").unwrap().unwrap();
    assert_eq!(
        image.get_attribute("href").as_deref(),
        Some("/assets/seal.svg")
    );
    assert_eq!(image.get_attribute("width").as_deref(), Some("80"));
    assert_eq!(image.get_attribute("height").as_deref(), Some("80"));
    assert_eq!(image.get_attribute("opacity").as_deref(), Some("0.5"));
}

#[wasm_bindgen_test]
fn degenerate_props_still_produce_a_usable_mark() {
    let overlay = Watermark::create(WatermarkProps {
        text: Some(String::new()),
        image: None,
        opacity: 4.0,
        gap: (0, 0),
        font_size: 0,
        ..Default::default()
    });

    // An empty caption with no image falls back to the default mark.
    assert_eq!(
        overlay.get_attribute("data-watermark").as_deref(),
        Some("CONFIDENTIAL")
    );
    assert_eq!(overlay.get_attribute("data-opacity").as_deref(), Some("1"));

    let tile = pattern(&overlay);
    assert_eq!(tile.get_attribute("width").as_deref(), Some("1"));
    assert_eq!(tile.get_attribute("height").as_deref(), Some("1"));

    let text = overlay.query_selector("pattern text").unwrap().unwrap();
    assert_eq!(text.get_attribute("font-size").as_deref(), Some("1"));
    assert_eq!(text.get_attribute("opacity").as_deref(), Some("1"));
}
