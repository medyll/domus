#![cfg(target_arch = "wasm32")]

mod test_utils;

use domius_web::components::navigation::anchor::{anchor, AnchorLink, AnchorProps};
use domius_web::components::primitives::affix::{affix, AffixProps};
use domius_web::components::primitives::backtop::{backtop, BackTopProps};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// A tall scrolling container, so a test can actually move the reader.
struct Page {
    container: web_sys::Element,
    id: String,
}

impl Page {
    fn new(id: &str) -> Self {
        let container: web_sys::Element = test_utils::create_test_container(id).unchecked_into();
        container
            .set_attribute("style", "height: 200px; overflow: auto;")
            .expect("size scroll container");
        Self {
            container,
            id: format!("#{id}"),
        }
    }

    fn add_section(&self, id: &str, title: &str) {
        let section = test_utils::document()
            .create_element("section")
            .expect("create section");
        section.set_id(id);
        section
            .set_attribute("style", "height: 400px;")
            .expect("size section");
        section.set_text_content(Some(title));
        self.container
            .append_child(&section)
            .expect("append section");
    }

    fn scroll_to(&self, offset: i32) {
        self.container.set_scroll_top(offset);
        let event = web_sys::Event::new("scroll").expect("create scroll event");
        self.container
            .dispatch_event(&event)
            .expect("dispatch scroll");
    }

    fn selector(&self) -> Option<String> {
        Some(self.id.clone())
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        test_utils::remove_element(&self.container);
    }
}

#[wasm_bindgen_test]
fn backtop_stays_out_of_the_way_until_the_reader_has_scrolled() {
    let page = Page::new("backtop-page");
    page.add_section("backtop-one", "one");
    page.add_section("backtop-two", "two");

    let control = backtop(BackTopProps {
        visibility_height: 100,
        target: page.selector(),
        class: Some("to-top".into()),
        ..Default::default()
    });
    page.container
        .append_child(&control)
        .expect("append backtop");

    assert_eq!(control.class_name(), "backtop to-top");
    assert!(control.has_attribute("hidden"));
    assert_eq!(
        control.get_attribute("data-visible").as_deref(),
        Some("false")
    );

    page.scroll_to(50);
    assert!(
        control.has_attribute("hidden"),
        "50px is under the threshold"
    );

    page.scroll_to(150);
    assert!(!control.has_attribute("hidden"));
    assert_eq!(
        control.get_attribute("data-visible").as_deref(),
        Some("true")
    );

    page.scroll_to(0);
    assert!(control.has_attribute("hidden"));
}

#[wasm_bindgen_test]
fn backtop_returns_the_reader_to_the_top() {
    let page = Page::new("backtop-return");
    page.add_section("return-one", "one");
    page.add_section("return-two", "two");

    let control = backtop(BackTopProps {
        visibility_height: 100,
        target: page.selector(),
        ..Default::default()
    });
    page.container
        .append_child(&control)
        .expect("append backtop");
    page.scroll_to(300);
    assert!(page.container.scroll_top() > 0);

    let button = control.query_selector("button").unwrap().unwrap();
    assert_eq!(button.get_attribute("type").as_deref(), Some("button"));
    assert_eq!(
        button.get_attribute("aria-label").as_deref(),
        Some("Back to top")
    );
    test_utils::simulate_click(&button);

    assert_eq!(page.container.scroll_top(), 0);
}

#[wasm_bindgen_test]
async fn removing_a_scroll_component_detaches_its_listener() {
    domius_web::init();
    let page = Page::new("backtop-disposal");
    page.add_section("disposal-one", "one");
    page.add_section("disposal-two", "two");
    let control = backtop(BackTopProps {
        visibility_height: 100,
        target: page.selector(),
        ..Default::default()
    });
    page.container
        .append_child(&control)
        .expect("append backtop");
    page.scroll_to(150);
    assert_eq!(
        control.get_attribute("data-visible").as_deref(),
        Some("true")
    );

    control.remove();
    for _ in 0..2 {
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(
            &wasm_bindgen::JsValue::UNDEFINED,
        ))
        .await
        .expect("let the disposal observer run");
    }
    page.scroll_to(0);

    assert_eq!(
        control.get_attribute("data-visible").as_deref(),
        Some("true"),
        "a detached component must no longer receive global scroll events"
    );
}

#[wasm_bindgen_test]
fn affix_takes_hold_past_its_offset_and_reserves_the_space() {
    let page = Page::new("affix-page");
    page.add_section("affix-one", "one");
    page.add_section("affix-two", "two");

    let block = affix(AffixProps {
        offset_top: 120,
        offset_bottom: Some(24),
        target: page.selector(),
        class: Some("toc".into()),
    });
    page.container.append_child(&block).expect("append affix");

    assert_eq!(block.class_name(), "affix toc");
    assert_eq!(
        block.get_attribute("data-offset-top").as_deref(),
        Some("120")
    );
    assert_eq!(
        block.get_attribute("data-offset-bottom").as_deref(),
        Some("24")
    );
    assert_eq!(
        block.get_attribute("data-affixed").as_deref(),
        Some("false")
    );

    let placeholder = block.query_selector(".affix-placeholder").unwrap().unwrap();
    assert!(placeholder.has_attribute("hidden"));
    assert_eq!(
        placeholder.get_attribute("aria-hidden").as_deref(),
        Some("true")
    );

    page.scroll_to(200);
    assert_eq!(block.get_attribute("data-affixed").as_deref(), Some("true"));
    assert!(
        !placeholder.has_attribute("hidden"),
        "the placeholder should hold the space the block left"
    );

    page.scroll_to(10);
    assert_eq!(
        block.get_attribute("data-affixed").as_deref(),
        Some("false")
    );
    assert!(placeholder.has_attribute("hidden"));
}

/// Ids are looked up document-wide, so each test needs its own set.
fn links(prefix: &str) -> Vec<AnchorLink> {
    ["First", "Second", "Third"]
        .iter()
        .enumerate()
        .map(|(index, title)| AnchorLink {
            href: format!("#{prefix}-{index}"),
            title: (*title).to_string(),
        })
        .collect()
}

fn sections(page: &Page, prefix: &str) {
    for index in 0..3 {
        page.add_section(&format!("{prefix}-{index}"), "section");
    }
}

#[wasm_bindgen_test]
fn the_anchor_list_is_a_navigation_of_real_links() {
    let page = Page::new("anchor-page");
    sections(&page, "list");

    let toc = anchor(AnchorProps {
        links: links("list"),
        show_boundary: true,
        target_container: page.selector(),
        class: Some("service-toc".into()),
        ..Default::default()
    });
    page.container.append_child(&toc).expect("append anchor");

    assert_eq!(toc.tag_name(), "NAV");
    assert_eq!(
        toc.get_attribute("aria-label").as_deref(),
        Some("On this page")
    );
    assert_eq!(toc.class_name(), "anchor service-toc");
    assert_eq!(toc.query_selector_all(".anchor-link").unwrap().length(), 3);
    assert_eq!(
        toc.query_selector(".anchor-link")
            .unwrap()
            .unwrap()
            .get_attribute("href")
            .as_deref(),
        Some("#list-0")
    );
    // The rule is decoration and should not be read out.
    assert_eq!(
        toc.query_selector(".anchor-line")
            .unwrap()
            .unwrap()
            .get_attribute("aria-hidden")
            .as_deref(),
        Some("true")
    );
}

#[wasm_bindgen_test]
fn the_anchor_marks_the_section_being_read() {
    let page = Page::new("anchor-current");
    sections(&page, "current");

    let toc = anchor(AnchorProps {
        links: links("current"),
        target_container: page.selector(),
        ..Default::default()
    });
    page.container.append_child(&toc).expect("append anchor");

    let current = |toc: &web_sys::Element| {
        toc.query_selector("[data-active]")
            .unwrap()
            .and_then(|entry| entry.text_content())
    };

    assert_eq!(current(&toc).as_deref(), Some("First"));
    assert_eq!(
        toc.query_selector("[data-active]")
            .unwrap()
            .unwrap()
            .get_attribute("aria-current")
            .as_deref(),
        Some("location")
    );

    // Sections are 400px tall, so this lands the reader inside the second.
    page.scroll_to(450);
    assert_eq!(current(&toc).as_deref(), Some("Second"));
    assert_eq!(
        toc.query_selector_all("[data-active]").unwrap().length(),
        1,
        "only one entry can be the one being read"
    );

    page.scroll_to(900);
    assert_eq!(current(&toc).as_deref(), Some("Third"));

    page.scroll_to(0);
    assert_eq!(current(&toc).as_deref(), Some("First"));
}

#[wasm_bindgen_test]
fn activating_an_anchor_moves_focus_to_its_section() {
    let page = Page::new("anchor-activate");
    sections(&page, "activate");

    let toc = anchor(AnchorProps {
        links: links("activate"),
        target_container: page.selector(),
        ..Default::default()
    });
    page.container.append_child(&toc).expect("append anchor");

    let second = toc.query_selector("[href='#activate-1']").unwrap().unwrap();
    test_utils::simulate_click(&second);

    let section = test_utils::get_element_by_id("activate-1").unwrap();
    // A reader sent to a section should be able to carry on from there.
    assert_eq!(section.get_attribute("tabindex").as_deref(), Some("-1"));
    assert_eq!(
        test_utils::document().active_element().map(|e| e.id()),
        Some("activate-1".to_string())
    );
}

#[wasm_bindgen_test]
fn an_empty_anchor_renders_an_empty_list() {
    let toc = anchor(AnchorProps::default());

    assert_eq!(toc.query_selector_all(".anchor-link").unwrap().length(), 0);
    assert!(toc.query_selector(".anchor-list").unwrap().is_some());
}
