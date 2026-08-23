//! Statistic component for Domius.
//!
//! Display statistic numbers with titles, prefixes, suffixes, and trends.

use web_sys::Element;

/// Trend type.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum TrendType {
    #[default]
    None,
    Increase,
    Decrease,
}

/// Statistic props.
#[derive(Clone, Default)]
pub struct StatisticProps {
    /// Statistic title
    pub title: Option<String>,
    /// Statistic value
    pub value: String,
    /// Value prefix (e.g., "$", "€")
    pub prefix: Option<String>,
    /// Value suffix (e.g., "%", "k")
    pub suffix: Option<String>,
    /// CSS class
    pub class: Option<String>,
    /// Trend type
    pub trend: Option<TrendType>,
    /// Trend value (e.g., "12.5%")
    pub trend_value: Option<String>,
    /// Show trend as positive (green) or negative (red)
    pub trend_positive: bool,
    /// Description text
    pub description: Option<String>,
    /// Precision for decimal numbers
    pub precision: Option<u32>,
}

/// Build a Statistic component.
///
/// # Example
///
/// ```ignore
/// use domius_web::components::statistic::{statistic, StatisticProps, TrendType};
///
/// let statistic_node = statistic(StatisticProps {
///     title: Some("Active Users".to_string()),
///     value: "112893".to_string(),
///     trend: Some(TrendType::Increase),
///     trend_value: Some("12.5%".to_string()),
///     trend_positive: true,
///     ..Default::default()
/// });
/// ```
pub fn statistic(props: StatisticProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("statistic");
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    // Title
    if let Some(title) = &props.title {
        let title_el: Element = document.create_element("div").unwrap();
        title_el.set_class_name("statistic-title");
        title_el.set_text_content(Some(title));
        container.append_child(&title_el).unwrap();
    }

    // Value container
    let value_container: Element = document.create_element("div").unwrap();
    value_container.set_class_name("statistic-value");

    // Prefix
    if let Some(prefix) = &props.prefix {
        let prefix_el: Element = document.create_element("span").unwrap();
        prefix_el.set_class_name("statistic-prefix");
        prefix_el.set_text_content(Some(prefix));
        value_container.append_child(&prefix_el).unwrap();
    }

    // Main value
    let value_el: Element = document.create_element("span").unwrap();
    value_el.set_class_name("statistic-value-main");
    value_el.set_text_content(Some(&props.value));
    value_container.append_child(&value_el).unwrap();

    // Suffix
    if let Some(suffix) = &props.suffix {
        let suffix_el: Element = document.create_element("span").unwrap();
        suffix_el.set_class_name("statistic-suffix");
        suffix_el.set_text_content(Some(suffix));
        value_container.append_child(&suffix_el).unwrap();
    }

    container.append_child(&value_container).unwrap();

    // Trend
    if let Some(trend) = props.trend {
        if trend != TrendType::None {
            let trend_el: Element = document.create_element("div").unwrap();
            trend_el.set_class_name("statistic-trend");

            if props.trend_positive {
                trend_el.set_attribute("data-trend", "positive").ok();
            } else {
                trend_el.set_attribute("data-trend", "negative").ok();
            }

            // Trend icon
            let icon = match trend {
                TrendType::Increase => "&#8593;", // ↑
                TrendType::Decrease => "&#8595;", // ↓
                _ => "",
            };

            if !icon.is_empty() {
                let icon_el: Element = document.create_element("span").unwrap();
                icon_el.set_inner_html(icon);
                trend_el.append_child(&icon_el).unwrap();
            }

            // Trend value
            if let Some(trend_value) = &props.trend_value {
                let value_el: Element = document.create_element("span").unwrap();
                value_el.set_class_name("statistic-trend-value");
                value_el.set_text_content(Some(trend_value));
                trend_el.append_child(&value_el).unwrap();
            }

            container.append_child(&trend_el).unwrap();
        }
    }

    // Description
    if let Some(description) = &props.description {
        let desc_el: Element = document.create_element("div").unwrap();
        desc_el.set_class_name("statistic-description");
        desc_el.set_text_content(Some(description));
        container.append_child(&desc_el).unwrap();
    }

    container
}

/// Statistic Card props (Statistic with card wrapper).
#[derive(Clone, Default)]
pub struct StatisticCardProps {
    /// Statistic props
    pub statistic: StatisticProps,
    /// CSS class
    pub class: Option<String>,
    /// Bordered style
    pub bordered: bool,
}

/// Build a StatisticCard component.
pub fn statistic_card(props: StatisticCardProps) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();

    let container: Element = document.create_element("div").unwrap();

    let mut classes = String::from("statistic-card");
    if props.bordered {
        classes.push_str(" statistic-card-bordered");
    }
    if let Some(class) = &props.class {
        classes.push_str(" ");
        classes.push_str(class);
    }
    container.set_class_name(&classes);

    let statistic_el = statistic(props.statistic);
    container.append_child(&statistic_el).unwrap();

    container
}
