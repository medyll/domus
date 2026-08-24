#![cfg(target_arch = "wasm32")]

mod test_utils;

use std::cell::RefCell;
use std::rc::Rc;

use domius_core::signal::signal;
use domius_web::components::pro::tour::{
    Tour, TourPosition, TourProps, TourStep, TARGET_ATTRIBUTE,
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

fn steps() -> Vec<TourStep> {
    vec![
        TourStep {
            target_id: "tour-health".into(),
            title: "Service health".into(),
            description: "Every service and its current state.".into(),
            position: TourPosition::Bottom,
        },
        TourStep {
            target_id: "tour-alerts".into(),
            title: "Open alerts".into(),
            description: "What needs an operator right now.".into(),
            position: TourPosition::Right,
        },
        TourStep {
            target_id: "tour-trends".into(),
            title: "Trends".into(),
            description: "How the last hour behaved.".into(),
            position: TourPosition::Top,
        },
    ]
}

/// Put the step targets in the document so the tour can mark them.
fn targets() -> Vec<web_sys::Element> {
    ["tour-health", "tour-alerts", "tour-trends"]
        .iter()
        .map(|id| {
            let element = test_utils::create_test_container(id);
            wasm_bindgen::JsCast::unchecked_into::<web_sys::Element>(element)
        })
        .collect()
}

fn drop_targets(targets: &[web_sys::Element]) {
    for target in targets {
        test_utils::remove_element(target);
    }
}

fn action(tour: &web_sys::Element, name: &str) -> Option<web_sys::Element> {
    tour.query_selector(&format!("[data-action='{name}']"))
        .unwrap()
}

fn click(element: &web_sys::Element) {
    test_utils::simulate_click(element);
}

#[wasm_bindgen_test]
fn an_inactive_tour_shows_nothing() {
    let tour = Tour::create(TourProps {
        steps: steps(),
        class: Some("help".into()),
        ..Default::default()
    });

    assert_eq!(tour.class_name(), "domius-tour help");
    assert_eq!(tour.get_attribute("data-steps").as_deref(), Some("3"));
    assert_eq!(tour.get_attribute("data-active").as_deref(), Some("false"));
    assert!(tour.has_attribute("hidden"));
    assert!(tour.get_attribute("data-step").is_none());
    assert!(action(&tour, "next").is_none());
}

#[wasm_bindgen_test]
fn an_active_tour_opens_on_its_first_step() {
    let targets = targets();
    let tour = Tour::create(TourProps {
        steps: steps(),
        active: signal(true),
        ..Default::default()
    });

    assert_eq!(tour.get_attribute("data-active").as_deref(), Some("true"));
    assert!(!tour.has_attribute("hidden"));
    assert_eq!(tour.get_attribute("data-step").as_deref(), Some("0"));

    let bubble = tour.query_selector(".domius-tour-step").unwrap().unwrap();
    assert_eq!(bubble.get_attribute("role").as_deref(), Some("dialog"));
    assert_eq!(bubble.get_attribute("aria-modal").as_deref(), Some("true"));
    assert_eq!(
        bubble.get_attribute("data-position").as_deref(),
        Some("bottom")
    );
    assert_eq!(
        bubble.get_attribute("data-target").as_deref(),
        Some("tour-health")
    );
    assert_eq!(
        bubble
            .query_selector(".domius-tour-title")
            .unwrap()
            .unwrap()
            .text_content(),
        Some("Service health".to_string())
    );
    // The dialog is named by the heading it shows.
    assert_eq!(
        bubble.get_attribute("aria-labelledby"),
        bubble
            .query_selector(".domius-tour-title")
            .unwrap()
            .unwrap()
            .get_attribute("id")
    );

    // Only the first step's target is marked.
    assert_eq!(
        targets[0].get_attribute(TARGET_ATTRIBUTE).as_deref(),
        Some("true")
    );
    assert!(targets[1].get_attribute(TARGET_ATTRIBUTE).is_none());

    // The first step has nowhere to go back to.
    assert!(action(&tour, "previous").is_none());
    assert!(action(&tour, "next").is_some());
    assert!(action(&tour, "finish").is_none());

    drop_targets(&targets);
}

#[wasm_bindgen_test]
async fn stepping_forward_moves_the_marked_target() {
    let targets = targets();
    let changed = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&changed);
    let tour = Tour::create(TourProps {
        steps: steps(),
        active: signal(true),
        on_step_change: Some(Box::new(move |step| recorded.borrow_mut().push(step))),
        ..Default::default()
    });

    click(&action(&tour, "next").unwrap());
    settle().await;

    assert_eq!(tour.get_attribute("data-step").as_deref(), Some("1"));
    assert_eq!(*changed.borrow(), vec![1]);
    assert!(targets[0].get_attribute(TARGET_ATTRIBUTE).is_none());
    assert_eq!(
        targets[1].get_attribute(TARGET_ATTRIBUTE).as_deref(),
        Some("true")
    );
    let bubble = tour.query_selector(".domius-tour-step").unwrap().unwrap();
    assert_eq!(
        bubble.get_attribute("data-position").as_deref(),
        Some("right")
    );

    // Now that there is a step behind, back appears.
    click(&action(&tour, "previous").unwrap());
    settle().await;
    assert_eq!(tour.get_attribute("data-step").as_deref(), Some("0"));
    assert_eq!(*changed.borrow(), vec![1, 0]);

    drop_targets(&targets);
}

#[wasm_bindgen_test]
async fn the_last_step_finishes_instead_of_advancing() {
    let targets = targets();
    let finished = Rc::new(RefCell::new(0));
    let counted = Rc::clone(&finished);
    let active = signal(true);
    let tour = Tour::create(TourProps {
        steps: steps(),
        active: active.clone(),
        current_step: signal(2),
        on_finish: Some(Box::new(move || *counted.borrow_mut() += 1)),
        ..Default::default()
    });

    assert!(action(&tour, "next").is_none());
    click(&action(&tour, "finish").unwrap());
    settle().await;

    assert_eq!(*finished.borrow(), 1);
    assert!(!active.get());
    assert!(tour.has_attribute("hidden"));
    // Nothing stays highlighted once the tour is over.
    assert!(targets
        .iter()
        .all(|t| t.get_attribute(TARGET_ATTRIBUTE).is_none()));

    drop_targets(&targets);
}

#[wasm_bindgen_test]
async fn skipping_closes_the_tour() {
    let targets = targets();
    let skipped = Rc::new(RefCell::new(0));
    let counted = Rc::clone(&skipped);
    let active = signal(true);
    let tour = Tour::create(TourProps {
        steps: steps(),
        active: active.clone(),
        on_skip: Some(Box::new(move || *counted.borrow_mut() += 1)),
        ..Default::default()
    });

    click(&action(&tour, "skip").unwrap());
    settle().await;

    assert_eq!(*skipped.borrow(), 1);
    assert!(!active.get());
    assert!(tour.has_attribute("hidden"));

    drop_targets(&targets);
}

#[wasm_bindgen_test]
async fn escape_leaves_the_tour() {
    let targets = targets();
    let active = signal(true);
    let tour = Tour::create(TourProps {
        steps: steps(),
        active: active.clone(),
        ..Default::default()
    });

    test_utils::simulate_key_press(&tour, "Escape");
    settle().await;
    assert!(
        !active.get(),
        "a modal overlay must be dismissible by keyboard"
    );
    assert!(tour.has_attribute("hidden"));

    drop_targets(&targets);
}

#[wasm_bindgen_test]
async fn clicking_the_overlay_leaves_only_when_allowed() {
    let targets = targets();
    let sticky = signal(true);
    let tour = Tour::create(TourProps {
        steps: steps(),
        active: sticky.clone(),
        close_on_overlay: false,
        ..Default::default()
    });
    click(
        &tour
            .query_selector("[data-role='overlay']")
            .unwrap()
            .unwrap(),
    );
    settle().await;
    assert!(sticky.get(), "the overlay should be inert when told to be");

    let dismissible = signal(true);
    let other = Tour::create(TourProps {
        steps: steps(),
        active: dismissible.clone(),
        close_on_overlay: true,
        ..Default::default()
    });
    click(
        &other
            .query_selector("[data-role='overlay']")
            .unwrap()
            .unwrap(),
    );
    settle().await;
    assert!(!dismissible.get());

    drop_targets(&targets);
}

#[wasm_bindgen_test]
fn indicators_track_the_current_step() {
    let targets = targets();
    let tour = Tour::create(TourProps {
        steps: steps(),
        active: signal(true),
        current_step: signal(1),
        ..Default::default()
    });

    let dots = tour
        .query_selector_all("[data-role='indicators'] li")
        .unwrap();
    assert_eq!(dots.length(), 3);
    let current = tour.query_selector("[data-current]").unwrap().unwrap();
    assert_eq!(current.get_attribute("data-index").as_deref(), Some("1"));
    assert_eq!(
        current.get_attribute("aria-current").as_deref(),
        Some("step")
    );

    let bare = Tour::create(TourProps {
        steps: steps(),
        active: signal(true),
        show_indicators: false,
        ..Default::default()
    });
    assert!(bare
        .query_selector("[data-role='indicators']")
        .unwrap()
        .is_none());

    drop_targets(&targets);
}

#[wasm_bindgen_test]
fn a_tour_without_steps_never_opens() {
    let tour = Tour::create(TourProps {
        steps: vec![],
        active: signal(true),
        ..Default::default()
    });

    assert_eq!(tour.get_attribute("data-steps").as_deref(), Some("0"));
    assert_eq!(tour.get_attribute("data-active").as_deref(), Some("false"));
    assert!(tour.has_attribute("hidden"));
}

#[wasm_bindgen_test]
async fn a_modal_tour_moves_traps_and_restores_focus() {
    use wasm_bindgen::JsCast;

    let targets = targets();
    let opener = test_utils::document()
        .create_element("button")
        .expect("create opener");
    opener.set_id("tour-opener");
    test_utils::document()
        .body()
        .unwrap()
        .append_child(&opener)
        .expect("attach opener");
    opener
        .dyn_ref::<web_sys::HtmlElement>()
        .unwrap()
        .focus()
        .expect("focus opener");

    let active = signal(false);
    let tour = Tour::create(TourProps {
        steps: steps(),
        active: active.clone(),
        ..Default::default()
    });
    test_utils::document()
        .body()
        .unwrap()
        .append_child(&tour)
        .expect("attach tour");
    active.set(true);
    settle().await;

    let bubble = tour.query_selector(".domius-tour-step").unwrap().unwrap();
    assert_eq!(
        bubble.get_attribute("aria-describedby"),
        bubble
            .query_selector(".domius-tour-description")
            .unwrap()
            .unwrap()
            .get_attribute("id")
    );
    assert_eq!(
        test_utils::document()
            .active_element()
            .unwrap()
            .get_attribute("data-action")
            .as_deref(),
        Some("next")
    );

    let next = action(&tour, "next").unwrap();
    next.dyn_ref::<web_sys::HtmlElement>()
        .unwrap()
        .focus()
        .unwrap();
    test_utils::simulate_key_press(&next, "Tab");
    assert_eq!(
        test_utils::document()
            .active_element()
            .unwrap()
            .get_attribute("data-action")
            .as_deref(),
        Some("skip"),
        "Tab from the last action should wrap to the first"
    );

    click(&action(&tour, "skip").unwrap());
    settle().await;
    assert_eq!(
        test_utils::document()
            .active_element()
            .map(|element| element.id()),
        Some("tour-opener".to_string())
    );

    tour.remove();
    opener.remove();
    drop_targets(&targets);
}
