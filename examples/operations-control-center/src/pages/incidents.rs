use domius_core::signal::signal;
use domius_core::Signal;
use domius_web::components::data::statistic::{statistic_card, StatisticCardProps, StatisticProps};
use domius_web::components::feedback::progress::{ProgressBar, ProgressProps, ProgressVariant};
use domius_web::components::feedback::toast::{ToastContainer, ToastManager};
use domius_web::context::provide_context;
use domius_web::disposal::ViewScope;
use domius_web::{domus, DomiusComponent, DomiusNode, DomiusPage};

use crate::components::{
    incident_feed, incident_history, incident_queue, IncidentFeedProps, IncidentHistoryProps,
    IncidentQueueProps,
};
use crate::data::{Incident, IncidentSeverity, MonitoringData, Service};
use crate::state::{FilterContext, MonitoringContext};

const INCIDENTS_PER_PAGE: usize = 10;

pub struct IncidentsPage;

pub struct IncidentsState {
    incidents: Vec<Incident>,
    services: Vec<Service>,
    data: Signal<MonitoringData>,
    filters: FilterContext,
    open_count: usize,
    critical_count: usize,
    acknowledged_count: usize,
}

impl DomiusComponent for IncidentsPage {
    type Props = ();
    type State = IncidentsState;

    fn setup(_: Self::Props) -> Self::State {
        let data = MonitoringContext::current()
            .expect("monitoring context is missing")
            .data;
        // The filters outlive a single visit, so reuse the shared ones when
        // they exist and publish them the first time we need them.
        let filters = FilterContext::current().unwrap_or_else(|| {
            let filters = FilterContext::over(data.clone());
            filters.clone().provide();
            filters
        });
        let window = data.get();
        let services = window.services;
        let incidents = window.incidents;
        let open_count = incidents
            .iter()
            .filter(|incident| !incident.acknowledged)
            .count();
        let critical_count = incidents
            .iter()
            .filter(|incident| incident.severity == IncidentSeverity::Critical)
            .count();
        let acknowledged_count = incidents.len() - open_count;
        IncidentsState {
            incidents,
            services,
            data,
            filters,
            open_count,
            critical_count,
            acknowledged_count,
        }
    }

    fn render(state: &Self::State) -> DomiusNode {
        let root = domus! {
            main(class: "incidents-page") {
                header(class: "section-header") {
                    h1 { "Incident command" }
                    p { "Triage the deterministic production queue" }
                }
                section(id: "incident-statistics") { }
                section(id: "resolution", class: "panel") {
                    h2 { "Resolution progress" }
                    p(id: "resolution-summary") { }
                    div(id: "resolution-progress") { }
                }
                section(class: "panel") {
                    header(class: "section-header") {
                        h2 { "Working queue" }
                        p { "Filter, reorder and acknowledge without losing your place" }
                    }
                    div(id: "incident-queue") { }
                }
                section(class: "panel") {
                    header(class: "section-header") {
                        h2 { "Progressive incident feed" }
                        p { "Scroll or use the load button to reveal the next batch" }
                    }
                    div(id: "incident-feed") { }
                }
                section(class: "panel") {
                    header(class: "section-header") {
                        h2 { "All incidents" }
                        p { "Sort, filter, select and paginate the queue" }
                    }
                    div(id: "incident-history") { }
                }
            }
        };

        let statistics = root
            .query_selector("#incident-statistics")
            .expect("query incident statistics")
            .expect("incident statistics host");
        append_statistic(
            &statistics,
            "Open",
            state.open_count,
            "Require operator action",
        );
        append_statistic(
            &statistics,
            "Critical",
            state.critical_count,
            "Highest severity across all services",
        );
        append_statistic(
            &statistics,
            "Acknowledged",
            state.acknowledged_count,
            "Already owned by an operator",
        );

        append_resolution(&root, &state.data);

        let toasts = ToastManager::new();
        provide_context(toasts.clone());
        let queue = incident_queue(IncidentQueueProps {
            filters: state.filters.clone(),
            data: state.data.clone(),
            services: state.services.clone(),
            toasts,
        });
        root.query_selector("#incident-queue")
            .expect("query incident queue")
            .expect("incident queue host")
            .append_child(&queue)
            .expect("append incident queue");
        root.append_child(&ToastContainer::create())
            .expect("append toast container");

        let feed = incident_feed(IncidentFeedProps {
            incidents: state.incidents.clone(),
            batch_size: INCIDENTS_PER_PAGE,
        });
        root.query_selector("#incident-feed")
            .expect("query incident feed")
            .expect("incident feed host")
            .append_child(&feed)
            .expect("append incident feed");

        let history = incident_history(IncidentHistoryProps {
            incidents: state.incidents.clone(),
            page_size: INCIDENTS_PER_PAGE,
            filterable: true,
            selectable: true,
        });
        root.query_selector("#incident-history")
            .expect("query incident history")
            .expect("incident history host")
            .append_child(&history)
            .expect("append incident history");
        root
    }
}

impl DomiusPage for IncidentsPage {
    fn route() -> &'static str {
        "/incidents"
    }

    fn title(_: &Self::State) -> String {
        "Incident command | Domius".to_string()
    }
}

/// How much of the queue has been dealt with, following the shared data.
fn append_resolution(root: &web_sys::Element, data: &Signal<MonitoringData>) {
    let panel = root
        .query_selector("#resolution")
        .expect("query resolution panel")
        .expect("resolution panel");
    let summary = root
        .query_selector("#resolution-summary")
        .expect("query resolution summary")
        .expect("resolution summary");
    let percentage = signal(0u8);

    let scope = ViewScope::attach(&panel);
    let watched = data.clone();
    let reported = percentage.clone();
    let panel_for_effect = panel.clone();
    scope.effect(move || {
        let incidents = watched.get().incidents;
        let acknowledged = incidents
            .iter()
            .filter(|incident| incident.acknowledged)
            .count();
        let share = if incidents.is_empty() {
            0
        } else {
            ((acknowledged * 100) / incidents.len()) as u8
        };
        reported.set(share);
        summary.set_text_content(Some(&format!(
            "{acknowledged} of {} incidents acknowledged",
            incidents.len()
        )));
        panel_for_effect
            .set_attribute("data-acknowledged", &acknowledged.to_string())
            .expect("expose acknowledged count");
    });

    root.query_selector("#resolution-progress")
        .expect("query resolution progress")
        .expect("resolution progress host")
        .append_child(&ProgressBar::create(ProgressProps {
            value: percentage,
            max: 100,
            show_label: true,
            variant: ProgressVariant::Linear,
            class: Some("resolution".to_string()),
            ..Default::default()
        }))
        .expect("append resolution progress");
}

fn append_statistic(host: &web_sys::Element, title: &str, value: usize, description: &str) {
    host.append_child(&statistic_card(StatisticCardProps {
        statistic: StatisticProps {
            title: Some(title.to_string()),
            value: value.to_string(),
            description: Some(description.to_string()),
            ..Default::default()
        },
        bordered: true,
        ..Default::default()
    }))
    .expect("append incident statistic");
}

#[cfg(test)]
mod tests {
    use crate::data::{monitoring_fixture, IncidentSeverity};

    #[test]
    fn fixture_drives_all_incident_indicators() {
        let data = monitoring_fixture(0xD0_51_05);
        let open = data
            .incidents
            .iter()
            .filter(|incident| !incident.acknowledged)
            .count();
        let critical = data
            .incidents
            .iter()
            .filter(|incident| incident.severity == IncidentSeverity::Critical)
            .count();
        assert_eq!(data.incidents.len(), 48);
        assert_eq!(open, 36);
        assert!(critical > 0);
    }
}
