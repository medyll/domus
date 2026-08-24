use domius_web::components::data::badge::{Badge, BadgeProps, BadgeVariant};
use domius_web::components::data::timeline::{Timeline, TimelineEvent, TimelineProps};
use domius_web::components::feedback::tooltip::{Tooltip, TooltipPosition, TooltipProps};
use domius_web::components::navigation::anchor::{anchor, AnchorLink, AnchorProps};
use domius_web::components::navigation::breadcrumbs::{
    BreadcrumbItem, Breadcrumbs, BreadcrumbsProps,
};
use domius_web::components::primitives::affix::{affix, AffixProps};
use domius_web::components::primitives::backtop::{backtop, BackTopProps};
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
                    p(id: "service-summary") { }
                    span(id: "service-status") { }
                }
                div(id: "service-contents") { }
                section(id: "service-timeline-section", class: "panel panel-bordered") {
                    h2(tabindex: "-1") { "Recent incident timeline" }
                    div(id: "service-timeline") { }
                }
                section(id: "service-incidents-section", class: "panel panel-bordered") {
                    h2(tabindex: "-1") { "Incident history" }
                    div(id: "service-incidents") { }
                }
                div(id: "service-backtop") { }
            }
        };

        append_breadcrumbs(&root, service);
        append_status(&root, service.status);
        append_summary(&root, &service_summary, service);
        append_contents(&root);
        append_timeline(&root, &state.incidents);
        append_incident_history(&root, &state.incidents);
        append_backtop(&root);
        root
    }
}

/// A table of contents that sticks once the reader scrolls past the header.
fn append_contents(root: &Element) {
    let sticky = affix(AffixProps {
        offset_top: 96,
        class: Some("service-contents".to_string()),
        ..Default::default()
    });
    sticky
        .append_child(&anchor(AnchorProps {
            links: vec![
                AnchorLink {
                    href: "#service-timeline-section".to_string(),
                    title: "Timeline".to_string(),
                },
                AnchorLink {
                    href: "#service-incidents-section".to_string(),
                    title: "Incident history".to_string(),
                },
            ],
            offset_top: 96,
            show_boundary: true,
            class: Some("service-anchor".to_string()),
            ..Default::default()
        }))
        .expect("append service anchor");
    root.query_selector("#service-contents")
        .expect("query service contents")
        .expect("service contents host")
        .append_child(&sticky)
        .expect("append service contents");
}

/// The summary abbreviates two figures, so spell them out within reach.
fn append_summary(root: &Element, summary: &str, service: &Service) {
    let host = root
        .query_selector("#service-summary")
        .expect("query service summary")
        .expect("service summary host");
    let value = host
        .owner_document()
        .expect("service summary document")
        .create_element("span")
        .expect("create service summary value");
    value.set_text_content(Some(summary));
    host.append_child(&Tooltip::create(TooltipProps {
        content: format!(
            "Median response time {} milliseconds, {:.2} percent of requests failing",
            service.latency_ms, service.error_rate
        ),
        position: TooltipPosition::Bottom,
        children: value,
        class: Some("service-summary".to_string()),
        ..Default::default()
    }))
    .expect("append service summary tooltip");
}

fn append_backtop(root: &Element) {
    root.query_selector("#service-backtop")
        .expect("query service backtop")
        .expect("service backtop host")
        .append_child(&backtop(BackTopProps {
            visibility_height: 320,
            class: Some("service-backtop".to_string()),
            ..Default::default()
        }))
        .expect("append service backtop");
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
