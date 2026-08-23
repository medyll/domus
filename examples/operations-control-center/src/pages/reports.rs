use std::collections::BTreeMap;

use domius_web::components::data::charts::{ChartDataPoint, ChartType, Charts, ChartsProps};
use domius_web::components::data::statistic::{statistic_card, StatisticCardProps, StatisticProps};
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
        root
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
        assert_eq!(
            severities.iter().map(|point| point.value).sum::<f64>(),
            48.0
        );
    }
}
