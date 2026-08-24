//! Accessible SVG data visualizations.

use std::f64::consts::TAU;

use web_sys::{Document, Element};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Donut,
    Area,
    Scatter,
    Radar,
    Polar,
}

impl ChartType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Bar => "bar",
            Self::Pie => "pie",
            Self::Donut => "donut",
            Self::Area => "area",
            Self::Scatter => "scatter",
            Self::Radar => "radar",
            Self::Polar => "polar",
        }
    }
}

#[derive(Clone)]
pub struct ChartDataPoint {
    pub label: String,
    pub value: f64,
}

#[derive(Clone)]
pub struct ChartsProps {
    pub chart_type: ChartType,
    pub data: Vec<ChartDataPoint>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub show_legend: bool,
    pub show_tooltip: bool,
    pub animated: bool,
    pub colors: Vec<String>,
    pub class: Option<String>,
}

impl Default for ChartsProps {
    fn default() -> Self {
        Self {
            chart_type: ChartType::Bar,
            data: Vec::new(),
            width: Some(400),
            height: Some(300),
            show_legend: true,
            show_tooltip: true,
            animated: true,
            colors: vec!["primary".to_string()],
            class: None,
        }
    }
}

pub struct Charts;

impl Charts {
    pub fn create(props: ChartsProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let figure = document.create_element("figure").expect("create chart");
        let mut classes = vec!["domius-chart"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        figure.set_class_name(&classes.join(" "));
        figure
            .set_attribute("data-chart-type", props.chart_type.as_str())
            .expect("set chart type");
        figure
            .set_attribute("data-animated", &props.animated.to_string())
            .expect("set animation mode");

        let width = props.width.unwrap_or(400).max(1);
        let height = props.height.unwrap_or(300).max(1);
        let svg = svg_element(&document, "svg");
        svg.set_attribute("viewBox", &format!("0 0 {width} {height}"))
            .expect("set chart view box");
        svg.set_attribute("role", "img").expect("set chart role");
        svg.set_attribute(
            "aria-label",
            &format!(
                "{} chart with {} data points",
                props.chart_type.as_str(),
                props.data.len()
            ),
        )
        .expect("label chart");

        if props.data.is_empty() {
            let empty = svg_element(&document, "text");
            empty
                .set_attribute("x", &(width / 2).to_string())
                .expect("position empty chart");
            empty
                .set_attribute("y", &(height / 2).to_string())
                .expect("position empty chart");
            empty.set_text_content(Some("No data"));
            svg.append_child(&empty).expect("append empty state");
        } else {
            render_series(&document, &svg, &props, width, height);
        }
        figure.append_child(&svg).expect("append chart graphic");

        if props.show_legend {
            append_legend(&document, &figure, &props);
        }
        figure
    }
}

fn render_series(document: &Document, svg: &Element, props: &ChartsProps, width: u32, height: u32) {
    match props.chart_type {
        ChartType::Bar => render_bars(document, svg, props, width, height),
        ChartType::Line | ChartType::Area | ChartType::Scatter => {
            render_cartesian(document, svg, props, width, height)
        }
        ChartType::Pie | ChartType::Donut => render_circular(document, svg, props, width, height),
        ChartType::Radar | ChartType::Polar => render_radial(document, svg, props, width, height),
    }
}

fn render_bars(document: &Document, svg: &Element, props: &ChartsProps, width: u32, height: u32) {
    let max = maximum(&props.data);
    let slot = f64::from(width) / props.data.len() as f64;
    for (index, point) in props.data.iter().enumerate() {
        let bar_height = normalized(point.value, max) * f64::from(height);
        let bar = svg_element(document, "rect");
        bar.set_attribute("x", &(index as f64 * slot + slot * 0.1).to_string())
            .expect("position chart bar");
        bar.set_attribute("y", &(f64::from(height) - bar_height).to_string())
            .expect("position chart bar");
        bar.set_attribute("width", &(slot * 0.8).to_string())
            .expect("size chart bar");
        bar.set_attribute("height", &bar_height.to_string())
            .expect("size chart bar");
        decorate_mark(document, &bar, props, point, index);
        svg.append_child(&bar).expect("append chart bar");
    }
}

fn render_cartesian(
    document: &Document,
    svg: &Element,
    props: &ChartsProps,
    width: u32,
    height: u32,
) {
    let max = maximum(&props.data);
    let denominator = props.data.len().saturating_sub(1).max(1) as f64;
    let points = props
        .data
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let x = index as f64 / denominator * f64::from(width);
            let y = f64::from(height) - normalized(point.value, max) * f64::from(height);
            (x, y)
        })
        .collect::<Vec<_>>();

    if props.chart_type != ChartType::Scatter {
        let shape = svg_element(
            document,
            if props.chart_type == ChartType::Area {
                "polygon"
            } else {
                "polyline"
            },
        );
        let mut coordinates = points
            .iter()
            .map(|(x, y)| format!("{x},{y}"))
            .collect::<Vec<_>>();
        if props.chart_type == ChartType::Area {
            coordinates.insert(0, format!("0,{height}"));
            coordinates.push(format!("{width},{height}"));
        }
        shape
            .set_attribute("points", &coordinates.join(" "))
            .expect("set chart points");
        shape.set_attribute("data-series", "0").expect("set series");
        apply_color(&shape, props, 0);
        svg.append_child(&shape).expect("append chart shape");
    }

    for (index, (point, (x, y))) in props.data.iter().zip(points).enumerate() {
        let marker = svg_element(document, "circle");
        marker
            .set_attribute("cx", &x.to_string())
            .expect("position marker");
        marker
            .set_attribute("cy", &y.to_string())
            .expect("position marker");
        marker.set_attribute("r", "4").expect("size marker");
        decorate_mark(document, &marker, props, point, index);
        svg.append_child(&marker).expect("append chart marker");
    }
}

fn render_circular(
    document: &Document,
    svg: &Element,
    props: &ChartsProps,
    width: u32,
    height: u32,
) {
    let radius = f64::from(width.min(height)) * 0.35;
    let circumference = TAU * radius;
    let total: f64 = props.data.iter().map(|point| point.value.max(0.0)).sum();
    let mut offset = 0.0;
    for (index, point) in props.data.iter().enumerate() {
        let fraction = if total > 0.0 {
            point.value.max(0.0) / total
        } else {
            0.0
        };
        let segment = svg_element(document, "circle");
        segment
            .set_attribute("cx", &(width / 2).to_string())
            .expect("center segment");
        segment
            .set_attribute("cy", &(height / 2).to_string())
            .expect("center segment");
        segment
            .set_attribute("r", &radius.to_string())
            .expect("size segment");
        segment
            .set_attribute("fill", "none")
            .expect("clear segment fill");
        let thickness = if props.chart_type == ChartType::Donut {
            radius * 0.35
        } else {
            radius
        };
        segment
            .set_attribute("stroke-width", &thickness.to_string())
            .expect("size segment stroke");
        segment
            .set_attribute(
                "stroke-dasharray",
                &format!("{} {}", fraction * circumference, circumference),
            )
            .expect("size segment arc");
        segment
            .set_attribute("stroke-dashoffset", &(-offset * circumference).to_string())
            .expect("position segment arc");
        segment
            .set_attribute(
                "transform",
                &format!("rotate(-90 {} {})", width / 2, height / 2),
            )
            .expect("rotate segment");
        decorate_mark(document, &segment, props, point, index);
        svg.append_child(&segment).expect("append chart segment");
        offset += fraction;
    }
}

fn render_radial(document: &Document, svg: &Element, props: &ChartsProps, width: u32, height: u32) {
    let radius = f64::from(width.min(height)) * 0.4;
    let max = maximum(&props.data);
    let center_x = f64::from(width) / 2.0;
    let center_y = f64::from(height) / 2.0;
    let points = props
        .data
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let angle = index as f64 / props.data.len() as f64 * TAU - TAU / 4.0;
            let scale = if props.chart_type == ChartType::Polar {
                1.0
            } else {
                normalized(point.value, max)
            };
            format!(
                "{},{}",
                center_x + angle.cos() * radius * scale,
                center_y + angle.sin() * radius * scale
            )
        })
        .collect::<Vec<_>>();
    let polygon = svg_element(document, "polygon");
    polygon
        .set_attribute("points", &points.join(" "))
        .expect("set radial points");
    polygon
        .set_attribute("data-series", "0")
        .expect("set radial series");
    apply_color(&polygon, props, 0);
    svg.append_child(&polygon).expect("append radial chart");
}

fn append_legend(document: &Document, figure: &Element, props: &ChartsProps) {
    let legend = document.create_element("ul").expect("create chart legend");
    legend.set_class_name("domius-chart-legend");
    legend
        .set_attribute("aria-label", "Chart legend")
        .expect("label legend");
    for (index, point) in props.data.iter().enumerate() {
        let item = document.create_element("li").expect("create legend item");
        item.set_attribute("data-series", &index.to_string())
            .expect("set legend series");
        apply_color(&item, props, index);
        item.set_text_content(Some(&format!("{}: {}", point.label, point.value)));
        legend.append_child(&item).expect("append legend item");
    }
    figure.append_child(&legend).expect("append chart legend");
}

fn decorate_mark(
    document: &Document,
    mark: &Element,
    props: &ChartsProps,
    point: &ChartDataPoint,
    index: usize,
) {
    mark.set_attribute("data-series", &index.to_string())
        .expect("set mark series");
    mark.set_attribute("aria-label", &format!("{}: {}", point.label, point.value))
        .expect("label chart mark");
    apply_color(mark, props, index);
    if props.show_tooltip {
        let title = svg_element(document, "title");
        title.set_text_content(Some(&format!("{}: {}", point.label, point.value)));
        mark.append_child(&title).expect("append chart tooltip");
    }
}

fn apply_color(element: &Element, props: &ChartsProps, index: usize) {
    if let Some(color) = props.colors.get(index % props.colors.len().max(1)) {
        element
            .set_attribute("data-color", color)
            .expect("set series color");
    }
}

fn maximum(data: &[ChartDataPoint]) -> f64 {
    data.iter()
        .map(|point| point.value.max(0.0))
        .fold(0.0, f64::max)
        .max(1.0)
}

fn normalized(value: f64, maximum: f64) -> f64 {
    value.max(0.0) / maximum
}

fn svg_element(document: &Document, tag: &str) -> Element {
    document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
        .expect("create SVG element")
}
