#![cfg(target_arch = "wasm32")]

use domius_web::components::pro::result::{Result, ResultAction, ResultProps, ResultStatus};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn renders_status_title_description_and_icon() {
    let result = Result::create(ResultProps {
        status: ResultStatus::Error,
        title: "Reports unavailable".into(),
        description: Some("The metric source returned nothing.".into()),
        class: Some("report-failure".into()),
        ..Default::default()
    });

    assert_eq!(result.tag_name(), "SECTION");
    assert_eq!(result.class_name(), "domius-result report-failure");
    assert_eq!(
        result.get_attribute("data-status").as_deref(),
        Some("error")
    );
    assert_eq!(result.get_attribute("role").as_deref(), Some("status"));
    assert_eq!(
        result
            .query_selector(".domius-result-title")
            .unwrap()
            .unwrap()
            .text_content(),
        Some("Reports unavailable".to_string())
    );
    assert_eq!(
        result
            .query_selector(".domius-result-description")
            .unwrap()
            .unwrap()
            .text_content(),
        Some("The metric source returned nothing.".to_string())
    );

    let icon = result
        .query_selector(".domius-result-icon")
        .unwrap()
        .unwrap();
    assert_eq!(icon.text_content(), Some("✕".to_string()));
    assert_eq!(icon.get_attribute("aria-hidden").as_deref(), Some("true"));
}

#[wasm_bindgen_test]
fn a_missing_description_leaves_no_empty_paragraph() {
    let result = Result::success("Report exported", None);

    assert_eq!(
        result.get_attribute("data-status").as_deref(),
        Some("success")
    );
    assert!(result
        .query_selector(".domius-result-description")
        .unwrap()
        .is_none());
    assert!(result
        .query_selector(".domius-result-actions")
        .unwrap()
        .is_none());
}

#[wasm_bindgen_test]
fn actions_render_as_real_links() {
    let result = Result::create(ResultProps {
        status: ResultStatus::Info,
        title: "No metrics in range".into(),
        actions: vec![
            ResultAction::new("Back to overview", "/overview").primary(),
            ResultAction::new("Open incidents", "/incidents"),
        ],
        ..Default::default()
    });

    let nav = result
        .query_selector(".domius-result-actions")
        .unwrap()
        .unwrap();
    assert_eq!(nav.tag_name(), "NAV");
    assert_eq!(
        nav.get_attribute("aria-label").as_deref(),
        Some("Result actions")
    );

    let links = result.query_selector_all("a.domius-result-action").unwrap();
    assert_eq!(links.length(), 2);

    let primary = result.query_selector("[data-primary]").unwrap().unwrap();
    assert_eq!(primary.get_attribute("href").as_deref(), Some("/overview"));
    assert_eq!(primary.text_content(), Some("Back to overview".to_string()));

    let secondary = result
        .query_selector("a[href='/incidents']")
        .unwrap()
        .unwrap();
    assert!(secondary.get_attribute("data-primary").is_none());
}

#[wasm_bindgen_test]
fn a_custom_status_keeps_its_own_token_and_icon() {
    let result = Result::create(ResultProps {
        status: ResultStatus::Custom("404".into()),
        title: "Page Not Found".into(),
        icon: Some("？".into()),
        ..Default::default()
    });

    assert_eq!(result.get_attribute("data-status").as_deref(), Some("404"));
    assert_eq!(
        result
            .query_selector(".domius-result-icon")
            .unwrap()
            .unwrap()
            .text_content(),
        Some("？".to_string())
    );
}

#[wasm_bindgen_test]
fn the_not_found_helper_offers_a_way_back() {
    let result = Result::not_found(vec![ResultAction::new("Back to overview", "/overview")]);

    assert_eq!(result.get_attribute("data-status").as_deref(), Some("404"));
    let link = result
        .query_selector("a.domius-result-action")
        .unwrap()
        .unwrap();
    assert_eq!(link.get_attribute("href").as_deref(), Some("/overview"));
}
