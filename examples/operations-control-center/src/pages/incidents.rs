use domius_web::components::data::statistic::{statistic_card, StatisticCardProps, StatisticProps};
use domius_web::{domus, DomiusComponent, DomiusNode, DomiusPage};

use crate::components::{incident_feed, incident_history, IncidentFeedProps, IncidentHistoryProps};
use crate::data::{Incident, IncidentSeverity};
use crate::state::MonitoringContext;

const INCIDENTS_PER_PAGE: usize = 10;

pub struct IncidentsPage;

pub struct IncidentsState {
    incidents: Vec<Incident>,
    open_count: usize,
    critical_count: usize,
    acknowledged_count: usize,
}

impl DomiusComponent for IncidentsPage {
    type Props = ();
    type State = IncidentsState;

    fn setup(_: Self::Props) -> Self::State {
        let incidents = MonitoringContext::current()
            .expect("monitoring context is missing")
            .data
            .get()
            .incidents;
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
