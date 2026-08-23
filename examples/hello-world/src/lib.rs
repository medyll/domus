//! hello-world — a Domius WASM application demonstrating core features.
//!
//! Features shown:
//! - Reactive signals (`signal`, `create_effect`)
//! - Counter with increment/decrement
//! - Dynamic list add/remove
//! - Declarative DOM construction via `domus!`
//! - Scoped CSS (via data-domus attribute convention)

use domius_core::signal::signal;
use domius_web::domus;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlButtonElement, HtmlInputElement};

fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

fn el(tag: &str) -> Element {
    document().create_element(tag).unwrap()
}

// ---------------------------------------------------------------------------
// Counter section
// ---------------------------------------------------------------------------

fn build_counter(app: &Element) {
    let count = signal(0i32);
    let section = domus! {
        section(class: "counter") {
            h2 { "Counter" }
            span(id: "counter-display") { {count.get()} }
            div(class: "btn-row") {
                button(on_click: {move |_| count.set(count.get() - 1)}) { "−" }
                button(on_click: {move |_| count.set(count.get() + 1)}) { "+" }
                button(on_click: {move |_| count.set(0)}) { "Reset" }
            }
        }
    };

    section
        .set_attribute("data-domius-scope", "counter")
        .unwrap();
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
