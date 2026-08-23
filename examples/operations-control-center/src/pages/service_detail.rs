use domius_web::components::data::badge::{Badge, BadgeProps, BadgeVariant};
use domius_web::components::data::timeline::{Timeline, TimelineEvent, TimelineProps};
use domius_web::components::navigation::breadcrumbs::{
    BreadcrumbItem, Breadcrumbs, BreadcrumbsProps,
};
use domius_web::{domus, DomiusComponent, DomiusNode, DomiusPage};
use web_sys::Element;

use crate::components::incident_history::severity_label;
use crate::components::{incident_history, IncidentHistoryProps};
use crate::data::{Incident, MonitoringData, Service, ServiceStatus};
use crate::state::MonitoringContext;

const INCIDENTS_PER_PAGE: usize = 5;

pub struct ServiceDetailPage;

pub struct ServiceDetailProps {
    pub service_id: String,
}

pub struct ServiceDetailState {
    service_id: String,
    service: Option<Service>,
    incidents: Vec<Incident>,
}

impl DomiusComponent for ServiceDetailPage {
    type Props = ServiceDetailProps;
    type State = ServiceDetailState;

    fn setup(props: Self::Props) -> Self::State {
        let data: MonitoringData = MonitoringContext::current()
            .expect("monitoring context is missing")
            .data
            .get();
        let service = data.service(&props.service_id).cloned();
        let incidents = data
            .incidents
            .into_iter()
            .filter(|incident| incident.service_id == props.service_id)
            .collect();
        ServiceDetailState {
            service_id: props.service_id,
            service,
            incidents,
        }
    }

    fn render(state: &Self::State) -> DomiusNode {
        let Some(service) = state.service.as_ref() else {
            let missing_service = format!("No service matches {}", state.service_id);
            return domus! {
                main(class: "service-detail") {
                    h1 { "Service not found" }
                    p { {missing_service} }
                    a(href: "/overview") { "Return to overview" }
                }
            };
        };
        let service_name = service.name.clone();
        let service_summary = format!(
            "{} ms latency · {:.2}% error rate",
            service.latency_ms, service.error_rate
        );

        let root = domus! {
            main(class: "service-detail") {
                section(id: "service-breadcrumbs") { }
                header(class: "section-header") {
                    h1 { {service_name} }
                    p { {service_summary} }
                    span(id: "service-status") { }
                }
                section(class: "panel panel-bordered") {
                    h2 { "Recent incident timeline" }
                    div(id: "service-timeline") { }
                }
                section(class: "panel panel-bordered") {
                    h2 { "Incident history" }
                    div(id: "service-incidents") { }
                }
            }
        };

        append_breadcrumbs(&root, service);
        append_status(&root, service.status);
        append_timeline(&root, &state.incidents);
        append_incident_history(&root, &state.incidents);
        root
    }
}

impl DomiusPage for ServiceDetailPage {
    fn route() -> &'static str {
        "/services/:id"
    }

    fn title(state: &Self::State) -> String {
        state.service.as_ref().map_or_else(
            || "Service not found | Domius".to_string(),
            |service| format!("{} service | Domius", service.name),
        )
    }
}

fn append_breadcrumbs(root: &Element, service: &Service) {
    let breadcrumbs = Breadcrumbs::create(BreadcrumbsProps {
        items: vec![
            BreadcrumbItem {
                label: "Overview".into(),
                href: Some("/overview".into()),
                disabled: false,
            },
            BreadcrumbItem {
                label: "Services".into(),
                href: Some("/overview#service-health".into()),
                disabled: false,
            },
            BreadcrumbItem {
                label: service.name.clone(),
                href: None,
                disabled: false,
            },
        ],
        ..Default::default()
    });
    root.query_selector("#service-breadcrumbs")
        .expect("query breadcrumbs")
        .expect("breadcrumbs host")
        .append_child(&breadcrumbs)
        .expect("append breadcrumbs");
}

fn append_status(root: &Element, status: ServiceStatus) {
    let badge = Badge::create(BadgeProps {
        children: status_label(status).into(),
        variant: status_variant(status),
        dot: true,
        ..Default::default()
    });
    root.query_selector("#service-status")
        .expect("query status")
        .expect("status host")
        .append_child(&badge)
        .expect("append service status");
}

fn append_timeline(root: &Element, incidents: &[Incident]) {
    let events = incidents
        .iter()
        .take(5)
        .map(|incident| TimelineEvent {
            title: incident.title.clone(),
            description: Some(format!(
                "{} severity · {}",
                severity_label(incident.severity),
                if incident.acknowledged {
                    "acknowledged"
                } else {
                    "open"
                }
            )),
            timestamp: Some(format!("{} minutes ago", incident.opened_minutes_ago)),
            icon: None,
            color: Some(severity_label(incident.severity).to_lowercase()),
        })
        .collect();
    let timeline = Timeline::create(TimelineProps {
        events,
        ..Default::default()
    });
    root.query_selector("#service-timeline")
        .expect("query timeline")
        .expect("timeline host")
        .append_child(&timeline)
        .expect("append timeline");
}

fn append_incident_history(root: &Element, incidents: &[Incident]) {
    let history = incident_history(IncidentHistoryProps {
        incidents: incidents.to_vec(),
        page_size: INCIDENTS_PER_PAGE,
        filterable: true,
        selectable: true,
    });
    root.query_selector("#service-incidents")
        .expect("query incidents")
        .expect("incidents host")
        .append_child(&history)
        .expect("append incident history");
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
    use crate::data::monitoring_fixture;

    #[test]
    fn incident_rows_cover_every_service_page() {
        let data = monitoring_fixture(7);
        let incidents: Vec<_> = data
            .incidents
            .iter()
            .filter(|incident| incident.service_id == "svc-03")
            .collect();
        assert_eq!(incidents.len(), 8);
    }
}
