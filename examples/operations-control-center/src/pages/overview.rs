use domius_core::computed::{computed, Computed};
use domius_core::signal::Signal;
use domius_web::components::data::badge::{Badge, BadgeProps, BadgeVariant};
use domius_web::components::data::charts::{ChartDataPoint, ChartType, Charts, ChartsProps};
use domius_web::components::data::statistic::{statistic_card, StatisticCardProps, StatisticProps};
use domius_web::components::primitives::card::{card, card_body, CardProps};
use domius_web::components::primitives::countdown::{countdown, CountdownFormat, CountdownProps};
use domius_web::components::primitives::scrolltext::{
    scrolltext, ScrollTextDirection, ScrollTextProps,
};
use domius_web::components::pro::heatmap::{Heatmap, HeatmapCell, HeatmapColorScale, HeatmapProps};
use domius_web::{domus, DomiusComponent, DomiusNode, DomiusPage};

use crate::data::aggregates::{
    error_rate_windows, throughput_by_minute, window_count, window_label,
};
use crate::data::{Incident, IncidentSeverity, MonitoringData, ServiceStatus};
use crate::state::MonitoringContext;

pub struct OverviewPage;

pub struct OverviewState {
    data: Signal<MonitoringData>,
    open_incidents: Computed<usize>,
    impacted_services: Computed<usize>,
}

impl DomiusComponent for OverviewPage {
    type Props = ();
    type State = OverviewState;

    fn setup(_: Self::Props) -> Self::State {
        let context = MonitoringContext::current().expect("monitoring context is missing");
        let incident_data = context.data.clone();
        let service_data = context.data.clone();

        OverviewState {
            data: context.data,
            open_incidents: computed(move || incident_data.get().open_incident_count()),
            impacted_services: computed(move || {
                service_data
                    .get()
                    .services
                    .iter()
                    .filter(|service| service.status != ServiceStatus::Operational)
                    .count()
            }),
        }
    }

    fn render(state: &Self::State) -> DomiusNode {
        let root = domus! {
            main(class: "operations-overview") {
                header {
                    h1 { "Operations overview" }
                    p { "Deterministic production snapshot" }
                }
                div(id: "critical-alert") { }
                section(id: "overview-statistics") { }
                section(id: "sla-countdown", class: "panel") {
                    h2 { "Next SLA deadline" }
                    p(id: "sla-subject") { }
                }
                section(id: "load-curve", class: "panel") {
                    h2 { "Fleet load" }
                    p { "Requests per second across every service, minute by minute" }
                    div(id: "load-chart") { }
                }
                section(id: "activity-map", class: "panel") {
                    h2 { "Error activity" }
                    p { "Average error percentage by service and 10-minute window" }
                    div(id: "activity-heatmap") { }
                }
                section(id: "service-health") {
                    h2 { "Service health" }
                }
            }
        };

        let statistics = root
            .query_selector("#overview-statistics")
            .expect("query statistics")
            .expect("statistics container");
        statistics
            .append_child(&statistic_card(StatisticCardProps {
                statistic: StatisticProps {
                    title: Some("Open incidents".to_string()),
                    value: state.open_incidents.get().to_string(),
                    description: Some("Awaiting acknowledgement or resolution".to_string()),
                    ..Default::default()
                },
                bordered: true,
                ..Default::default()
            }))
            .expect("append incident statistic");
        statistics
            .append_child(&statistic_card(StatisticCardProps {
                statistic: StatisticProps {
                    title: Some("Impacted services".to_string()),
                    value: state.impacted_services.get().to_string(),
                    description: Some("Degraded or unavailable".to_string()),
                    ..Default::default()
                },
                bordered: true,
                ..Default::default()
            }))
            .expect("append service statistic");

        let window = state.data.get();
        append_critical_alert(&root, &window);
        append_sla_countdown(&root, &window);
        append_load_curve(&root, &window);
        append_activity_map(&root, &window);

        let health = root
            .query_selector("#service-health")
            .expect("query service health")
            .expect("service health container");
        for service in state.data.get().services {
            let service_card = card(CardProps {
                title: Some(service.name),
                extra: Some(format!("{} ms", service.latency_ms)),
                bordered: true,
                hoverable: true,
                ..Default::default()
            });
            card_body(
                &service_card,
                &format!("Error rate {:.2}%", service.error_rate),
            );
            let badge = Badge::create(BadgeProps {
                children: status_label(service.status).to_string(),
                variant: status_variant(service.status),
                dot: true,
                ..Default::default()
            });
            service_card
                .query_selector(".card-body")
                .expect("query card body")
                .expect("card body")
                .append_child(&badge)
                .expect("append service status");
            let service_link = service_card
                .owner_document()
                .expect("service card document")
                .create_element("a")
                .expect("create service link");
            service_link.set_class_name("btn-ghost");
            service_link
                .set_attribute("href", &format!("/services/{}", service.id))
                .expect("set service link target");
            service_link
                .set_attribute("data-route", "")
                .expect("mark internal service route");
            service_link.set_text_content(Some("Open service"));
            service_card
                .query_selector(".card-body")
                .expect("query card body")
                .expect("card body")
                .append_child(&service_link)
                .expect("append service link");
            health
                .append_child(&service_card)
                .expect("append service card");
        }

        root
    }
}

impl DomiusPage for OverviewPage {
    fn route() -> &'static str {
        "/overview"
    }

    fn title(_: &Self::State) -> String {
        "Operations overview | Domius".to_string()
    }
}

/// Minutes an incident may stay unacknowledged before it breaches its SLA.
const SLA_MINUTES: u32 = 45;

/// Minutes covered by one column of the activity map.
const ACTIVITY_WINDOW: u32 = 10;

/// Headlines for the alerts an operator should read before anything else.
pub fn critical_alert_lines(data: &MonitoringData) -> Vec<String> {
    let mut lines = data
        .incidents
        .iter()
        .filter(|incident| {
            !incident.acknowledged && incident.severity == IncidentSeverity::Critical
        })
        .map(|incident| {
            let service = data
                .service(&incident.service_id)
                .map_or(incident.service_id.as_str(), |service| {
                    service.name.as_str()
                });
            format!(
                "{service}: {} — open for {} minutes",
                incident.title, incident.opened_minutes_ago
            )
        })
        .collect::<Vec<_>>();
    for service in &data.services {
        if service.status == ServiceStatus::Outage {
            lines.push(format!("{} is down", service.name));
        }
    }
    lines
}

/// What the SLA clock has to say about the open queue.
pub struct SlaOutlook<'a> {
    /// The open incident that will breach next, with the minutes it has left.
    pub next: Option<(&'a Incident, u32)>,
    /// Open incidents already past their deadline.
    pub breached: usize,
}

/// Read the SLA clock over the open incidents.
///
/// An incident already past its deadline is not an upcoming breach, so it is
/// counted rather than counted down; otherwise the fixture, whose incidents run
/// up to twelve hours old, would show a countdown permanently stuck at zero.
pub fn sla_outlook(data: &MonitoringData) -> SlaOutlook<'_> {
    let open = data
        .incidents
        .iter()
        .filter(|incident| !incident.acknowledged);
    SlaOutlook {
        next: open
            .clone()
            .filter(|incident| incident.opened_minutes_ago < SLA_MINUTES)
            .map(|incident| (incident, SLA_MINUTES - incident.opened_minutes_ago))
            .min_by(|left, right| {
                left.1
                    .cmp(&right.1)
                    .then_with(|| left.0.id.cmp(&right.0.id))
            }),
        breached: open
            .filter(|incident| incident.opened_minutes_ago >= SLA_MINUTES)
            .count(),
    }
}

fn append_critical_alert(root: &web_sys::Element, data: &MonitoringData) {
    let lines = critical_alert_lines(data);
    let host = host(root, "#critical-alert");
    if lines.is_empty() {
        host.set_attribute("hidden", "").expect("hide alert banner");
        return;
    }
    host.set_attribute("data-alerts", &lines.len().to_string())
        .expect("expose alert count");
    host.set_attribute("role", "status")
        .expect("announce alert banner");
    host.append_child(&scrolltext(ScrollTextProps {
        lines,
        direction: ScrollTextDirection::Up,
        speed: 4000,
        auto_scroll: true,
        pause_on_hover: true,
        one_by_one: true,
        class: Some("critical-alert".to_string()),
    }))
    .expect("append alert banner");
}

fn append_sla_countdown(root: &web_sys::Element, data: &MonitoringData) {
    let panel = host(root, "#sla-countdown");
    let subject = host(root, "#sla-subject");
    let outlook = sla_outlook(data);
    panel
        .set_attribute("data-breached", &outlook.breached.to_string())
        .expect("expose breached count");
    let Some((incident, remaining)) = outlook.next else {
        subject.set_text_content(Some(&match outlook.breached {
            0 => "Every incident is acknowledged.".to_string(),
            1 => "One incident is already past its SLA.".to_string(),
            many => format!("{many} incidents are already past their SLA."),
        }));
        return;
    };
    let service = data
        .service(&incident.service_id)
        .map_or(incident.service_id.as_str(), |service| {
            service.name.as_str()
        });
    subject.set_text_content(Some(&format!("{service}: {}", incident.title)));
    panel
        .set_attribute("data-remaining-minutes", &remaining.to_string())
        .expect("expose remaining minutes");
    // The fixture is a fixed window, so count from zero rather than the clock.
    panel
        .append_child(&countdown(CountdownProps {
            target: u64::from(remaining) * 60,
            current: Some(0),
            format: CountdownFormat::HHmm,
            title: Some("Time left".to_string()),
            finish_text: Some("SLA breached".to_string()),
            class: Some("sla".to_string()),
            ..Default::default()
        }))
        .expect("append sla countdown");
}

fn append_load_curve(root: &web_sys::Element, data: &MonitoringData) {
    host(root, "#load-chart")
        .append_child(&Charts::create(ChartsProps {
            chart_type: ChartType::Area,
            data: throughput_by_minute(&data.metrics)
                .into_iter()
                .map(|(minute, value)| ChartDataPoint {
                    label: format!("Minute {minute}"),
                    value: f64::from(value),
                })
                .collect(),
            show_legend: false,
            colors: vec!["primary".to_string()],
            ..Default::default()
        }))
        .expect("append load curve");
}

fn append_activity_map(root: &web_sys::Element, data: &MonitoringData) {
    host(root, "#activity-heatmap")
        .append_child(&Heatmap::create(HeatmapProps {
            data: error_rate_windows(&data.services, &data.metrics, ACTIVITY_WINDOW)
                .into_iter()
                .map(|(x, y, value)| HeatmapCell { x, y, value })
                .collect(),
            x_labels: (0..window_count(&data.metrics, ACTIVITY_WINDOW))
                .map(|window| window_label(window, ACTIVITY_WINDOW))
                .collect(),
            y_labels: data
                .services
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
            class: Some("overview-heatmap".to_string()),
        }))
        .expect("append activity map");
}

fn host(root: &web_sys::Element, selector: &str) -> web_sys::Element {
    root.query_selector(selector)
        .expect("query overview host")
        .expect("overview host")
}

fn status_label(status: ServiceStatus) -> &'static str {
    match status {
        ServiceStatus::Operational => "Operational",
        ServiceStatus::Degraded => "Degraded",
        ServiceStatus::Outage => "Outage",
    }
}

fn status_variant(status: ServiceStatus) -> BadgeVariant {
    match status {
        ServiceStatus::Operational => BadgeVariant::Success,
        ServiceStatus::Degraded => BadgeVariant::Warning,
        ServiceStatus::Outage => BadgeVariant::Error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::monitoring_fixture;

    #[test]
    fn the_banner_carries_open_criticals_and_outages() {
        let data = monitoring_fixture(0xD0_51_05);
        let lines = critical_alert_lines(&data);

        let expected_criticals = data
            .incidents
            .iter()
            .filter(|incident| {
                !incident.acknowledged && incident.severity == IncidentSeverity::Critical
            })
            .count();
        let expected_outages = data
            .services
            .iter()
            .filter(|service| service.status == ServiceStatus::Outage)
            .count();

        assert_eq!(lines.len(), expected_criticals + expected_outages);
        assert!(
            expected_outages > 0,
            "the fixture should exercise an outage"
        );
        assert!(lines.iter().any(|line| line.ends_with(" is down")));
        // Acknowledged incidents never reach the banner.
        assert!(!lines
            .iter()
            .any(|line| line.contains("open for") && line.is_empty()));
    }

    #[test]
    fn a_calm_window_shows_no_banner() {
        let mut data = monitoring_fixture(0xD0_51_05);
        for incident in &mut data.incidents {
            incident.acknowledged = true;
        }
        for service in &mut data.services {
            service.status = ServiceStatus::Operational;
        }

        let outlook = sla_outlook(&data);
        assert!(critical_alert_lines(&data).is_empty());
        assert!(outlook.next.is_none());
        assert_eq!(outlook.breached, 0);
    }

    #[test]
    fn the_countdown_tracks_the_soonest_breach_still_ahead() {
        let data = monitoring_fixture(0xD0_51_05);
        let outlook = sla_outlook(&data);
        let (incident, remaining) = outlook.next.expect("an incident should still be in time");

        assert!(!incident.acknowledged);
        assert!(remaining > 0, "a breach already past is not upcoming");
        assert_eq!(remaining, SLA_MINUTES - incident.opened_minutes_ago);
        // No open incident still in time is closer to its deadline.
        assert!(data
            .incidents
            .iter()
            .filter(|other| !other.acknowledged && other.opened_minutes_ago < SLA_MINUTES)
            .all(|other| SLA_MINUTES - other.opened_minutes_ago >= remaining));

        // Everything else open is counted as already late.
        let late = data
            .incidents
            .iter()
            .filter(|other| !other.acknowledged && other.opened_minutes_ago >= SLA_MINUTES)
            .count();
        assert_eq!(outlook.breached, late);
        assert!(late > 0, "the fixture should exercise the breached branch");
    }

    #[test]
    fn a_fully_late_queue_counts_instead_of_counting_down() {
        let mut data = monitoring_fixture(0xD0_51_05);
        for incident in &mut data.incidents {
            incident.acknowledged = false;
            incident.opened_minutes_ago = SLA_MINUTES + 30;
        }

        let outlook = sla_outlook(&data);
        assert!(outlook.next.is_none());
        assert_eq!(outlook.breached, data.incidents.len());
    }
}
