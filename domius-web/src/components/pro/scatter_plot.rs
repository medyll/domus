//! Accessible SVG scatter plot for correlation analysis.

use web_sys::{Document, Element};

/// Data point for scatter plot.
#[derive(Clone)]
pub struct ScatterPoint {
    pub x: f64,
    pub y: f64,
    pub label: Option<String>,
    pub color: Option<String>,
    pub size: Option<f64>,
}

/// Props for the ScatterPlot component.
#[derive(Clone)]
pub struct ScatterPlotProps {
    pub points: Vec<ScatterPoint>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub x_min: Option<f64>,
    pub x_max: Option<f64>,
    pub y_min: Option<f64>,
    pub y_max: Option<f64>,
    pub show_grid: bool,
    pub show_labels: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub class: Option<String>,
}

impl Default for ScatterPlotProps {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            x_label: None,
            y_label: None,
            x_min: None,
            x_max: None,
            y_min: None,
            y_max: None,
            show_grid: true,
            show_labels: false,
            width: Some(400),
            height: Some(300),
            class: None,
        }
    }
}

/// Inclusive numeric interval mapped onto one plot axis.
#[derive(Clone, Copy)]
struct Domain {
    minimum: f64,
    maximum: f64,
}

impl Domain {
    /// Resolve an axis domain from explicit bounds, falling back to the data.
    fn resolve(values: &[f64], minimum: Option<f64>, maximum: Option<f64>) -> Self {
        let finite = values
            .iter()
            .copied()
            .filter(|value| value.is_finite())
            .collect::<Vec<_>>();
        let low = minimum.unwrap_or_else(|| finite.iter().copied().reduce(f64::min).unwrap_or(0.0));
        let high =
            maximum.unwrap_or_else(|| finite.iter().copied().reduce(f64::max).unwrap_or(0.0));
        let (low, high) = if low <= high {
            (low, high)
        } else {
            (high, low)
        };
        Self {
            minimum: low,
            maximum: high,
        }
    }

    /// Position a value inside the domain, centring it when the domain is constant.
    fn ratio(self, value: f64) -> f64 {
        if self.is_constant() {
            0.5
        } else {
            (value - self.minimum) / (self.maximum - self.minimum)
        }
    }

    fn contains(self, value: f64) -> bool {
        value >= self.minimum && value <= self.maximum
    }

    fn is_constant(self) -> bool {
        (self.maximum - self.minimum).abs() < f64::EPSILON
    }
}

/// Plot area reserved for marks, leaving room for axis labels.
struct Frame {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl Frame {
    fn new(width: f64, height: f64) -> Self {
        let left = (width * 0.12).clamp(1.0, 44.0);
        Self {
            left,
            right: (width - 12.0).max(left + 1.0),
            top: (height * 0.04).clamp(1.0, 12.0),
            bottom: height - (height * 0.12).clamp(1.0, 28.0),
        }
    }

    fn width(&self) -> f64 {
        (self.right - self.left).max(1.0)
    }

    fn height(&self) -> f64 {
        (self.bottom - self.top).max(1.0)
    }

    fn x(&self, ratio: f64) -> f64 {
        self.left + ratio.clamp(0.0, 1.0) * self.width()
    }

    fn y(&self, ratio: f64) -> f64 {
        self.bottom - ratio.clamp(0.0, 1.0) * self.height()
    }
}

/// ScatterPlot component.
pub struct ScatterPlot;

impl ScatterPlot {
    /// Create a scatter plot element.
    pub fn create(props: ScatterPlotProps) -> Element {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let figure = document
            .create_element("figure")
            .expect("create scatter plot");
        let mut classes = vec!["domius-scatter-plot"];
        if let Some(class) = props.class.as_deref() {
            classes.push(class);
        }
        figure.set_class_name(&classes.join(" "));
        figure
            .set_attribute("data-points", &props.points.len().to_string())
            .expect("set point count");

        let width = f64::from(props.width.unwrap_or(400).max(1));
        let height = f64::from(props.height.unwrap_or(300).max(1));
        let x_domain = Domain::resolve(
            &props.points.iter().map(|point| point.x).collect::<Vec<_>>(),
            props.x_min,
            props.x_max,
        );
        let y_domain = Domain::resolve(
            &props.points.iter().map(|point| point.y).collect::<Vec<_>>(),
            props.y_min,
            props.y_max,
        );
        let frame = Frame::new(width, height);

        let svg = svg_element(&document, "svg");
        svg.set_attribute("viewBox", &format!("0 0 {width} {height}"))
            .expect("set scatter view box");
        svg.set_attribute("role", "img")
            .expect("set scatter plot role");
        svg.set_attribute("aria-label", &summary(&props, x_domain, y_domain))
            .expect("label scatter plot");
        svg.set_attribute("data-x-domain", &domain_text(x_domain))
            .expect("set x domain");
        svg.set_attribute("data-y-domain", &domain_text(y_domain))
            .expect("set y domain");
        if x_domain.is_constant() || y_domain.is_constant() {
            svg.set_attribute("data-constant-domain", "true")
                .expect("flag constant domain");
        }

        if props.show_grid {
            append_grid(&document, &svg, &frame);
        }
        append_axes(&document, &svg, &frame, &props, x_domain, y_domain);

        if props.points.is_empty() {
            figure
                .set_attribute("data-empty", "true")
                .expect("flag empty scatter plot");
            append_empty_state(&document, &svg, width, height);
        } else {
            append_marks(&document, &svg, &frame, &props, x_domain, y_domain);
        }

        figure.append_child(&svg).expect("append scatter graphic");
        figure
    }
}

fn append_empty_state(document: &Document, svg: &Element, width: f64, height: f64) {
    let empty = svg_element(document, "text");
    empty
        .set_attribute("x", &(width / 2.0).to_string())
        .expect("position empty state");
    empty
        .set_attribute("y", &(height / 2.0).to_string())
        .expect("position empty state");
    empty
        .set_attribute("data-role", "empty")
        .expect("mark empty state");
    empty
        .set_attribute("text-anchor", "middle")
        .expect("centre empty state");
    empty.set_text_content(Some("No data"));
    svg.append_child(&empty).expect("append empty state");
}

fn append_grid(document: &Document, svg: &Element, frame: &Frame) {
    let grid = svg_element(document, "g");
    grid.set_attribute("data-role", "grid")
        .expect("mark grid group");
    grid.set_attribute("aria-hidden", "true")
        .expect("hide grid from readers");
    for step in 0..=4 {
        let ratio = f64::from(step) / 4.0;
        let vertical = svg_element(document, "line");
        let x = frame.x(ratio);
        set_line(&vertical, x, frame.top, x, frame.bottom);
        vertical
            .set_attribute("data-axis", "x")
            .expect("mark vertical grid line");
        grid.append_child(&vertical).expect("append grid line");

        let horizontal = svg_element(document, "line");
        let y = frame.y(ratio);
        set_line(&horizontal, frame.left, y, frame.right, y);
        horizontal
            .set_attribute("data-axis", "y")
            .expect("mark horizontal grid line");
        grid.append_child(&horizontal).expect("append grid line");
    }
    svg.append_child(&grid).expect("append grid group");
}

fn append_axes(
    document: &Document,
    svg: &Element,
    frame: &Frame,
    props: &ScatterPlotProps,
    x_domain: Domain,
    y_domain: Domain,
) {
    let axes = svg_element(document, "g");
    axes.set_attribute("data-role", "axes")
        .expect("mark axes group");

    let x_axis = svg_element(document, "line");
    set_line(&x_axis, frame.left, frame.bottom, frame.right, frame.bottom);
    x_axis
        .set_attribute("data-axis", "x")
        .expect("mark x axis line");
    axes.append_child(&x_axis).expect("append x axis");

    let y_axis = svg_element(document, "line");
    set_line(&y_axis, frame.left, frame.top, frame.left, frame.bottom);
    y_axis
        .set_attribute("data-axis", "y")
        .expect("mark y axis line");
    axes.append_child(&y_axis).expect("append y axis");

    append_tick(
        document,
        &axes,
        "x",
        frame.left,
        frame.bottom,
        x_domain.minimum,
    );
    append_tick(
        document,
        &axes,
        "x",
        frame.right,
        frame.bottom,
        x_domain.maximum,
    );
    append_tick(
        document,
        &axes,
        "y",
        frame.left,
        frame.bottom,
        y_domain.minimum,
    );
    append_tick(
        document,
        &axes,
        "y",
        frame.left,
        frame.top,
        y_domain.maximum,
    );

    if let Some(label) = props.x_label.as_deref() {
        append_axis_label(
            document,
            &axes,
            "x",
            label,
            (frame.left + frame.right) / 2.0,
            frame.bottom,
        );
    }
    if let Some(label) = props.y_label.as_deref() {
        append_axis_label(
            document,
            &axes,
            "y",
            label,
            frame.left,
            (frame.top + frame.bottom) / 2.0,
        );
    }
    svg.append_child(&axes).expect("append axes group");
}

fn append_tick(document: &Document, axes: &Element, axis: &str, x: f64, y: f64, value: f64) {
    let tick = svg_element(document, "text");
    tick.set_attribute("x", &x.to_string())
        .expect("position tick");
    tick.set_attribute("y", &y.to_string())
        .expect("position tick");
    tick.set_attribute("data-role", "tick")
        .expect("mark tick label");
    tick.set_attribute("data-axis", axis)
        .expect("assign tick axis");
    tick.set_attribute("aria-hidden", "true")
        .expect("hide tick from readers");
    tick.set_text_content(Some(&format_number(value)));
    axes.append_child(&tick).expect("append tick label");
}

fn append_axis_label(document: &Document, axes: &Element, axis: &str, label: &str, x: f64, y: f64) {
    let text = svg_element(document, "text");
    text.set_attribute("x", &x.to_string())
        .expect("position axis label");
    text.set_attribute("y", &y.to_string())
        .expect("position axis label");
    text.set_attribute("data-role", "axis-label")
        .expect("mark axis label");
    text.set_attribute("data-axis", axis)
        .expect("assign label axis");
    text.set_attribute("text-anchor", "middle")
        .expect("centre axis label");
    text.set_text_content(Some(label));
    axes.append_child(&text).expect("append axis label");
}

fn append_marks(
    document: &Document,
    svg: &Element,
    frame: &Frame,
    props: &ScatterPlotProps,
    x_domain: Domain,
    y_domain: Domain,
) {
    let marks = svg_element(document, "g");
    marks
        .set_attribute("data-role", "marks")
        .expect("mark point group");
    for (index, point) in props.points.iter().enumerate() {
        let x = frame.x(x_domain.ratio(point.x));
        let y = frame.y(y_domain.ratio(point.y));
        let mark = svg_element(document, "circle");
        mark.set_attribute("cx", &x.to_string())
            .expect("position mark");
        mark.set_attribute("cy", &y.to_string())
            .expect("position mark");
        mark.set_attribute("r", &point.size.unwrap_or(4.0).max(1.0).to_string())
            .expect("size mark");
        mark.set_attribute("data-index", &index.to_string())
            .expect("index mark");
        mark.set_attribute("data-x", &format_number(point.x))
            .expect("set mark x value");
        mark.set_attribute("data-y", &format_number(point.y))
            .expect("set mark y value");
        mark.set_attribute("data-color", point.color.as_deref().unwrap_or("primary"))
            .expect("set mark colour token");
        if !x_domain.contains(point.x) || !y_domain.contains(point.y) {
            mark.set_attribute("data-outside", "true")
                .expect("flag clamped mark");
        }

        let title = svg_element(document, "title");
        title.set_text_content(Some(&describe(point)));
        mark.append_child(&title).expect("append mark title");
        marks.append_child(&mark).expect("append mark");

        if props.show_labels {
            if let Some(label) = point.label.as_deref() {
                append_point_label(document, &marks, index, label, x, y);
            }
        }
    }
    svg.append_child(&marks).expect("append mark group");
}

fn append_point_label(
    document: &Document,
    marks: &Element,
    index: usize,
    label: &str,
    x: f64,
    y: f64,
) {
    let text = svg_element(document, "text");
    text.set_attribute("x", &x.to_string())
        .expect("position mark label");
    text.set_attribute("y", &y.to_string())
        .expect("position mark label");
    text.set_attribute("data-role", "point-label")
        .expect("mark point label");
    text.set_attribute("data-index", &index.to_string())
        .expect("index point label");
    text.set_attribute("aria-hidden", "true")
        .expect("hide duplicated label");
    text.set_text_content(Some(label));
    marks.append_child(&text).expect("append mark label");
}

fn describe(point: &ScatterPoint) -> String {
    let coordinates = format!("{}, {}", format_number(point.x), format_number(point.y));
    match point.label.as_deref() {
        Some(label) => format!("{label}: {coordinates}"),
        None => coordinates,
    }
}

fn summary(props: &ScatterPlotProps, x_domain: Domain, y_domain: Domain) -> String {
    let x_label = props.x_label.as_deref().unwrap_or("x");
    let y_label = props.y_label.as_deref().unwrap_or("y");
    format!(
        "Scatter plot of {} points, {x_label} from {} to {}, {y_label} from {} to {}",
        props.points.len(),
        format_number(x_domain.minimum),
        format_number(x_domain.maximum),
        format_number(y_domain.minimum),
        format_number(y_domain.maximum)
    )
}

fn domain_text(domain: Domain) -> String {
    format!(
        "{} {}",
        format_number(domain.minimum),
        format_number(domain.maximum)
    )
}

fn set_line(line: &Element, x1: f64, y1: f64, x2: f64, y2: f64) {
    line.set_attribute("x1", &x1.to_string())
        .expect("set line start");
    line.set_attribute("y1", &y1.to_string())
        .expect("set line start");
    line.set_attribute("x2", &x2.to_string())
        .expect("set line end");
    line.set_attribute("y2", &y2.to_string())
        .expect("set line end");
}

fn svg_element(document: &Document, tag: &str) -> Element {
    document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
        .expect("create SVG element")
}

fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "0".to_string();
    }
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}
