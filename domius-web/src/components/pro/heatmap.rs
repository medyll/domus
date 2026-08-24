//! Accessible tabular heatmap.

use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Document, Element};

#[derive(Clone)]
pub struct HeatmapCell {
    pub x: usize,
    pub y: usize,
    pub value: f64,
}

pub struct HeatmapProps {
    pub data: Vec<HeatmapCell>,
    pub x_labels: Vec<String>,
    pub y_labels: Vec<String>,
    pub color_scale: HeatmapColorScale,
    pub show_values: bool,
    pub on_cell_click: Option<Box<dyn Fn(usize, usize)>>,
    pub class: Option<String>,
}

#[derive(Clone)]
pub enum HeatmapColorScale {
    Sequential(Vec<String>),
    Diverging(Vec<String>),
    Categorical(Vec<String>),
}

impl Default for HeatmapProps {
    fn default() -> Self {
        Self {
            data: vec![],
            x_labels: vec![],
            y_labels: vec![],
            color_scale: HeatmapColorScale::Sequential(vec!["#ffffff".into(), "#0000ff".into()]),
            show_values: false,
            on_cell_click: None,
            class: None,
        }
    }
}

pub struct Heatmap;

impl Heatmap {
    pub fn create(props: HeatmapProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let container = document
            .create_element("div")
            .expect("create heatmap container");
        let mut classes = vec!["table-container", "heatmap"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        container.set_class_name(&classes.join(" "));
        let table = document
            .create_element("table")
            .expect("create heatmap table");
        table
            .set_attribute("aria-label", "Heatmap")
            .expect("label heatmap");
        table
            .set_attribute("data-color-scale", scale_name(&props.color_scale))
            .expect("set scale type");
        append_header(&document, &table, &props.x_labels);

        let (minimum, maximum) = value_range(&props.data);
        let palette = palette(&props.color_scale);
        let callback = props.on_cell_click.map(Rc::<dyn Fn(usize, usize)>::from);
        let body = document
            .create_element("tbody")
            .expect("create heatmap body");
        for y in 0..props.y_labels.len() {
            let row = document.create_element("tr").expect("create heatmap row");
            let label = document.create_element("th").expect("create row label");
            label
                .set_attribute("scope", "row")
                .expect("scope row label");
            label.set_text_content(Some(&props.y_labels[y]));
            row.append_child(&label).expect("append row label");
            for x in 0..props.x_labels.len() {
                let cell = document.create_element("td").expect("create heatmap cell");
                cell.set_attribute("data-x", &x.to_string()).expect("set x");
                cell.set_attribute("data-y", &y.to_string()).expect("set y");
                if let Some(point) = props.data.iter().find(|point| point.x == x && point.y == y) {
                    let color =
                        color_for(point.value, minimum, maximum, &palette, &props.color_scale);
                    cell.set_attribute("data-value", &format_number(point.value))
                        .expect("set value");
                    cell.set_attribute("data-color", color)
                        .expect("set color token");
                    append_content(
                        &document,
                        &cell,
                        point,
                        &props.x_labels[x],
                        &props.y_labels[y],
                        props.show_values,
                        callback.clone(),
                    );
                } else {
                    cell.set_attribute("data-empty", "true")
                        .expect("mark empty cell");
                    cell.set_text_content(Some("—"));
                }
                row.append_child(&cell).expect("append heatmap cell");
            }
            body.append_child(&row).expect("append heatmap row");
        }
        table.append_child(&body).expect("append heatmap body");
        container
            .append_child(&table)
            .expect("append heatmap table");
        container
    }
}

fn append_header(document: &Document, table: &Element, labels: &[String]) {
    let head = document
        .create_element("thead")
        .expect("create heatmap head");
    let row = document
        .create_element("tr")
        .expect("create heatmap header row");
    let corner = document
        .create_element("th")
        .expect("create heatmap corner");
    corner
        .set_attribute("aria-hidden", "true")
        .expect("hide corner");
    row.append_child(&corner).expect("append heatmap corner");
    for label in labels {
        let header = document.create_element("th").expect("create column label");
        header
            .set_attribute("scope", "col")
            .expect("scope column label");
        header.set_text_content(Some(label));
        row.append_child(&header).expect("append column label");
    }
    head.append_child(&row).expect("append header row");
    table.append_child(&head).expect("append heatmap head");
}

fn append_content(
    document: &Document,
    cell: &Element,
    point: &HeatmapCell,
    x_label: &str,
    y_label: &str,
    show_value: bool,
    callback: Option<Rc<dyn Fn(usize, usize)>>,
) {
    let value = format_number(point.value);
    if let Some(callback) = callback {
        let button = document
            .create_element("button")
            .expect("create heatmap button");
        button
            .set_attribute("type", "button")
            .expect("type heatmap button");
        button
            .set_attribute("aria-label", &format!("{y_label}, {x_label}: {value}"))
            .expect("label heatmap button");
        button.set_text_content(Some(if show_value { &value } else { "Select" }));
        let x = point.x;
        let y = point.y;
        let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| callback(x, y));
        button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .expect("listen heatmap click");
        handler.forget();
        cell.append_child(&button).expect("append heatmap button");
    } else if show_value {
        cell.set_text_content(Some(&value));
    } else {
        cell.set_attribute("aria-label", &format!("{y_label}, {x_label}: {value}"))
            .expect("label heatmap cell");
    }
}

fn value_range(data: &[HeatmapCell]) -> (f64, f64) {
    let values: Vec<f64> = data
        .iter()
        .map(|cell| cell.value)
        .filter(|value| value.is_finite())
        .collect();
    (
        values.iter().copied().reduce(f64::min).unwrap_or_default(),
        values.iter().copied().reduce(f64::max).unwrap_or_default(),
    )
}

fn palette(scale: &HeatmapColorScale) -> Vec<&str> {
    let colors = match scale {
        HeatmapColorScale::Sequential(colors)
        | HeatmapColorScale::Diverging(colors)
        | HeatmapColorScale::Categorical(colors) => colors,
    };
    if colors.is_empty() {
        vec!["neutral"]
    } else {
        colors.iter().map(String::as_str).collect()
    }
}

fn color_for<'a>(
    value: f64,
    minimum: f64,
    maximum: f64,
    colors: &'a [&str],
    scale: &HeatmapColorScale,
) -> &'a str {
    if matches!(scale, HeatmapColorScale::Categorical(_)) {
        return colors[(value.abs().round() as usize) % colors.len()];
    }
    let ratio = if (maximum - minimum).abs() < f64::EPSILON {
        0.0
    } else {
        ((value - minimum) / (maximum - minimum)).clamp(0.0, 1.0)
    };
    colors[((ratio * colors.len() as f64).floor() as usize).min(colors.len() - 1)]
}

fn scale_name(scale: &HeatmapColorScale) -> &'static str {
    match scale {
        HeatmapColorScale::Sequential(_) => "sequential",
        HeatmapColorScale::Diverging(_) => "diverging",
        HeatmapColorScale::Categorical(_) => "categorical",
    }
}

fn format_number(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}
