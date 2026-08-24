use std::collections::BTreeMap;

use domius_web::components::data::charts::{ChartDataPoint, ChartType, Charts, ChartsProps};
use domius_web::components::data::statistic::{statistic_card, StatisticCardProps, StatisticProps};
use domius_web::components::pro::data_grid::{DataGrid, DataGridProps, GridColumn};
use domius_web::components::pro::heatmap::{
    Heatmap, HeatmapCell, HeatmapColorScale, HeatmapProps,
};
use domius_web::components::pro::pivot_table::{
    Aggregator, PivotData, PivotTable, PivotTableProps,
};
use domius_web::{domus, DomiusComponent, DomiusNode, DomiusPage};

use crate::data::{Incident, IncidentSeverity, Metric, Service};
use crate::state::MonitoringContext;

pub struct ReportsPage;

pub struct ReportsState {
    services: Vec<Service>,
    incidents: Vec<Incident>,
    metrics: Vec<Metric>,
}

impl DomiusComponent for ReportsPage {
    type Props = ();
    type State = ReportsState;

    fn setup(_: Self::Props) -> Self::State {
        let data = MonitoringContext::current()
            .expect("monitoring context is missing")
            .data
            .get();
        ReportsState {
            services: data.services,
            incidents: data.incidents,
            metrics: data.metrics,
        }
    }

    fn render(state: &Self::State) -> DomiusNode {
        let throughput = throughput_by_minute(&state.metrics);
        let average_throughput = throughput.iter().map(|point| point.value).sum::<f64>()
            / throughput.len().max(1) as f64;
        let peak_throughput = throughput
            .iter()
            .map(|point| point.value)
            .fold(0.0, f64::max);
        let average_error = if state.metrics.is_empty() {
            0.0
        } else {
            state
                .metrics
                .iter()
                .map(|metric| metric.error_rate)
                .sum::<f64>()
                / state.metrics.len() as f64
        };
        let root = domus! {
            main(class: "reports-page") {
                header(class: "section-header") {
                    h1 { "Reliability reports" }
                    p { "A 60-minute deterministic operational window" }
                }
                section(id: "report-statistics") { }
                section(class: "panel") {
                    h2 { "Fleet throughput" }
                    div(id: "throughput-chart") { }
                }
                section(class: "panel") {
                    h2 { "Average error rate by service" }
                    div(id: "error-chart") { }
                }
                section(class: "panel") {
                    h2 { "Incident severity mix" }
                    div(id: "severity-chart") { }
                }
                section(class: "panel") {
                    h2 { "Raw metric workspace" }
                    p { "All 360 measurements in a frozen-column grid" }
                    div(id: "metric-grid") { }
                }
                section(class: "panel") {
                    h2 { "Throughput by operational window" }
                    p { "Average requests per second grouped by service and 20-minute window" }
                    div(id: "metric-pivot") { }
                }
                section(class: "panel") {
                    h2 { "Error-rate activity map" }
                    p { "Average error percentage by service and 10-minute window" }
                    div(id: "error-heatmap") { }
                }
            }
        };

        let statistics = root
            .query_selector("#report-statistics")
            .expect("query report statistics")
            .expect("report statistics host");
        append_statistic(
            &statistics,
            "Average throughput",
            format!("{average_throughput:.0}"),
            Some("req/s"),
        );
        append_statistic(
            &statistics,
            "Peak throughput",
            format!("{peak_throughput:.0}"),
            Some("req/s"),
        );
        append_statistic(
            &statistics,
            "Average error rate",
            format!("{average_error:.2}"),
            Some("%"),
        );

        append_chart(
            &root,
            "#throughput-chart",
            ChartsProps {
                chart_type: ChartType::Line,
                data: throughput,
                show_legend: false,
                ..Default::default()
            },
        );
        append_chart(
            &root,
            "#error-chart",
            ChartsProps {
                chart_type: ChartType::Bar,
                data: error_rate_by_service(&state.services, &state.metrics),
                colors: vec!["warning".to_string()],
                ..Default::default()
            },
        );
        append_chart(
            &root,
            "#severity-chart",
            ChartsProps {
                chart_type: ChartType::Donut,
                data: incidents_by_severity(&state.incidents),
                colors: vec![
                    "neutral".to_string(),
                    "primary".to_string(),
                    "warning".to_string(),
                    "critical".to_string(),
                ],
                ..Default::default()
            },
        );
        root.query_selector("#metric-grid")
            .expect("query metric grid")
            .expect("metric grid host")
            .append_child(&metric_grid(&state.metrics))
            .expect("append metric grid");
        root.query_selector("#metric-pivot")
            .expect("query metric pivot")
            .expect("metric pivot host")
            .append_child(&metric_pivot(&state.services, &state.metrics))
            .expect("append metric pivot");
        root.query_selector("#error-heatmap")
            .expect("query error heatmap")
            .expect("error heatmap host")
            .append_child(&error_heatmap(&state.services, &state.metrics))
            .expect("append error heatmap");
        root
    }
}

fn error_heatmap(services: &[Service], metrics: &[Metric]) -> web_sys::Element {
    Heatmap::create(HeatmapProps {
        data: heatmap_data(services, metrics),
        x_labels: (0..6)
            .map(|window| format!("{:02}-{:02} min", window * 10, window * 10 + 9))
            .collect(),
        y_labels: services.iter().map(|service| service.name.clone()).collect(),
        color_scale: HeatmapColorScale::Sequential(vec![
            "healthy".to_string(),
            "watch".to_string(),
            "warning".to_string(),
            "critical".to_string(),
        ]),
        show_values: true,
        on_cell_click: None,
        class: Some("report-heatmap".to_string()),
    })
}

fn heatmap_data(services: &[Service], metrics: &[Metric]) -> Vec<HeatmapCell> {
    services
        .iter()
        .enumerate()
        .flat_map(|(y, service)| {
            (0..6).map(move |x| {
                let window = metrics
                    .iter()
                    .filter(|metric| metric.service_id == service.id && metric.minute / 10 == x as u32)
                    .collect::<Vec<_>>();
                HeatmapCell {
                    x,
                    y,
                    value: window.iter().map(|metric| metric.error_rate).sum::<f64>()
                        / window.len().max(1) as f64,
                }
            })
        })
        .collect()
}

fn metric_pivot(services: &[Service], metrics: &[Metric]) -> web_sys::Element {
    PivotTable::create(PivotTableProps {
        data: metric_pivot_data(services, metrics),
        rows: vec!["service".to_string()],
        columns: vec!["window".to_string()],
        values: vec!["throughput".to_string()],
        aggregator: Aggregator::Average,
        show_totals: true,
        collapsible: true,
        class: Some("report-pivot".to_string()),
    })
}

fn metric_pivot_data(services: &[Service], metrics: &[Metric]) -> Vec<PivotData> {
    let names = services
        .iter()
        .map(|service| (service.id.as_str(), service.name.as_str()))
        .collect::<BTreeMap<_, _>>();
    metrics
        .iter()
        .map(|metric| {
            [
                (
                    "service".to_string(),
                    names
                        .get(metric.service_id.as_str())
                        .copied()
                        .unwrap_or(metric.service_id.as_str())
                        .to_string(),
                ),
                (
                    "window".to_string(),
                    format!("{:02}-{:02} min", metric.minute / 20 * 20, metric.minute / 20 * 20 + 19),
                ),
                ("throughput".to_string(), metric.requests_per_second.to_string()),
            ]
            .into_iter()
            .collect()
        })
        .collect()
}

fn metric_grid(metrics: &[Metric]) -> web_sys::Element {
    DataGrid::create(DataGridProps {
        columns: vec![
            grid_column("service", "Service"),
            grid_column("minute", "Minute"),
            grid_column("throughput", "Requests/s"),
            grid_column("error", "Error rate"),
        ],
        data: metric_rows(metrics),
        editable: false,
        virtualized: true,
        frozen_rows: 1,
        frozen_columns: 1,
        ..Default::default()
    })
}

fn metric_rows(metrics: &[Metric]) -> Vec<Vec<String>> {
    metrics
        .iter()
        .map(|metric| {
            vec![
                metric.service_id.clone(),
                metric.minute.to_string(),
                metric.requests_per_second.to_string(),
                format!("{:.2}%", metric.error_rate),
            ]
        })
        .collect()
}

fn grid_column(field: &str, header: &str) -> GridColumn {
    GridColumn {
        field: field.to_string(),
        header: header.to_string(),
        width: None,
        editable: false,
        cell_renderer: None,
    }
}

impl DomiusPage for ReportsPage {
    fn route() -> &'static str {
        "/reports"
    }

    fn title(_: &Self::State) -> String {
        "Reliability reports | Domius".to_string()
    }
}

fn append_statistic(host: &web_sys::Element, title: &str, value: String, suffix: Option<&str>) {
    host.append_child(&statistic_card(StatisticCardProps {
        statistic: StatisticProps {
            title: Some(title.to_string()),
            value,
            suffix: suffix.map(str::to_string),
            ..Default::default()
        },
        bordered: true,
        ..Default::default()
    }))
    .expect("append report statistic");
}

fn append_chart(root: &web_sys::Element, selector: &str, props: ChartsProps) {
    root.query_selector(selector)
        .expect("query report chart")
        .expect("report chart host")
        .append_child(&Charts::create(props))
        .expect("append report chart");
}

fn throughput_by_minute(metrics: &[Metric]) -> Vec<ChartDataPoint> {
    let mut totals = BTreeMap::<u32, u32>::new();
    for metric in metrics {
        *totals.entry(metric.minute).or_default() += metric.requests_per_second;
    }
    totals
        .into_iter()
        .map(|(minute, value)| ChartDataPoint {
            label: format!("Minute {minute}"),
            value: f64::from(value),
        })
        .collect()
}

fn error_rate_by_service(services: &[Service], metrics: &[Metric]) -> Vec<ChartDataPoint> {
    services
        .iter()
        .map(|service| {
            let service_metrics = metrics
                .iter()
                .filter(|metric| metric.service_id == service.id)
                .collect::<Vec<_>>();
            let average = service_metrics
                .iter()
                .map(|metric| metric.error_rate)
                .sum::<f64>()
                / service_metrics.len().max(1) as f64;
            ChartDataPoint {
                label: service.name.clone(),
                value: average,
            }
        })
        .collect()
}

fn incidents_by_severity(incidents: &[Incident]) -> Vec<ChartDataPoint> {
    [
        ("Low", IncidentSeverity::Low),
        ("Medium", IncidentSeverity::Medium),
        ("High", IncidentSeverity::High),
        ("Critical", IncidentSeverity::Critical),
    ]
    .into_iter()
    .map(|(label, severity)| ChartDataPoint {
        label: label.to_string(),
        value: incidents
            .iter()
            .filter(|incident| incident.severity == severity)
            .count() as f64,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::monitoring_fixture;

    #[test]
    fn report_aggregations_cover_the_complete_fixture() {
        let data = monitoring_fixture(7);
        let throughput = throughput_by_minute(&data.metrics);
        let errors = error_rate_by_service(&data.services, &data.metrics);
        let severities = incidents_by_severity(&data.incidents);
        assert_eq!(throughput.len(), 60);
        assert_eq!(errors.len(), 6);
        assert_eq!(severities.len(), 4);
        assert_eq!(metric_rows(&data.metrics).len(), 360);
        let pivot = metric_pivot_data(&data.services, &data.metrics);
        assert_eq!(pivot.len(), 360);
        assert_eq!(heatmap_data(&data.services, &data.metrics).len(), 36);
        assert_eq!(
            pivot
                .iter()
                .map(|row| row["window"].as_str())
                .collect::<std::collections::BTreeSet<_>>(),
            ["00-19 min", "20-39 min", "40-59 min"].into_iter().collect()
        );
        assert_eq!(
            severities.iter().map(|point| point.value).sum::<f64>(),
            48.0
        );
    }
}
