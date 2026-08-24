//! Accessible data aggregation and summarization table.

use std::collections::{BTreeSet, HashMap};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Document, Element};

#[derive(Clone)]
pub enum Aggregator {
    Sum,
    Count,
    Average,
    Min,
    Max,
    First,
    Last,
}

pub type PivotData = HashMap<String, String>;

#[derive(Clone)]
pub struct PivotTableProps {
    pub data: Vec<PivotData>,
    pub rows: Vec<String>,
    pub columns: Vec<String>,
    pub values: Vec<String>,
    pub aggregator: Aggregator,
    pub show_totals: bool,
    pub collapsible: bool,
    pub class: Option<String>,
}

impl Default for PivotTableProps {
    fn default() -> Self {
        Self {
            data: vec![],
            rows: vec![],
            columns: vec![],
            values: vec![],
            aggregator: Aggregator::Sum,
            show_totals: true,
            collapsible: true,
            class: None,
        }
    }
}

pub struct PivotTable;

impl PivotTable {
    pub fn create(props: PivotTableProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let container = document
            .create_element("div")
            .expect("create pivot container");
        let mut classes = vec!["table-container"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        container.set_class_name(&classes.join(" "));
        container
            .set_attribute("data-aggregator", aggregator_name(&props.aggregator))
            .expect("set aggregator");

        let table = document
            .create_element("table")
            .expect("create pivot table");
        table.set_class_name("table");
        table
            .set_attribute("aria-label", "Pivot table")
            .expect("label pivot table");
        let row_keys = dimension_keys(&props.data, &props.rows);
        let column_keys = dimension_keys(&props.data, &props.columns);
        let values = if props.values.is_empty() {
            vec![String::new()]
        } else {
            props.values.clone()
        };

        let head = document.create_element("thead").expect("create head");
        let header_row = document.create_element("tr").expect("create header row");
        if props.rows.is_empty() {
            append_header(&document, &header_row, "Group");
        }
        for field in &props.rows {
            append_header(&document, &header_row, field);
        }
        for column in &column_keys {
            for value in &values {
                let dimension = display_key(column, "Value");
                append_header(
                    &document,
                    &header_row,
                    &if value.is_empty() {
                        dimension
                    } else {
                        format!("{dimension} / {value}")
                    },
                );
            }
        }
        if props.show_totals {
            for value in &values {
                append_header(
                    &document,
                    &header_row,
                    &if value.is_empty() {
                        "Total".into()
                    } else {
                        format!("Total / {value}")
                    },
                );
            }
        }
        head.append_child(&header_row).expect("append header row");
        table.append_child(&head).expect("append head");

        let body = document.create_element("tbody").expect("create body");
        for (row_index, row_key) in row_keys.iter().enumerate() {
            let row = document.create_element("tr").expect("create row");
            row.set_attribute("data-row", &row_index.to_string())
                .expect("index row");
            append_row_labels(
                &document,
                &row,
                row_key,
                props.rows.is_empty(),
                props.collapsible,
                row_index,
            );
            for column_key in &column_keys {
                let records = matching_rows(
                    &props.data,
                    &props.rows,
                    row_key,
                    &props.columns,
                    column_key,
                );
                for value in &values {
                    append_value(
                        &document,
                        &row,
                        &aggregate(&records, value, &props.aggregator),
                    );
                }
            }
            if props.show_totals {
                let records = matching_rows(&props.data, &props.rows, row_key, &[], &[]);
                for value in &values {
                    append_value(
                        &document,
                        &row,
                        &aggregate(&records, value, &props.aggregator),
                    );
                }
            }
            body.append_child(&row).expect("append row");
        }
        table.append_child(&body).expect("append body");

        if props.show_totals {
            let foot = document.create_element("tfoot").expect("create foot");
            let row = document.create_element("tr").expect("create total row");
            let label = document.create_element("th").expect("create total label");
            label.set_attribute("scope", "row").expect("scope total");
            label
                .set_attribute("colspan", &props.rows.len().max(1).to_string())
                .expect("span total");
            label.set_text_content(Some("Total"));
            row.append_child(&label).expect("append total label");
            for column_key in &column_keys {
                let records = matching_rows(&props.data, &[], &[], &props.columns, column_key);
                for value in &values {
                    append_value(
                        &document,
                        &row,
                        &aggregate(&records, value, &props.aggregator),
                    );
                }
            }
            let records: Vec<&PivotData> = props.data.iter().collect();
            for value in &values {
                append_value(
                    &document,
                    &row,
                    &aggregate(&records, value, &props.aggregator),
                );
            }
            foot.append_child(&row).expect("append total row");
            table.append_child(&foot).expect("append foot");
        }
        container.append_child(&table).expect("append pivot table");
        container
    }
}

fn dimension_keys(data: &[PivotData], fields: &[String]) -> Vec<Vec<String>> {
    if fields.is_empty() {
        return vec![vec![]];
    }
    data.iter()
        .map(|item| {
            fields
                .iter()
                .map(|field| item.get(field).cloned().unwrap_or_default())
                .collect::<Vec<_>>()
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn matching_rows<'a>(
    data: &'a [PivotData],
    row_fields: &[String],
    row_key: &[String],
    column_fields: &[String],
    column_key: &[String],
) -> Vec<&'a PivotData> {
    data.iter()
        .filter(|item| matches_key(item, row_fields, row_key))
        .filter(|item| matches_key(item, column_fields, column_key))
        .collect()
}

fn matches_key(item: &PivotData, fields: &[String], key: &[String]) -> bool {
    fields.iter().zip(key).all(|(field, expected)| {
        item.get(field).map(String::as_str).unwrap_or_default() == expected
    })
}

fn aggregate(data: &[&PivotData], value: &str, aggregator: &Aggregator) -> String {
    let raw: Vec<&str> = if value.is_empty() {
        vec![""; data.len()]
    } else {
        data.iter()
            .filter_map(|item| item.get(value).map(String::as_str))
            .collect()
    };
    match aggregator {
        Aggregator::Count => return raw.len().to_string(),
        Aggregator::First => return raw.first().copied().unwrap_or_default().to_string(),
        Aggregator::Last => return raw.last().copied().unwrap_or_default().to_string(),
        _ => {}
    }
    let numbers: Vec<f64> = raw
        .iter()
        .filter_map(|value| value.parse().ok())
        .filter(|value: &f64| value.is_finite())
        .collect();
    if numbers.is_empty() {
        return String::new();
    }
    let result = match aggregator {
        Aggregator::Sum => numbers.iter().sum(),
        Aggregator::Average => numbers.iter().sum::<f64>() / numbers.len() as f64,
        Aggregator::Min => numbers
            .iter()
            .copied()
            .reduce(f64::min)
            .expect("numbers exist"),
        Aggregator::Max => numbers
            .iter()
            .copied()
            .reduce(f64::max)
            .expect("numbers exist"),
        _ => unreachable!(),
    };
    if result.fract().abs() < f64::EPSILON {
        format!("{result:.0}")
    } else {
        format!("{result:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

fn aggregator_name(value: &Aggregator) -> &'static str {
    match value {
        Aggregator::Sum => "sum",
        Aggregator::Count => "count",
        Aggregator::Average => "average",
        Aggregator::Min => "min",
        Aggregator::Max => "max",
        Aggregator::First => "first",
        Aggregator::Last => "last",
    }
}

fn display_key(key: &[String], fallback: &str) -> String {
    if key.is_empty() {
        fallback.into()
    } else {
        key.iter()
            .map(|part| if part.is_empty() { "—" } else { part })
            .collect::<Vec<_>>()
            .join(" / ")
    }
}

fn append_header(document: &Document, row: &Element, label: &str) {
    let header = document.create_element("th").expect("create header");
    header.set_attribute("scope", "col").expect("scope header");
    header.set_text_content(Some(label));
    row.append_child(&header).expect("append header");
}

fn append_row_labels(
    document: &Document,
    row: &Element,
    key: &[String],
    empty: bool,
    collapsible: bool,
    row_index: usize,
) {
    let labels = if empty {
        vec!["All".into()]
    } else {
        key.to_vec()
    };
    for (index, value) in labels.into_iter().enumerate() {
        let header = document.create_element("th").expect("create row header");
        header
            .set_attribute("scope", "row")
            .expect("scope row header");
        header
            .set_attribute("data-dimension-index", &index.to_string())
            .expect("index dimension");
        header
            .set_attribute("data-row", &row_index.to_string())
            .expect("index header row");
        if collapsible && index == 0 {
            let button = document
                .create_element("button")
                .expect("create collapse button");
            button.set_attribute("type", "button").expect("type button");
            button
                .set_attribute("aria-expanded", "true")
                .expect("expand row");
            button
                .set_attribute("aria-label", &format!("Collapse {value}"))
                .expect("label button");
            button.set_text_content(Some(&value));
            let controlled_row = row.clone();
            let controlled_button = button.clone();
            let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                let collapse =
                    controlled_button.get_attribute("aria-expanded").as_deref() != Some("false");
                controlled_button
                    .set_attribute("aria-expanded", if collapse { "false" } else { "true" })
                    .expect("toggle button");
                controlled_row
                    .set_attribute("data-collapsed", &collapse.to_string())
                    .expect("toggle row");
                let cells = controlled_row
                    .query_selector_all("td")
                    .expect("query cells");
                for cell_index in 0..cells.length() {
                    if let Some(cell) = cells
                        .item(cell_index)
                        .and_then(|node| node.dyn_into::<Element>().ok())
                    {
                        cell.set_attribute("data-collapsed", &collapse.to_string())
                            .expect("toggle cell");
                    }
                }
            });
            button
                .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                .expect("listen collapse");
            handler.forget();
            header
                .append_child(&button)
                .expect("append collapse button");
        } else {
            header.set_text_content(Some(if value.is_empty() { "—" } else { &value }));
        }
        row.append_child(&header).expect("append row header");
    }
}

fn append_value(document: &Document, row: &Element, value: &str) {
    let cell = document.create_element("td").expect("create value cell");
    cell.set_attribute("data-value", value).expect("set value");
    cell.set_text_content(Some(if value.is_empty() { "—" } else { value }));
    row.append_child(&cell).expect("append value cell");
}
