//! Rating component - Star/heart rating display.

use domius_core::signal::{signal, Signal};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, MouseEvent};

/// Rating icon type.
#[derive(Clone, PartialEq)]
pub enum RatingIcon {
    Star,
    Heart,
    Thumb,
    Custom(String),
}

impl Default for RatingIcon {
    fn default() -> Self {
        Self::Star
    }
}

/// Props for the Rating component.
pub struct RatingProps {
    pub value: Signal<u8>,
    pub max: u8,
    pub icon: RatingIcon,
    pub readonly: bool,
    pub allow_half: bool,
    pub allow_clear: bool,
    pub size: u32,
    pub color: Option<String>,
    pub on_change: Option<Box<dyn Fn(u8)>>,
    pub class: Option<String>,
}

impl Default for RatingProps {
    fn default() -> Self {
        Self {
            value: signal(0),
            max: 5,
            icon: RatingIcon::default(),
            readonly: false,
            allow_half: false,
            allow_clear: false,
            size: 24,
            color: None,
            on_change: None,
            class: None,
        }
    }
}

/// Rating component.
pub struct Rating;

impl Rating {
    /// Create a rating element.
    pub fn create(props: RatingProps) -> (Element, Signal<u8>) {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let container: HtmlElement = document.create_element("div").unwrap().dyn_into().unwrap();
        container.set_attribute("class", "domius-rating").unwrap();

        let icon_char = match props.icon {
            RatingIcon::Star => "★",
            RatingIcon::Heart => "♥",
            RatingIcon::Thumb => "👍",
            RatingIcon::Custom(ref c) => c.as_str(),
        };

        for i in 1..=props.max {
            let icon_el: HtmlElement = document.create_element("span").unwrap().dyn_into().unwrap();

            let mut classes = vec!["domius-rating-icon".to_string()];
            if i <= props.value.get() {
                classes.push("domius-rating-filled".to_string());
            }
            icon_el.set_attribute("class", &classes.join(" ")).unwrap();
            icon_el.set_attribute("data-value", &i.to_string()).unwrap();
            icon_el.set_text_content(Some(icon_char));
            icon_el
                .set_attribute("style", &format!("font-size: {}px", props.size))
                .unwrap();

            if !props.readonly {
                let value_clone = props.value.clone();
                let on_change_clone = props.on_change.as_ref().map(|_| {
                    let handler = props.on_change.as_ref().unwrap();
                    let i_clone = i;
                    Closure::wrap(Box::new(move |_event: MouseEvent| {
                        handler(i_clone);
                    }) as Box<dyn FnMut(MouseEvent)>)
                });

                let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                    value_clone.set(i);
                }) as Box<dyn FnMut(MouseEvent)>);

                icon_el
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();

                if let Some(change_closure) = on_change_clone {
                    icon_el
                        .add_event_listener_with_callback(
                            "click",
                            change_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    change_closure.forget();
                }
            }

            container.append_child(&icon_el).unwrap();
        }

        (container.into(), props.value)
    }
}
