//! Modal/Dialog component - Overlay dialog window.

use domius_core::signal::Signal;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, MouseEvent};

use crate::hooks::use_click_outside;
use crate::hooks::use_keyboard;
use crate::hooks::use_keyboard::KeyboardConfig;

/// Modal size.
#[derive(Clone, PartialEq, Debug)]
pub enum ModalSize {
    Sm,
    Md,
    Lg,
    Xl,
    Full,
}

impl Default for ModalSize {
    fn default() -> Self {
        Self::Md
    }
}

/// Props for the Modal component.
pub struct ModalProps {
    pub open: Signal<bool>,
    pub title: Option<String>,
    pub content: String,
    pub closable: bool,
    pub close_on_overlay: bool,
    pub close_on_escape: bool,
    pub size: ModalSize,
    pub show_footer: bool,
    pub confirm_text: Option<String>,
    pub cancel_text: Option<String>,
    pub on_close: Option<Box<dyn Fn()>>,
    pub on_confirm: Option<Box<dyn Fn()>>,
    pub class: Option<String>,
}

impl Default for ModalProps {
    fn default() -> Self {
        Self {
            open: domius_core::signal::signal(false),
            title: None,
            content: String::new(),
            closable: true,
            close_on_overlay: true,
            close_on_escape: true,
            size: ModalSize::default(),
            show_footer: false,
            confirm_text: Some("OK".to_string()),
            cancel_text: Some("Cancel".to_string()),
            on_close: None,
            on_confirm: None,
            class: None,
        }
    }
}

/// Modal component.
pub struct Modal;

impl Modal {
    /// Create a modal element.
    pub fn create(props: ModalProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        // Create backdrop
        let backdrop: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();
        backdrop.set_attribute("class", "domius-modal-backdrop").unwrap();

        // Create modal container
        let modal: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();
        
        let mut classes = vec!["domius-modal".to_string()];
        classes.push(format!("domius-modal-{:?}", props.size).to_lowercase());
        if let Some(class) = &props.class {
            classes.push(class.clone());
        }
        modal.set_attribute("class", &classes.join(" ")).unwrap();
        modal.set_attribute("role", "dialog").unwrap();
        modal.set_attribute("aria-modal", "true").unwrap();

        // Create header
        if props.title.is_some() || props.closable {
            let header: HtmlElement = document
                .create_element("div")
                .unwrap()
                .dyn_into()
                .unwrap();
            header.set_attribute("class", "domius-modal-header").unwrap();

            if let Some(title) = &props.title {
                let title_el: HtmlElement = document
                    .create_element("h2")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                title_el.set_attribute("class", "domius-modal-title").unwrap();
                title_el.set_text_content(Some(title));
                header.append_child(&title_el).unwrap();
            }

            if props.closable {
                let close_btn: HtmlElement = document
                    .create_element("button")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                close_btn.set_attribute("class", "domius-modal-close").unwrap();
                close_btn.set_attribute("aria-label", "Close").unwrap();
                close_btn.set_text_content(Some("×"));

                if let Some(handler) = props.on_close.as_ref() {
                    let handler_ref = handler;
                    let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                        handler_ref();
                    }) as Box<dyn FnMut(MouseEvent)>);
                    close_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                        .unwrap();
                    closure.forget();
                }

                header.append_child(&close_btn).unwrap();
            }

            modal.append_child(&header).unwrap();
        }

        // Create body
        let body: HtmlElement = document
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap();
        body.set_attribute("class", "domius-modal-body").unwrap();
        body.set_text_content(Some(&props.content));
        modal.append_child(&body).unwrap();

        // Create footer if needed
        if props.show_footer {
            let footer: HtmlElement = document
                .create_element("div")
                .unwrap()
                .dyn_into()
                .unwrap();
            footer.set_attribute("class", "domius-modal-footer").unwrap();

            // Cancel button
            if let Some(cancel_text) = &props.cancel_text {
                let cancel_btn: HtmlElement = document
                    .create_element("button")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                cancel_btn.set_attribute("class", "domius-btn domius-btn-secondary").unwrap();
                cancel_btn.set_text_content(Some(cancel_text));

                if let Some(handler) = props.on_close.as_ref() {
                    let handler_ref = handler;
                    let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                        handler_ref();
                    }) as Box<dyn FnMut(MouseEvent)>);
                    cancel_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                        .unwrap();
                    closure.forget();
                }

                footer.append_child(&cancel_btn).unwrap();
            }

            // Confirm button
            if let Some(confirm_text) = &props.confirm_text {
                let confirm_btn: HtmlElement = document
                    .create_element("button")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                confirm_btn.set_attribute("class", "domius-btn domius-btn-primary").unwrap();
                confirm_btn.set_text_content(Some(confirm_text));

                if let Some(handler) = props.on_confirm.as_ref() {
                    let handler_ref = handler;
                    let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                        handler_ref();
                    }) as Box<dyn FnMut(MouseEvent)>);
                    confirm_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                        .unwrap();
                    closure.forget();
                }

                footer.append_child(&confirm_btn).unwrap();
            }

            modal.append_child(&footer).unwrap();
        }

        backdrop.append_child(&modal).unwrap();

        // Handle close on overlay click
        if props.close_on_overlay {
            let on_close_ref = props.on_close.as_ref();
            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                if let Some(handler) = on_close_ref {
                    if event.target().unwrap().dyn_ref::<web_sys::Element>()
                        .map(|el| el.class_list().contains("domius-modal-backdrop"))
                        .unwrap_or(false)
                    {
                        handler();
                    }
                }
            }) as Box<dyn FnMut(MouseEvent)>);
            
            backdrop.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }

        backdrop.into()
    }
}
