//! hello-world — a Domius WASM application demonstrating core features.
//!
//! Features shown:
//! - Reactive signals (`signal`, `create_effect`)
//! - Counter with increment/decrement
//! - Dynamic list add/remove
//! - DOM construction via `web_sys`
//! - Scoped CSS (via data-domus attribute convention)

use domius_core::signal::signal;
use domius_core::effect::create_effect;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlButtonElement, HtmlInputElement};

fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

fn el(tag: &str) -> Element {
    document().create_element(tag).unwrap()
}

fn text_node(text: &str) -> web_sys::Text {
    document().create_text_node(text)
}

// ---------------------------------------------------------------------------
// Counter section
// ---------------------------------------------------------------------------

fn build_counter(app: &Element) {
    let section = el("section");
    section.set_attribute("data-domius-scope", "counter").unwrap();

    let h2 = el("h2");
    h2.set_text_content(Some("Counter"));
    section.append_child(&h2).unwrap();

    let count = signal(0i32);
    let set_count = count.clone();

    // Display node
    let display = el("span");
    display.set_attribute("id", "counter-display").unwrap();
    display.set_text_content(Some("0"));
    section.append_child(&display).unwrap();

    // Reactive update
    {
        let display = display.clone();
        let count = count.clone();
        create_effect(move || {
            let val = count.get();
            display.set_text_content(Some(&val.to_string()));
        });
    }

    // Buttons
    let row = el("div");
    row.set_attribute("class", "btn-row").unwrap();

    let dec: HtmlButtonElement = el("button").dyn_into().unwrap();
    dec.set_text_content(Some("−"));
    {
        let set_count = set_count.clone();
        let count = count.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            set_count.set(count.get() - 1);
        }) as Box<dyn Fn(web_sys::MouseEvent)>);
        dec.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    let inc: HtmlButtonElement = el("button").dyn_into().unwrap();
    inc.set_text_content(Some("+"));
    {
        let set_count = set_count.clone();
        let count = count.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            set_count.set(count.get() + 1);
        }) as Box<dyn Fn(web_sys::MouseEvent)>);
        inc.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    let reset: HtmlButtonElement = el("button").dyn_into().unwrap();
    reset.set_text_content(Some("Reset"));
    {
        let set_count = set_count.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            set_count.set(0);
        }) as Box<dyn Fn(web_sys::MouseEvent)>);
        reset.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    row.append_child(&dec).unwrap();
    row.append_child(&inc).unwrap();
    row.append_child(&reset).unwrap();
    section.append_child(&row).unwrap();
    app.append_child(&section).unwrap();
}

// ---------------------------------------------------------------------------
// Todo list section
// ---------------------------------------------------------------------------

fn build_todo_list(app: &Element) {
    let section = el("section");
    section.set_attribute("data-domius-scope", "todo").unwrap();

    let h2 = el("h2");
    h2.set_text_content(Some("Todo List"));
    section.append_child(&h2).unwrap();

    let input_row = el("div");
    input_row.set_attribute("class", "input-row").unwrap();

    let input: HtmlInputElement = el("input").dyn_into().unwrap();
    input.set_attribute("type", "text").unwrap();
    input.set_attribute("placeholder", "New item…").unwrap();
    input.set_attribute("id", "todo-input").unwrap();

    let add_btn: HtmlButtonElement = el("button").dyn_into().unwrap();
    add_btn.set_text_content(Some("Add"));

    let list_el = el("ul");
    list_el.set_attribute("id", "todo-list").unwrap();

    {
        let input = input.clone();
        let list_el = list_el.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let val = input.value();
            let trimmed = val.trim().to_string();
            if trimmed.is_empty() {
                return;
            }
            input.set_value("");

            let li = el("li");
            let span = el("span");
            span.set_text_content(Some(&trimmed));

            let del: HtmlButtonElement = el("button").dyn_into().unwrap();
            del.set_text_content(Some("✕"));
            del.set_attribute("class", "del-btn").unwrap();

            let li_clone = li.clone();
            let del_cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                li_clone.remove();
            }) as Box<dyn Fn(web_sys::MouseEvent)>);
            del.set_onclick(Some(del_cb.as_ref().unchecked_ref()));
            del_cb.forget();

            li.append_child(&span).unwrap();
            li.append_child(&del).unwrap();
            list_el.append_child(&li).unwrap();
        }) as Box<dyn Fn(web_sys::MouseEvent)>);
        add_btn.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    input_row.append_child(&input).unwrap();
    input_row.append_child(&add_btn).unwrap();
    section.append_child(&input_row).unwrap();
    section.append_child(&list_el).unwrap();
    app.append_child(&section).unwrap();
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[wasm_bindgen(start)]
pub fn main() {
    domius_web::init();

    let document = document();
    let app = document.get_element_by_id("app").expect("#app not found");

    let heading = el("h1");
    heading.set_text_content(Some("Domius — Hello World"));
    app.append_child(&heading).unwrap();

    build_counter(&app);
    build_todo_list(&app);
}
