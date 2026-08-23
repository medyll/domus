use std::collections::HashMap;
use std::rc::Rc;

use domius_web::components::data::badge::{Badge, BadgeProps, BadgeVariant};
use domius_web::components::data::table::{
    Column, ColumnAlign, DataTable, DataTableProps, RowData,
};
use domius_web::components::data::timeline::{Timeline, TimelineEvent, TimelineProps};
use domius_web::components::navigation::breadcrumbs::{
    BreadcrumbItem, Breadcrumbs, BreadcrumbsProps,
};
use domius_web::components::navigation::pagination::{Pagination, PaginationProps};
use domius_web::{domus, DomiusComponent, DomiusNode, DomiusPage};
use web_sys::Element;

use crate::data::{Incident, IncidentSeverity, MonitoringData, Service, ServiceStatus};
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
                    div(id: "service-pagination") { }
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
    let host = root
        .query_selector("#service-incidents")
        .expect("query incidents")
        .expect("incidents host");
    let incident_data = Rc::new(incidents.to_vec());
    render_incident_page(&host, incident_data.as_ref(), 1);

    let callback_host = host.clone();
    let callback_data = Rc::clone(&incident_data);
    let (pagination, _) = Pagination::create(PaginationProps {
        total_items: incident_data.len(),
        page_size: INCIDENTS_PER_PAGE,
        on_page_change: Some(Box::new(move |page| {
            render_incident_page(&callback_host, callback_data.as_ref(), page)
        })),
        ..Default::default()
    });
    root.query_selector("#service-pagination")
        .expect("query pagination")
        .expect("pagination host")
        .append_child(&pagination)
        .expect("append pagination");
}

fn render_incident_page(host: &Element, incidents: &[Incident], page: usize) {
    host.set_text_content(None);
    let start = page.saturating_sub(1) * INCIDENTS_PER_PAGE;
    let rows = incidents
        .iter()
        .skip(start)
        .take(INCIDENTS_PER_PAGE)
        .map(incident_row)
        .collect();
    let table = DataTable::create(DataTableProps {
        columns: incident_columns(),
        data: rows,
        filterable: true,
        selectable: true,
        ..Default::default()
    });
    host.append_child(&table).expect("append incident table");
}

fn incident_columns() -> Vec<Column> {
    vec![
        column("id", "Incident", true, true, ColumnAlign::Left),
        column("severity", "Severity", true, true, ColumnAlign::Center),
        column("status", "Status", true, true, ColumnAlign::Center),
        column("age", "Age", true, false, ColumnAlign::Right),
    ]
}

fn column(
    field: &str,
    header: &str,
    sortable: bool,
    filterable: bool,
    align: ColumnAlign,
) -> Column {
    Column {
        field: field.into(),
        header: header.into(),
        sortable,
        filterable,
        width: None,
        align,
    }
}

fn incident_row(incident: &Incident) -> RowData {
    HashMap::from([
        ("id".into(), incident.id.clone()),
        ("severity".into(), severity_label(incident.severity).into()),
        (
            "status".into(),
            if incident.acknowledged {
                "Acknowledged".into()
            } else {
                "Open".into()
            },
        ),
        ("age".into(), format!("{} min", incident.opened_minutes_ago)),
    ])
}

fn severity_label(severity: IncidentSeverity) -> &'static str {
    match severity {
        IncidentSeverity::Low => "Low",
        IncidentSeverity::Medium => "Medium",
        IncidentSeverity::High => "High",
        IncidentSeverity::Critical => "Critical",
    }
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
    fn incident_rows_cover_every_service_page() {
        let data = monitoring_fixture(7);
        let incidents: Vec<_> = data
            .incidents
            .iter()
            .filter(|incident| incident.service_id == "svc-03")
            .collect();
        assert_eq!(incidents.len(), 8);
        let rows: Vec<_> = incidents
            .iter()
            .map(|incident| incident_row(incident))
            .collect();
        assert_eq!(rows.len(), 8);
        assert!(rows.iter().all(|row| row.contains_key("severity")));
    }
}
