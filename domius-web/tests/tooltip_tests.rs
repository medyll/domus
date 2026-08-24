#![cfg(target_arch = "wasm32")]

mod test_utils;

use domius_web::components::feedback::tooltip::{Tooltip, TooltipPosition, TooltipProps};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn trigger(tag: &str, text: &str) -> web_sys::Element {
    let element = test_utils::document()
        .create_element(tag)
        .expect("create trigger");
    element.set_text_content(Some(text));
    element
}

fn dispatch(element: &web_sys::Element, event: &str) {
    let event = web_sys::Event::new(event).expect("create event");
    element.dispatch_event(&event).expect("dispatch event");
}

fn hint(wrapper: &web_sys::Element) -> web_sys::Element {
    wrapper.query_selector(".domius-tooltip").unwrap().unwrap()
}

fn visible(wrapper: &web_sys::Element) -> bool {
    hint(wrapper).get_attribute("data-visible").as_deref() == Some("true")
}

#[wasm_bindgen_test]
fn the_hint_describes_its_trigger() {
    let button = trigger("button", "142 ms");
    let wrapper = Tooltip::create(TooltipProps {
        content: "Median latency over the last hour".into(),
        position: TooltipPosition::BottomEnd,
        children: button.clone(),
        class: Some("latency".into()),
        ..Default::default()
    });

    let hint = hint(&wrapper);
    assert_eq!(hint.get_attribute("role").as_deref(), Some("tooltip"));
    assert_eq!(
        hint.text_content(),
        Some("Median latency over the last hour".to_string())
    );
    assert_eq!(
        hint.get_attribute("data-position").as_deref(),
        Some("bottom-end")
    );
    assert!(hint.class_name().contains("domius-tooltip-bottom-end"));
    assert!(hint.class_name().contains("latency"));

    // The reader reaches the hint through the trigger, not by luck.
    assert_eq!(
        button.get_attribute("aria-describedby"),
        hint.get_attribute("id")
    );
    assert!(!hint.id().is_empty());
    assert!(!visible(&wrapper));
    assert_eq!(hint.get_attribute("aria-hidden").as_deref(), Some("true"));
}

#[wasm_bindgen_test]
fn two_tooltips_do_not_share_an_id() {
    let first = Tooltip::create(TooltipProps {
        content: "one".into(),
        children: trigger("button", "1"),
        ..Default::default()
    });
    let second = Tooltip::create(TooltipProps {
        content: "two".into(),
        children: trigger("button", "2"),
        ..Default::default()
    });

    assert_ne!(hint(&first).id(), hint(&second).id());
}

#[wasm_bindgen_test]
fn focus_shows_the_hint_without_waiting() {
    let wrapper = Tooltip::create(TooltipProps {
        content: "Median latency".into(),
        children: trigger("button", "142 ms"),
        delay: 5_000,
        ..Default::default()
    });

    dispatch(&wrapper, "focusin");
    assert!(
        visible(&wrapper),
        "a keyboard user should not wait out a hover delay"
    );
    assert_eq!(
        hint(&wrapper).get_attribute("aria-hidden").as_deref(),
        Some("false")
    );

    dispatch(&wrapper, "focusout");
    assert!(!visible(&wrapper));
}

#[wasm_bindgen_test]
fn hovering_without_delay_shows_the_hint() {
    let wrapper = Tooltip::create(TooltipProps {
        content: "Median latency".into(),
        children: trigger("button", "142 ms"),
        delay: 0,
        ..Default::default()
    });

    dispatch(&wrapper, "mouseenter");
    assert!(visible(&wrapper));

    dispatch(&wrapper, "mouseleave");
    assert!(!visible(&wrapper));
}

#[wasm_bindgen_test]
fn a_delayed_hint_waits_for_its_delay() {
    let wrapper = Tooltip::create(TooltipProps {
        content: "Median latency".into(),
        children: trigger("button", "142 ms"),
        delay: 5_000,
        ..Default::default()
    });

    dispatch(&wrapper, "mouseenter");
    assert!(!visible(&wrapper), "the hint should not appear immediately");
}

#[wasm_bindgen_test]
fn escape_dismisses_a_shown_hint() {
    let wrapper = Tooltip::create(TooltipProps {
        content: "Median latency".into(),
        children: trigger("button", "142 ms"),
        delay: 0,
        ..Default::default()
    });

    dispatch(&wrapper, "focusin");
    assert!(visible(&wrapper));

    test_utils::simulate_key_press(&wrapper, "Escape");
    assert!(!visible(&wrapper));
}

#[wasm_bindgen_test]
fn a_trigger_that_cannot_take_focus_is_given_a_stop() {
    let plain = trigger("span", "142 ms");
    Tooltip::create(TooltipProps {
        content: "Median latency".into(),
        children: plain.clone(),
        ..Default::default()
    });
    assert_eq!(plain.get_attribute("tabindex").as_deref(), Some("0"));

    // Controls that already take focus keep their own place in the order.
    let button = trigger("button", "142 ms");
    Tooltip::create(TooltipProps {
        content: "Median latency".into(),
        children: button.clone(),
        ..Default::default()
    });
    assert!(button.get_attribute("tabindex").is_none());
}

#[wasm_bindgen_test]
fn a_disabled_tooltip_renders_only_its_trigger() {
    let button = trigger("button", "142 ms");
    let wrapper = Tooltip::create(TooltipProps {
        content: "Median latency".into(),
        children: button.clone(),
        disabled: true,
        ..Default::default()
    });

    assert_eq!(
        wrapper.get_attribute("data-disabled").as_deref(),
        Some("true")
    );
    assert!(wrapper.query_selector(".domius-tooltip").unwrap().is_none());
    assert!(button.get_attribute("aria-describedby").is_none());
}
