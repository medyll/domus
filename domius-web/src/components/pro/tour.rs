//! Tour component - User onboarding guide.

use std::rc::Rc;

use domius_core::signal::Signal;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

use crate::disposal::ViewScope;

/// Attribute set on the element a step points at, while that step is showing.
pub const TARGET_ATTRIBUTE: &str = "data-tour-target";

/// Tour step.
#[derive(Clone)]
pub struct TourStep {
    pub target_id: String,
    pub title: String,
    pub description: String,
    pub position: TourPosition,
}

/// Tour step position.
#[derive(Clone, PartialEq)]
pub enum TourPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl TourPosition {
    fn token(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

impl Default for TourPosition {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Props for the Tour component.
pub struct TourProps {
    pub steps: Vec<TourStep>,
    pub active: Signal<bool>,
    pub current_step: Signal<usize>,
    pub show_arrows: bool,
    pub show_indicators: bool,
    pub close_on_overlay: bool,
    pub on_finish: Option<Box<dyn Fn()>>,
    pub on_skip: Option<Box<dyn Fn()>>,
    pub on_step_change: Option<Box<dyn Fn(usize)>>,
    pub class: Option<String>,
}

impl Default for TourProps {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            active: domius_core::signal::signal(false),
            current_step: domius_core::signal::signal(0),
            show_arrows: true,
            show_indicators: true,
            close_on_overlay: true,
            on_finish: None,
            on_skip: None,
            on_step_change: None,
            class: None,
        }
    }
}

/// Everything the buttons need to move the tour along.
struct Controls {
    steps: usize,
    active: Signal<bool>,
    current_step: Signal<usize>,
    on_finish: Option<Rc<dyn Fn()>>,
    on_skip: Option<Rc<dyn Fn()>>,
    on_step_change: Option<Rc<dyn Fn(usize)>>,
}

impl Controls {
    fn go_to(&self, step: usize) {
        let step = step.min(self.steps.saturating_sub(1));
        if self.current_step.get() == step {
            return;
        }
        self.current_step.set(step);
        if let Some(callback) = &self.on_step_change {
            callback(step);
        }
    }

    fn next(&self) {
        let current = self.current_step.get();
        if current + 1 >= self.steps {
            self.finish();
        } else {
            self.go_to(current + 1);
        }
    }

    fn previous(&self) {
        self.go_to(self.current_step.get().saturating_sub(1));
    }

    fn finish(&self) {
        self.active.set(false);
        if let Some(callback) = &self.on_finish {
            callback();
        }
    }

    fn skip(&self) {
        self.active.set(false);
        if let Some(callback) = &self.on_skip {
            callback();
        }
    }
}

/// Tour component.
pub struct Tour;

impl Tour {
    /// Create a tour overlay element.
    ///
    /// The overlay follows its `active` and `current_step` signals: it shows one
    /// step at a time, marks the element that step points at with
    /// [`TARGET_ATTRIBUTE`], and hides itself when the tour ends. Placement is
    /// exposed as `data-position` rather than written as inline geometry, so the
    /// stylesheet decides where a bubble actually sits.
    pub fn create(props: TourProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let root = document.create_element("div").expect("create tour");
        let mut classes = vec!["domius-tour"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        root.set_class_name(&classes.join(" "));
        root.set_attribute("data-steps", &props.steps.len().to_string())
            .expect("set tour length");

        let overlay = document.create_element("div").expect("create tour overlay");
        overlay.set_class_name("domius-tour-overlay");
        overlay
            .set_attribute("data-role", "overlay")
            .expect("mark tour overlay");
        root.append_child(&overlay).expect("append tour overlay");

        let bubble = document.create_element("div").expect("create tour step");
        bubble.set_class_name("domius-tour-step");
        bubble
            .set_attribute("role", "dialog")
            .expect("set tour step role");
        bubble
            .set_attribute("aria-modal", "true")
            .expect("mark tour step modal");
        root.append_child(&bubble).expect("append tour step");

        let controls = Rc::new(Controls {
            steps: props.steps.len(),
            active: props.active.clone(),
            current_step: props.current_step.clone(),
            on_finish: props.on_finish.map(Rc::from),
            on_skip: props.on_skip.map(Rc::from),
            on_step_change: props.on_step_change.map(Rc::from),
        });

        if props.close_on_overlay {
            let dismiss = Rc::clone(&controls);
            let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| dismiss.skip());
            overlay
                .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                .expect("listen to tour overlay");
            handler.forget();
        }
        listen_for_escape(&root, Rc::clone(&controls));

        // The tour owns its scope, so removing the overlay stops it following
        // the signals it was built on.
        let scope = ViewScope::attach(&root);
        let steps = props.steps;
        let show_arrows = props.show_arrows;
        let show_indicators = props.show_indicators;
        let active = props.active;
        let current_step = props.current_step;
        let root_for_effect = root.clone();
        scope.effect(move || {
            let showing = active.get() && !steps.is_empty();
            let index = current_step.get().min(steps.len().saturating_sub(1));
            root_for_effect
                .set_attribute("data-active", &showing.to_string())
                .expect("expose tour state");
            set_hidden(&root_for_effect, !showing);
            clear_targets(&document);
            bubble.set_text_content(None);

            if !showing {
                root_for_effect
                    .remove_attribute("data-step")
                    .expect("clear tour step");
                return;
            }

            let step = &steps[index];
            root_for_effect
                .set_attribute("data-step", &index.to_string())
                .expect("expose tour step");
            bubble
                .set_attribute("data-position", step.position.token())
                .expect("expose tour position");
            bubble
                .set_attribute("data-target", &step.target_id)
                .expect("expose tour target");
            mark_target(&document, &step.target_id);
            fill_bubble(
                &document,
                &bubble,
                step,
                index,
                &steps,
                show_arrows,
                show_indicators,
                &controls,
            );
        });

        root
    }
}

#[allow(clippy::too_many_arguments)]
fn fill_bubble(
    document: &Document,
    bubble: &Element,
    step: &TourStep,
    index: usize,
    steps: &[TourStep],
    show_arrows: bool,
    show_indicators: bool,
    controls: &Rc<Controls>,
) {
    let title = document.create_element("h2").expect("create tour title");
    title.set_class_name("domius-tour-title");
    let title_id = format!("domius-tour-title-{index}");
    title.set_id(&title_id);
    title.set_text_content(Some(&step.title));
    bubble.append_child(&title).expect("append tour title");
    bubble
        .set_attribute("aria-labelledby", &title_id)
        .expect("label tour step");

    let description = document
        .create_element("p")
        .expect("create tour description");
    description.set_class_name("domius-tour-description");
    description.set_text_content(Some(&step.description));
    bubble
        .append_child(&description)
        .expect("append tour description");

    if show_indicators {
        let indicators = document
            .create_element("ol")
            .expect("create tour indicators");
        indicators.set_class_name("domius-tour-indicators");
        indicators
            .set_attribute("data-role", "indicators")
            .expect("mark tour indicators");
        for (position, other) in steps.iter().enumerate() {
            let dot = document
                .create_element("li")
                .expect("create tour indicator");
            dot.set_attribute("data-index", &position.to_string())
                .expect("index tour indicator");
            if position == index {
                dot.set_attribute("data-current", "true")
                    .expect("mark current indicator");
                dot.set_attribute("aria-current", "step")
                    .expect("announce current indicator");
            }
            dot.set_text_content(Some(&other.title));
            indicators
                .append_child(&dot)
                .expect("append tour indicator");
        }
        bubble
            .append_child(&indicators)
            .expect("append tour indicators");
    }

    let actions = document.create_element("div").expect("create tour actions");
    actions.set_class_name("domius-tour-actions");
    append_action(document, &actions, "skip", "Skip", {
        let controls = Rc::clone(controls);
        move || controls.skip()
    });
    if show_arrows && index > 0 {
        append_action(document, &actions, "previous", "Back", {
            let controls = Rc::clone(controls);
            move || controls.previous()
        });
    }
    let last = index + 1 >= steps.len();
    append_action(
        document,
        &actions,
        if last { "finish" } else { "next" },
        if last { "Finish" } else { "Next" },
        {
            let controls = Rc::clone(controls);
            move || controls.next()
        },
    );
    bubble.append_child(&actions).expect("append tour actions");
}

fn append_action<F: Fn() + 'static>(
    document: &Document,
    actions: &Element,
    action: &str,
    label: &str,
    on_activate: F,
) {
    let button = document
        .create_element("button")
        .expect("create tour action");
    button
        .set_attribute("type", "button")
        .expect("type tour action");
    button
        .set_attribute("data-action", action)
        .expect("mark tour action");
    button.set_text_content(Some(label));
    let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| on_activate());
    button
        .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        .expect("listen to tour action");
    handler.forget();
    actions.append_child(&button).expect("append tour action");
}

/// A modal overlay that cannot be dismissed with the keyboard is a trap.
fn listen_for_escape(root: &Element, controls: Rc<Controls>) {
    let handler =
        Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "Escape" {
                controls.skip();
            }
        });
    root.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
        .expect("listen for tour escape");
    handler.forget();
}

fn mark_target(document: &Document, target_id: &str) {
    if let Some(target) = document.get_element_by_id(target_id) {
        target
            .set_attribute(TARGET_ATTRIBUTE, "true")
            .expect("mark tour target");
    }
}

fn clear_targets(document: &Document) {
    let marked = document
        .query_selector_all(&format!("[{TARGET_ATTRIBUTE}]"))
        .expect("query tour targets");
    for index in 0..marked.length() {
        if let Some(element) = marked
            .item(index)
            .and_then(|node| node.dyn_into::<Element>().ok())
        {
            element
                .remove_attribute(TARGET_ATTRIBUTE)
                .expect("clear tour target");
        }
    }
}

fn set_hidden(element: &Element, hidden: bool) {
    if hidden {
        element.set_attribute("hidden", "").expect("hide tour");
    } else {
        element.remove_attribute("hidden").expect("show tour");
    }
}
