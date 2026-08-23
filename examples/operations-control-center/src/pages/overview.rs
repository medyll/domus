use domius_core::computed::{computed, Computed};
use domius_core::signal::Signal;
use domius_web::components::data::badge::{Badge, BadgeProps, BadgeVariant};
use domius_web::components::data::statistic::{statistic_card, StatisticCardProps, StatisticProps};
use domius_web::components::primitives::card::{card, card_body, CardProps};
use domius_web::{domus, DomiusComponent, DomiusNode, DomiusPage};

use crate::data::{MonitoringData, ServiceStatus};
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
                section(id: "overview-statistics") { }
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
