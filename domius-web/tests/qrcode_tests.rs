#![cfg(target_arch = "wasm32")]

use domius_web::components::primitives::qrcode::{
    qrcode, qrcode_matrix, QRCodeErrorLevel, QRCodeProps,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const REPORT_URL: &str = "http://127.0.0.1:8080/reports";

/// Read the drawn grid back out of the SVG.
fn drawn_modules(code: &web_sys::Element) -> Vec<(u32, u32)> {
    let rects = code
        .query_selector_all("[data-role='modules'] rect")
        .unwrap();
    let mut drawn = Vec::new();
    for index in 0..rects.length() {
        let rect: web_sys::Element = rects.item(index).unwrap().unchecked_into();
        assert_eq!(rect.get_attribute("width").as_deref(), Some("1"));
        assert_eq!(rect.get_attribute("height").as_deref(), Some("1"));
        drawn.push((
            rect.get_attribute("x").unwrap().parse().unwrap(),
            rect.get_attribute("y").unwrap().parse().unwrap(),
        ));
    }
    drawn
}

#[wasm_bindgen_test]
fn draws_exactly_the_encoded_matrix() {
    let code = qrcode(QRCodeProps {
        value: REPORT_URL.to_string(),
        size: 180,
        error_level: QRCodeErrorLevel::Medium,
        include_margin: true,
        class: Some("report-code".into()),
        ..Default::default()
    });

    let matrix = qrcode_matrix(REPORT_URL, QRCodeErrorLevel::Medium, true).unwrap();
    assert_eq!(code.class_name(), "qrcode report-code");
    assert_eq!(
        code.get_attribute("data-value").as_deref(),
        Some(REPORT_URL)
    );
    assert_eq!(code.get_attribute("data-error-level").as_deref(), Some("M"));
    assert_eq!(
        code.get_attribute("data-modules"),
        Some(matrix.modules.to_string())
    );
    assert!(code.get_attribute("data-error").is_none());

    let svg = code.query_selector("svg").unwrap().unwrap();
    assert_eq!(
        svg.get_attribute("viewBox"),
        Some(format!("0 0 {0} {0}", matrix.extent()))
    );
    assert_eq!(svg.get_attribute("width").as_deref(), Some("180"));
    assert_eq!(svg.get_attribute("role").as_deref(), Some("img"));
    assert_eq!(
        svg.get_attribute("aria-label"),
        Some(format!("QR code for {REPORT_URL}"))
    );
    assert_eq!(svg.get_attribute("data-quiet-zone").as_deref(), Some("4"));

    assert_eq!(
        drawn_modules(&code),
        matrix.dark_modules().collect::<Vec<_>>()
    );
}

#[wasm_bindgen_test]
fn the_light_ground_is_drawn_under_the_modules() {
    let code = qrcode(QRCodeProps {
        value: REPORT_URL.to_string(),
        bg_color: Some("#fffdf7".into()),
        fg_color: Some("#101820".into()),
        include_margin: true,
        ..Default::default()
    });

    let background = code
        .query_selector("[data-role='background']")
        .unwrap()
        .unwrap();
    assert_eq!(background.get_attribute("fill").as_deref(), Some("#fffdf7"));
    assert_eq!(background.get_attribute("width").as_deref(), Some("100%"));

    let modules = code
        .query_selector("[data-role='modules']")
        .unwrap()
        .unwrap();
    assert_eq!(modules.get_attribute("fill").as_deref(), Some("#101820"));

    // The ground has to precede the modules, or it would paint over them.
    let children = code.query_selector("svg").unwrap().unwrap().children();
    assert_eq!(
        children
            .item(0)
            .unwrap()
            .get_attribute("data-role")
            .as_deref(),
        Some("background")
    );
}

#[wasm_bindgen_test]
fn a_size_of_zero_falls_back_to_a_scannable_size() {
    let code = qrcode(QRCodeProps {
        value: REPORT_URL.to_string(),
        include_margin: true,
        ..Default::default()
    });

    let matrix = qrcode_matrix(REPORT_URL, QRCodeErrorLevel::Low, true).unwrap();
    let svg = code.query_selector("svg").unwrap().unwrap();
    let width: u32 = svg.get_attribute("width").unwrap().parse().unwrap();
    assert_eq!(width, matrix.extent() * 6);
    assert!(width >= matrix.extent());
}

#[wasm_bindgen_test]
fn margin_free_codes_start_at_the_origin() {
    let code = qrcode(QRCodeProps {
        value: "domius".to_string(),
        include_margin: false,
        ..Default::default()
    });

    let svg = code.query_selector("svg").unwrap().unwrap();
    assert_eq!(svg.get_attribute("data-quiet-zone").as_deref(), Some("0"));
    let modules = drawn_modules(&code);
    // A finder pattern always occupies the top-left corner.
    assert!(modules.contains(&(0, 0)));
}

#[wasm_bindgen_test]
fn an_unencodable_value_reports_instead_of_panicking() {
    let code = qrcode(QRCodeProps {
        value: "x".repeat(8000),
        error_level: QRCodeErrorLevel::High,
        ..Default::default()
    });

    assert!(code.get_attribute("data-error").is_some());
    assert!(code.query_selector("svg").unwrap().is_none());
}
