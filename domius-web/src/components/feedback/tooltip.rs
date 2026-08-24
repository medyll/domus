//! Tooltip component - Contextual hints on hover and focus.

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

/// Tooltip position.
#[derive(Clone, PartialEq, Debug)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    BottomStart,
    BottomEnd,
}

impl TooltipPosition {
    fn token(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
            Self::TopStart => "top-start",
            Self::TopEnd => "top-end",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
        }
    }
}

impl Default for TooltipPosition {
    fn default() -> Self {
        Self::Top
    }
}

/// Props for the Tooltip component.
#[derive(Clone)]
pub struct TooltipProps {
    pub content: String,
    pub position: TooltipPosition,
    pub delay: u64,
    pub disabled: bool,
    pub children: Element,
    pub class: Option<String>,
}

impl Default for TooltipProps {
    fn default() -> Self {
        Self {
            content: String::new(),
            position: TooltipPosition::default(),
            delay: 200,
            disabled: false,
            children: web_sys::window()
                .expect("no window")
                .document()
                .expect("no document")
                .create_element("span")
                .expect("create tooltip placeholder"),
            class: None,
        }
    }
}

thread_local! {
    /// Tooltip ids must be unique for aria-describedby to point anywhere.
    static TOOLTIP_SEQUENCE: Cell<u32> = const { Cell::new(0) };
}

fn next_tooltip_id() -> String {
    let sequence = TOOLTIP_SEQUENCE.with(|counter| {
        let next = counter.get().wrapping_add(1);
        counter.set(next);
        next
    });
    format!("domius-tooltip-{sequence}")
}

/// Tooltip component.
pub struct Tooltip;

impl Tooltip {
    /// Wrap `children` so a hint appears on hover and on focus.
    ///
    /// The hint is bound to the trigger with `aria-describedby`, shows without
    /// delay for keyboard users, and answers Escape, so it is not a hint only a
    /// mouse can reach.
    pub fn create(props: TooltipProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let wrapper = document
            .create_element("span")
            .expect("create tooltip wrapper");
        wrapper.set_class_name("domius-tooltip-wrapper");
        wrapper
            .append_child(&props.children)
            .expect("append tooltip trigger");

        if props.disabled {
            wrapper
                .set_attribute("data-disabled", "true")
                .expect("mark tooltip disabled");
            return wrapper;
        }

        let id = next_tooltip_id();
        let hint = build_hint(&document, &props, &id);
        // The trigger is what a reader lands on, so the hint describes it.
        props
            .children
            .set_attribute("aria-describedby", &id)
            .expect("bind tooltip to its trigger");
        // A trigger that cannot take focus can never show a hint by keyboard.
        if !is_focusable(&props.children) {
            props
                .children
                .set_attribute("tabindex", "0")
                .expect("make tooltip trigger focusable");
        }
        wrapper.append_child(&hint).expect("append tooltip hint");
        let pending = Rc::new(Cell::new(None));

        // Hover waits, because a pointer crossing a control did not ask for a
        // hint; focus does not, because it did.
        listen(
            &wrapper,
            "mouseenter",
            &hint,
            Visibility::Show(props.delay),
            Rc::clone(&pending),
        );
        listen(
            &wrapper,
            "mouseleave",
            &hint,
            Visibility::Hide,
            Rc::clone(&pending),
        );
        listen(
            &wrapper,
            "focusin",
            &hint,
            Visibility::Show(0),
            Rc::clone(&pending),
        );
        listen(
            &wrapper,
            "focusout",
            &hint,
            Visibility::Hide,
            Rc::clone(&pending),
        );
        listen_for_escape(&wrapper, &hint, pending);

        wrapper
    }
}

fn build_hint(document: &Document, props: &TooltipProps, id: &str) -> Element {
    let hint = document.create_element("div").expect("create tooltip");
    let mut classes = vec!["domius-tooltip".to_string()];
    classes.push(format!("domius-tooltip-{}", props.position.token()));
    if let Some(class) = props.class.as_deref() {
        classes.push(class.to_string());
    }
    hint.set_class_name(&classes.join(" "));
    hint.set_id(id);
    hint.set_attribute("role", "tooltip")
        .expect("set tooltip role");
    hint.set_attribute("data-position", props.position.token())
        .expect("expose tooltip position");
    hint.set_text_content(Some(&props.content));
    set_visible(&hint, false);
    hint
}

/// What a listener should do to the hint.
#[derive(Clone, Copy)]
enum Visibility {
    Show(u64),
    Hide,
}

fn listen(
    wrapper: &Element,
    event: &str,
    hint: &Element,
    visibility: Visibility,
    pending: Rc<Cell<Option<i32>>>,
) {
    let hint = hint.clone();
    let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| match visibility {
        Visibility::Hide => {
            cancel_pending(&pending);
            set_visible(&hint, false);
        }
        Visibility::Show(0) => {
            cancel_pending(&pending);
            set_visible(&hint, true);
        }
        Visibility::Show(delay) => {
            cancel_pending(&pending);
            let hint = hint.clone();
            let scheduled = Rc::clone(&pending);
            let later = Closure::once_into_js(move || {
                scheduled.set(None);
                set_visible(&hint, true);
            });
            if let Some(window) = web_sys::window() {
                if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    later.unchecked_ref(),
                    delay as i32,
                ) {
                    pending.set(Some(id));
                }
            }
        }
    });
    wrapper
        .add_event_listener_with_callback(event, handler.as_ref().unchecked_ref())
        .expect("listen for tooltip event");
    handler.forget();
}

fn listen_for_escape(wrapper: &Element, hint: &Element, pending: Rc<Cell<Option<i32>>>) {
    let hint = hint.clone();
    let handler =
        Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "Escape" {
                cancel_pending(&pending);
                set_visible(&hint, false);
            }
        });
    wrapper
        .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
        .expect("listen for tooltip escape");
    handler.forget();
}

fn cancel_pending(pending: &Cell<Option<i32>>) {
    if let Some(id) = pending.take() {
        if let Some(window) = web_sys::window() {
            window.clear_timeout_with_handle(id);
        }
    }
}

fn set_visible(hint: &Element, visible: bool) {
    hint.set_attribute("data-visible", &visible.to_string())
        .expect("expose tooltip visibility");
    hint.set_attribute("aria-hidden", &(!visible).to_string())
        .expect("hide tooltip from readers");
}

/// Elements that already take focus, and so need no tabindex of their own.
fn is_focusable(element: &Element) -> bool {
    if element.has_attribute("tabindex") {
        return true;
    }
    matches!(
        element.tag_name().to_ascii_lowercase().as_str(),
        "a" | "button" | "input" | "select" | "textarea" | "summary"
    )
}
