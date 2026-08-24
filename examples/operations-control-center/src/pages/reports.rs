use std::collections::BTreeMap;

use domius_web::components::data::charts::{ChartDataPoint, ChartType, Charts, ChartsProps};
use domius_web::components::data::statistic::{statistic_card, StatisticCardProps, StatisticProps};
use domius_web::components::pro::data_grid::{DataGrid, DataGridProps, GridColumn};
use domius_web::components::pro::heatmap::{Heatmap, HeatmapCell, HeatmapColorScale, HeatmapProps};
use domius_web::components::pro::pivot_table::{
    Aggregator, PivotData, PivotTable, PivotTableProps,
};
use domius_web::components::pro::result::{Result, ResultAction, ResultProps, ResultStatus};
use domius_web::components::pro::scatter_plot::{ScatterPlot, ScatterPlotProps, ScatterPoint};
use domius_web::components::pro::watermark::{Watermark, WatermarkProps};
use domius_web::{domus, DomiusComponent, DomiusNode, DomiusPage};

use crate::data::{Incident, IncidentSeverity, Metric, Service, ServiceStatus};
use crate::state::MonitoringContext;

pub struct ReportsPage;

pub struct ReportsState {
    services: Vec<Service>,
    incidents: Vec<Incident>,
    metrics: Vec<Metric>,
    status: ReportStatus,
}

/// Whether the loaded window can be reported on at all.
#[derive(Clone, Debug, PartialEq)]
pub enum ReportStatus {
    Ready,
    Empty,
    Failed(String),
}

/// Reports are only as trustworthy as the window behind them, so decide up front.
fn report_status(data: &crate::data::MonitoringData) -> ReportStatus {
    if data.services.is_empty() || data.metrics.is_empty() {
        return ReportStatus::Empty;
    }
    if let Some(orphan) = data
        .metrics
        .iter()
        .find(|metric| data.service(&metric.service_id).is_none())
    {
        return ReportStatus::Failed(format!(
            "A measurement references the unknown service {}.",
            orphan.service_id
        ));
    }
    ReportStatus::Ready
}

impl DomiusComponent for ReportsPage {
    type Props = ();
    type State = ReportsState;

    fn setup(_: Self::Props) -> Self::State {
        let data = MonitoringContext::current()
            .expect("monitoring context is missing")
            .data
            .get();
        let status = report_status(&data);
        ReportsState {
            services: data.services,
            incidents: data.incidents,
            metrics: data.metrics,
            status,
        }
    }

    fn render(state: &Self::State) -> DomiusNode {
        match &state.status {
            ReportStatus::Empty => {
                return unavailable_report(Result::create(ResultProps {
                    status: ResultStatus::Info,
                    title: "No measurements in this window".to_string(),
                    description: Some(
                        "The monitoring window holds no metric, so no report can be built."
                            .to_string(),
                    ),
                    actions: report_exits(),
                    class: Some("report-empty".to_string()),
                    ..Default::default()
                }))
            }
            ReportStatus::Failed(reason) => {
                return unavailable_report(Result::create(ResultProps {
                    status: ResultStatus::Error,
                    title: "Reports unavailable".to_string(),
                    description: Some(reason.clone()),
                    actions: report_exits(),
                    class: Some("report-failure".to_string()),
                    ..Default::default()
                }))
            }
            ReportStatus::Ready => {}
        }

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
                div(id: "export-region", class: "export-region") {
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
                    section(class: "panel") {
                        h2 { "Throughput against error rate" }
                        p { "One point per service and 10-minute window, sized by open incidents" }
                        div(id: "correlation-scatter") { }
                    }
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
        root.query_selector("#correlation-scatter")
            .expect("query correlation scatter")
            .expect("correlation scatter host")
            .append_child(&correlation_scatter(
                &state.services,
                &state.incidents,
                &state.metrics,
            ))
            .expect("append correlation scatter");
        mark_export_region(&root);
        root
    }
}

/// Wrap a result state in the page frame so navigation and titles stay in place.
fn unavailable_report(result: web_sys::Element) -> DomiusNode {
    let root = domus! {
        main(class: "reports-page") {
            header(class: "section-header") {
                h1 { "Reliability reports" }
                p { "A 60-minute deterministic operational window" }
            }
            div(id: "report-state") { }
        }
    };
    root.query_selector("#report-state")
        .expect("query report state")
        .expect("report state host")
        .append_child(&result)
        .expect("append report state");
    mark_internal_links(&root);
    root
}

/// Both dead ends offer the same two ways back into the application.
fn report_exits() -> Vec<ResultAction> {
    vec![
        ResultAction::new("Back to overview", "/overview").primary(),
        ResultAction::new("Open incidents", "/incidents"),
    ]
}

/// Let the shell intercept these links instead of reloading the application.
fn mark_internal_links(root: &web_sys::Element) {
    let links = root
        .query_selector_all(".domius-result-action")
        .expect("query result actions");
    for index in 0..links.length() {
        if let Some(link) = links
            .item(index)
            .and_then(|node| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(node).ok())
        {
            link.set_attribute("data-route", "")
                .expect("mark internal result link");
        }
    }
}

/// Flag the analytical views as the exportable area and lay the watermark over them.
fn mark_export_region(root: &web_sys::Element) {
    let region = root
        .query_selector("#export-region")
        .expect("query export region")
        .expect("export region host");
    region
        .set_attribute("data-exportable", "true")
        .expect("flag export region");
    region
        .append_child(&Watermark::create(WatermarkProps {
            text: Some("OPERATIONS INTERNAL".to_string()),
            opacity: 0.12,
            rotation: -24.0,
            gap: (260, 160),
            offset: (130, 80),
            font_size: 20,
            font_color: "#1f2933".to_string(),
            class: Some("report-watermark".to_string()),
            ..Default::default()
        }))
        .expect("append export watermark");
}

fn correlation_scatter(
    services: &[Service],
    incidents: &[Incident],
    metrics: &[Metric],
) -> web_sys::Element {
    ScatterPlot::create(ScatterPlotProps {
        points: correlation_points(services, incidents, metrics),
        x_label: Some("Requests per second".to_string()),
        y_label: Some("Error rate %".to_string()),
        // Anchor the error axis at zero so quiet windows read as quiet.
        y_min: Some(0.0),
        show_grid: true,
        show_labels: false,
        class: Some("report-scatter".to_string()),
        ..Default::default()
    })
}

/// Correlate throughput and error rate over the same 10-minute windows as the heatmap.
fn correlation_points(
    services: &[Service],
    incidents: &[Incident],
    metrics: &[Metric],
) -> Vec<ScatterPoint> {
    services
        .iter()
        .flat_map(|service| {
            let open = incidents
                .iter()
                .filter(|incident| incident.service_id == service.id && !incident.acknowledged)
                .count();
            (0..6).map(move |window| {
                let sample = metrics
                    .iter()
                    .filter(|metric| {
                        metric.service_id == service.id && metric.minute / 10 == window
                    })
                    .collect::<Vec<_>>();
                let divisor = sample.len().max(1) as f64;
                ScatterPoint {
                    x: sample
                        .iter()
                        .map(|metric| f64::from(metric.requests_per_second))
                        .sum::<f64>()
                        / divisor,
                    y: sample.iter().map(|metric| metric.error_rate).sum::<f64>() / divisor,
                    label: Some(format!(
                        "{} {:02}-{:02} min",
                        service.name,
                        window * 10,
                        window * 10 + 9
                    )),
                    color: Some(status_token(service.status).to_string()),
                    size: Some(4.0 + open.min(6) as f64),
                }
            })
        })
        .collect()
}

fn status_token(status: ServiceStatus) -> &'static str {
    match status {
        ServiceStatus::Operational => "healthy",
        ServiceStatus::Degraded => "warning",
        ServiceStatus::Outage => "critical",
    }
}

fn error_heatmap(services: &[Service], metrics: &[Metric]) -> web_sys::Element {
    Heatmap::create(HeatmapProps {
        data: heatmap_data(services, metrics),
        x_labels: (0..6)
            .map(|window| format!("{:02}-{:02} min", window * 10, window * 10 + 9))
            .collect(),
        y_labels: services
            .iter()
            .map(|service| service.name.clone())
            .collect(),
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
                    .filter(|metric| {
                        metric.service_id == service.id && metric.minute / 10 == x as u32
                    })
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
                    format!(
                        "{:02}-{:02} min",
                        metric.minute / 20 * 20,
                        metric.minute / 20 * 20 + 19
                    ),
                ),
                (
                    "throughput".to_string(),
                    metric.requests_per_second.to_string(),
                ),
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

    fn title(state: &Self::State) -> String {
        match state.status {
            ReportStatus::Ready => "Reliability reports | Domius".to_string(),
            ReportStatus::Empty => "No reportable measurements | Domius".to_string(),
            ReportStatus::Failed(_) => "Reports unavailable | Domius".to_string(),
        }
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
            ["00-19 min", "20-39 min", "40-59 min"]
                .into_iter()
                .collect()
        );
        assert_eq!(
            severities.iter().map(|point| point.value).sum::<f64>(),
            48.0
        );
    }

    #[test]
    fn the_seeded_window_is_reportable_but_a_broken_one_is_not() {
        let data = monitoring_fixture(7);
        assert_eq!(report_status(&data), ReportStatus::Ready);

        let mut without_metrics = data.clone();
        without_metrics.metrics.clear();
        assert_eq!(report_status(&without_metrics), ReportStatus::Empty);

        let mut without_services = data.clone();
        without_services.services.clear();
        assert_eq!(report_status(&without_services), ReportStatus::Empty);

        let mut orphaned = data;
        orphaned.metrics[12].service_id = "svc-99".to_string();
        assert_eq!(
            report_status(&orphaned),
            ReportStatus::Failed(
                "A measurement references the unknown service svc-99.".to_string()
            )
        );
    }

    #[test]
    fn correlation_reads_the_same_windows_as_the_heatmap() {
        let data = monitoring_fixture(7);
        let points = correlation_points(&data.services, &data.incidents, &data.metrics);
        let cells = heatmap_data(&data.services, &data.metrics);
        assert_eq!(points.len(), 36);
        assert_eq!(points.len(), cells.len());

        // Both views average the same measurements over the same windows.
        for (point, cell) in points.iter().zip(&cells) {
            assert!((point.y - cell.value).abs() < f64::EPSILON);
        }

        assert!(points.iter().all(|point| point.x > 0.0));
        assert!(points
            .iter()
            .all(|point| point.size.unwrap_or_default() >= 4.0));
        assert_eq!(
            points[0].label.as_deref(),
            Some(format!("{} 00-09 min", data.services[0].name).as_str())
        );
        assert_eq!(
            points
                .iter()
                .filter_map(|point| point.color.clone())
                .collect::<std::collections::BTreeSet<_>>(),
            ["critical", "healthy", "warning"]
                .into_iter()
                .map(str::to_string)
                .collect()
        );
    }
}
