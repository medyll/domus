#![cfg(target_arch = "wasm32")]

mod test_utils;

use domius_web::list::{DiffOp, KeyedList};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone)]
struct Row {
    id: &'static str,
    label: &'static str,
}

fn rows(ids: &[&'static str]) -> Vec<Row> {
    ids.iter()
        .map(|id| Row {
            id,
            label: "original",
        })
        .collect()
}

fn render(row: &Row) -> web_sys::Element {
    let element = test_utils::document()
        .create_element("li")
        .expect("create row");
    element.set_attribute("data-key", row.id).expect("key row");
    element.set_text_content(Some(row.label));
    element
}

fn rendered_keys(host: &web_sys::Element) -> Vec<String> {
    let children = host.children();
    (0..children.length())
        .map(|index| {
            children
                .item(index)
                .unwrap()
                .get_attribute("data-key")
                .expect("every child should carry its key")
        })
        .collect()
}

fn host() -> web_sys::Element {
    test_utils::create_div().unchecked_into()
}

#[wasm_bindgen_test]
fn builds_the_initial_list() {
    let host = host();
    let mut list = KeyedList::mount(host.clone());
    let patch = list.reconcile(&rows(&["a", "b", "c"]), |row| row.id.to_string(), render);

    assert_eq!(patch.ops, vec![DiffOp::Insert; 3]);
    assert!(patch.removes.is_empty());
    assert_eq!(list.len(), 3);
    assert_eq!(rendered_keys(&host), ["a", "b", "c"]);
}

#[wasm_bindgen_test]
fn reordering_moves_the_very_same_nodes() {
    let host = host();
    let mut list = KeyedList::mount(host.clone());
    list.reconcile(&rows(&["a", "b", "c"]), |row| row.id.to_string(), render);

    // Mark the standing nodes so a rebuild would be visible.
    for key in ["a", "b", "c"] {
        list.node(key)
            .expect("node should exist")
            .set_attribute("data-touched", key)
            .expect("mark node");
    }
    let before = list.node("b").expect("node should exist").clone();

    let patch = list.reconcile(&rows(&["c", "a", "b"]), |row| row.id.to_string(), render);

    assert!(patch.removes.is_empty());
    assert_eq!(
        patch.ops,
        vec![DiffOp::Keep(2), DiffOp::Keep(0), DiffOp::Keep(1)]
    );
    assert_eq!(rendered_keys(&host), ["c", "a", "b"]);
    assert_eq!(host.children().length(), 3);

    // Same node object, still carrying the mark it was given before the move.
    assert!(before.is_same_node(Some(list.node("b").unwrap())));
    for key in ["a", "b", "c"] {
        assert_eq!(
            list.node(key)
                .unwrap()
                .get_attribute("data-touched")
                .as_deref(),
            Some(key)
        );
    }
}

#[wasm_bindgen_test]
fn removing_an_item_leaves_the_survivors_untouched() {
    let host = host();
    let mut list = KeyedList::mount(host.clone());
    list.reconcile(
        &rows(&["a", "b", "c", "d"]),
        |row| row.id.to_string(),
        render,
    );
    for key in ["a", "b", "c", "d"] {
        list.node(key)
            .unwrap()
            .set_attribute("data-touched", key)
            .expect("mark node");
    }
    let survivor = list.node("d").unwrap().clone();

    let patch = list.reconcile(&rows(&["d", "b"]), |row| row.id.to_string(), render);

    assert_eq!(patch.removes, vec![0, 2]);
    assert_eq!(patch.ops, vec![DiffOp::Keep(3), DiffOp::Keep(1)]);
    assert_eq!(rendered_keys(&host), ["d", "b"]);
    assert_eq!(host.children().length(), 2);
    assert!(survivor.is_same_node(Some(list.node("d").unwrap())));
    assert_eq!(
        list.node("b")
            .unwrap()
            .get_attribute("data-touched")
            .as_deref(),
        Some("b")
    );
    assert!(list.node("a").is_none());
}

#[wasm_bindgen_test]
fn reordering_then_removing_keeps_the_nodes_that_stayed() {
    let host = host();
    let mut list = KeyedList::mount(host.clone());
    list.reconcile(
        &rows(&["a", "b", "c", "d"]),
        |row| row.id.to_string(),
        render,
    );
    let kept = ["b", "d"].map(|key| list.node(key).unwrap().clone());

    list.reconcile(
        &rows(&["d", "c", "b", "a"]),
        |row| row.id.to_string(),
        render,
    );
    list.reconcile(&rows(&["d", "b"]), |row| row.id.to_string(), render);

    assert_eq!(rendered_keys(&host), ["d", "b"]);
    assert!(kept[1].is_same_node(Some(list.node("d").unwrap())));
    assert!(kept[0].is_same_node(Some(list.node("b").unwrap())));
}

#[wasm_bindgen_test]
fn kept_nodes_are_refreshed_rather_than_rebuilt() {
    let host = host();
    let mut list = KeyedList::mount(host.clone());
    list.reconcile(&rows(&["a", "b"]), |row| row.id.to_string(), render);
    let before = list.node("a").unwrap().clone();

    let updated = vec![
        Row {
            id: "a",
            label: "acknowledged",
        },
        Row {
            id: "b",
            label: "original",
        },
    ];
    list.reconcile_with(
        &updated,
        |row| row.id.to_string(),
        render,
        |node, row| node.set_text_content(Some(row.label)),
    );

    assert!(before.is_same_node(Some(list.node("a").unwrap())));
    assert_eq!(
        list.node("a").unwrap().text_content(),
        Some("acknowledged".to_string())
    );
}

#[wasm_bindgen_test]
fn a_repeated_key_gets_a_node_of_its_own() {
    let host = host();
    let mut list = KeyedList::mount(host.clone());
    list.reconcile(&rows(&["a"]), |row| row.id.to_string(), render);
    list.reconcile(&rows(&["a", "a"]), |row| row.id.to_string(), render);

    // Two positions, two live nodes — not one node appended twice.
    assert_eq!(host.children().length(), 2);
    assert_eq!(rendered_keys(&host), ["a", "a"]);
}

#[wasm_bindgen_test]
fn shrinking_repeated_keys_removes_the_unclaimed_node() {
    let host = host();
    let mut list = KeyedList::mount(host.clone());
    list.reconcile(&rows(&["a", "a"]), |row| row.id.to_string(), render);
    let survivor = host.children().item(0).unwrap();

    let patch = list.reconcile(&rows(&["a"]), |row| row.id.to_string(), render);

    assert_eq!(patch.removes, vec![1]);
    assert_eq!(patch.ops, vec![DiffOp::Keep(0)]);
    assert_eq!(host.children().length(), 1);
    assert!(survivor.is_same_node(host.first_child().as_ref()));
}

#[wasm_bindgen_test]
fn clearing_the_list_empties_the_host() {
    let host = host();
    let mut list = KeyedList::mount(host.clone());
    list.reconcile(&rows(&["a", "b"]), |row| row.id.to_string(), render);

    let patch = list.reconcile(&[], |row: &Row| row.id.to_string(), render);

    assert_eq!(patch.removes, vec![0, 1]);
    assert!(list.is_empty());
    assert_eq!(host.children().length(), 0);
}

#[wasm_bindgen_test]
fn mounting_adopts_a_host_that_already_had_children() {
    let host = host();
    let stale = test_utils::document().create_element("li").unwrap();
    host.append_child(&stale).unwrap();

    let mut list = KeyedList::mount(host.clone());
    assert_eq!(host.children().length(), 0);

    list.reconcile(&rows(&["a"]), |row| row.id.to_string(), render);
    assert_eq!(rendered_keys(&host), ["a"]);
}
