use std::cell::RefCell;
use std::rc::Rc;

use domius_core::{create_effect, signal, Signal};
use domius_web::components::feedback::toast::ToastManager;
use domius_web::domus;
use domius_web::list::KeyedList;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlSelectElement};

use crate::data::{Incident, IncidentSeverity, MonitoringData, Service};
use crate::state::{Acknowledgement, FilterContext};

/// How the queue is sorted. Keys stay stable across every order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QueueOrder {
    #[default]
    Newest,
    Oldest,
    Severity,
}

impl QueueOrder {
    fn token(self) -> &'static str {
        match self {
            Self::Newest => "newest",
            Self::Oldest => "oldest",
            Self::Severity => "severity",
        }
    }

    fn parse(token: &str) -> Self {
        match token {
            "oldest" => Self::Oldest,
            "severity" => Self::Severity,
            _ => Self::Newest,
        }
    }
}

/// Sort a matching set for display, without touching which incidents match.
pub fn ordered(incidents: &[Incident], order: QueueOrder) -> Vec<Incident> {
    let mut ordered = incidents.to_vec();
    match order {
        QueueOrder::Newest => {
            ordered.sort_by(|left, right| left.opened_minutes_ago.cmp(&right.opened_minutes_ago))
        }
        QueueOrder::Oldest => {
            ordered.sort_by(|left, right| right.opened_minutes_ago.cmp(&left.opened_minutes_ago))
        }
        QueueOrder::Severity => ordered.sort_by(|left, right| {
            right
                .severity
                .cmp(&left.severity)
                .then_with(|| left.opened_minutes_ago.cmp(&right.opened_minutes_ago))
        }),
    }
    ordered
}

pub struct IncidentQueueProps {
    pub filters: FilterContext,
    pub data: Signal<MonitoringData>,
    pub services: Vec<Service>,
    pub toasts: ToastManager,
}

/// The operator's working queue: filter it, reorder it, acknowledge from it.
///
/// The list is keyed by incident id, so reordering and acknowledging move or
/// drop nodes rather than rebuilding the queue underneath the operator.
pub fn incident_queue(props: IncidentQueueProps) -> Element {
    let root = domus! {
        section(class: "incident-queue") {
            form(class: "incident-filters") {
                p(class: "field") {
                    label(for: "filter-severity") { "Severity" }
                    select(id: "filter-severity") { }
                }
                p(class: "field") {
                    label(for: "filter-service") { "Service" }
                    select(id: "filter-service") { }
                }
                p(class: "field") {
                    label(for: "filter-state") { "State" }
                    select(id: "filter-state") { }
                }
                p(class: "field") {
                    label(for: "queue-order") { "Order" }
                    select(id: "queue-order") { }
                }
                button(type: "button", id: "clear-filters") { "Clear filters" }
            }
            p(id: "queue-count", role: "status") { }
            ol(id: "queue-list", class: "queue-list") { }
        }
    };

    let form = root
        .query_selector("form")
        .expect("query filter form")
        .expect("filter form")
        .unchecked_into::<Element>();
    form.set_attribute("aria-label", "Incident filters")
        .expect("label filter form");

    fill_options(
        &root,
        "#filter-severity",
        &[
            ("any", "Any severity"),
            ("low", "Low"),
            ("medium", "Medium"),
            ("high", "High"),
            ("critical", "Critical"),
        ],
    );
    let mut services = vec![("any".to_string(), "Any service".to_string())];
    services.extend(
        props
            .services
            .iter()
            .map(|service| (service.id.clone(), service.name.clone())),
    );
    fill_options(
        &root,
        "#filter-service",
        &services
            .iter()
            .map(|(value, label)| (value.as_str(), label.as_str()))
            .collect::<Vec<_>>(),
    );
    fill_options(
        &root,
        "#filter-state",
        &[
            ("any", "Any state"),
            ("open", "Open"),
            ("acknowledged", "Acknowledged"),
        ],
    );
    fill_options(
        &root,
        "#queue-order",
        &[
            ("newest", "Newest first"),
            ("oldest", "Oldest first"),
            ("severity", "Severity first"),
        ],
    );

    let order = signal(QueueOrder::default());
    let list = Rc::new(RefCell::new(KeyedList::mount(
        root.query_selector("#queue-list")
            .expect("query queue list")
            .expect("queue list host"),
    )));
    let count = root
        .query_selector("#queue-count")
        .expect("query queue count")
        .expect("queue count host");

    let render_incident = {
        let data = props.data.clone();
        let toasts = props.toasts.clone();
        move |incident: &Incident| incident_card(incident, data.clone(), toasts.clone())
    };
    let render_incident = Rc::new(RefCell::new(render_incident));

    {
        let matching = props.filters.matching.clone();
        let order = order.clone();
        let list = Rc::clone(&list);
        let render_incident = Rc::clone(&render_incident);
        create_effect(move || {
            let chosen = order.get();
            let incidents = ordered(&matching.get(), chosen);
            count.set_text_content(Some(&describe_count(incidents.len())));
            count
                .set_attribute("data-count", &incidents.len().to_string())
                .expect("expose queue count");
            count
                .set_attribute("data-order", chosen.token())
                .expect("expose queue order");
            let render = render_incident.borrow_mut();
            list.borrow_mut().reconcile_with(
                &incidents,
                |incident| incident.id.clone(),
                |incident| render(incident),
                refresh_card,
            );
        });
    }

    wire_controls(&root, props.filters.clone(), order);
    root
}

fn describe_count(count: usize) -> String {
    match count {
        0 => "No incident matches these filters".to_string(),
        1 => "1 incident".to_string(),
        many => format!("{many} incidents"),
    }
}

/// Read every control at once so one change of mind is one update.
fn wire_controls(root: &Element, filters: FilterContext, order: Signal<QueueOrder>) {
    let apply = {
        let root = root.clone();
        let filters = filters.clone();
        let order = order.clone();
        Rc::new(move || {
            filters.apply(
                parse_severity(&selected(&root, "#filter-severity")),
                parse_service(&selected(&root, "#filter-service")),
                parse_state(&selected(&root, "#filter-state")),
            );
            order.set(QueueOrder::parse(&selected(&root, "#queue-order")));
        })
    };

    for selector in [
        "#filter-severity",
        "#filter-service",
        "#filter-state",
        "#queue-order",
    ] {
        let control = root
            .query_selector(selector)
            .expect("query queue control")
            .expect("queue control");
        let apply = Rc::clone(&apply);
        let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| apply());
        control
            .add_event_listener_with_callback("change", handler.as_ref().unchecked_ref())
            .expect("listen to queue control");
        handler.forget();
    }

    let reset = root
        .query_selector("#clear-filters")
        .expect("query clear filters")
        .expect("clear filters button");
    let root_for_reset = root.clone();
    let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        for selector in [
            "#filter-severity",
            "#filter-service",
            "#filter-state",
            "#queue-order",
        ] {
            select_value(
                &root_for_reset,
                selector,
                if selector == "#queue-order" {
                    "newest"
                } else {
                    "any"
                },
            );
        }
        filters.clear();
        order.set(QueueOrder::Newest);
    });
    reset
        .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        .expect("listen to clear filters");
    handler.forget();
}

fn incident_card(
    incident: &Incident,
    data: Signal<MonitoringData>,
    toasts: ToastManager,
) -> Element {
    let title = incident.title.clone();
    let opened = format!("Opened {} minutes ago", incident.opened_minutes_ago);
    let service = incident.service_id.clone();
    let card = domus! {
        li(class: "queue-item") {
            article {
                h3(class: "queue-item-title") { {title} }
                p(class: "queue-item-meta") {
                    span(class: "queue-item-service") { {service} }
                    span(class: "queue-item-age") { {opened} }
                }
                button(type: "button", class: "acknowledge") { }
            }
        }
    };
    card.set_attribute("data-key", &incident.id)
        .expect("key incident card");

    let acknowledged_title = incident.title.clone();
    let id = incident.id.clone();
    let button = card
        .query_selector(".acknowledge")
        .expect("query acknowledge button")
        .expect("acknowledge button");
    let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        let mut changed = false;
        data.update(|window| {
            if let Some(incident) = window
                .incidents
                .iter_mut()
                .find(|candidate| candidate.id == id && !candidate.acknowledged)
            {
                incident.acknowledged = true;
                changed = true;
            }
        });
        if changed {
            toasts.success(format!("{acknowledged_title} acknowledged"));
        }
    });
    button
        .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        .expect("listen to acknowledgement");
    handler.forget();

    refresh_card(&card, incident);
    card
}

/// Bring a card that survived reconciliation up to date, node identity intact.
fn refresh_card(card: &Element, incident: &Incident) {
    card.set_attribute("data-severity", severity_token(incident.severity))
        .expect("expose incident severity");
    card.set_attribute("data-acknowledged", &incident.acknowledged.to_string())
        .expect("expose incident state");
    let button = card
        .query_selector(".acknowledge")
        .expect("query acknowledge button")
        .expect("acknowledge button");
    button.set_text_content(Some(if incident.acknowledged {
        "Acknowledged"
    } else {
        "Acknowledge"
    }));
    if incident.acknowledged {
        button
            .set_attribute("disabled", "")
            .expect("disable button");
    } else {
        button.remove_attribute("disabled").expect("enable button");
    }
    button
        .set_attribute(
            "aria-label",
            &format!("Acknowledge {} ({})", incident.title, incident.id),
        )
        .expect("label acknowledge button");
}

fn severity_token(severity: IncidentSeverity) -> &'static str {
    match severity {
        IncidentSeverity::Low => "low",
        IncidentSeverity::Medium => "medium",
        IncidentSeverity::High => "high",
        IncidentSeverity::Critical => "critical",
    }
}

fn parse_severity(token: &str) -> Option<IncidentSeverity> {
    match token {
        "low" => Some(IncidentSeverity::Low),
        "medium" => Some(IncidentSeverity::Medium),
        "high" => Some(IncidentSeverity::High),
        "critical" => Some(IncidentSeverity::Critical),
        _ => None,
    }
}

fn parse_service(token: &str) -> Option<String> {
    match token {
        "any" | "" => None,
        service => Some(service.to_string()),
    }
}

fn parse_state(token: &str) -> Acknowledgement {
    match token {
        "open" => Acknowledgement::Open,
        "acknowledged" => Acknowledgement::Acknowledged,
        _ => Acknowledgement::Any,
    }
}

fn fill_options(root: &Element, selector: &str, options: &[(&str, &str)]) {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let control = root
        .query_selector(selector)
        .expect("query queue control")
        .expect("queue control");
    for (value, label) in options {
        let option = document.create_element("option").expect("create option");
        option.set_attribute("value", value).expect("value option");
        option.set_text_content(Some(label));
        control.append_child(&option).expect("append option");
    }
}

fn selected(root: &Element, selector: &str) -> String {
    root.query_selector(selector)
        .expect("query queue control")
        .and_then(|control| control.dyn_into::<HtmlSelectElement>().ok())
        .map(|control| control.value())
        .unwrap_or_default()
}

fn select_value(root: &Element, selector: &str, value: &str) {
    if let Some(control) = root
        .query_selector(selector)
        .expect("query queue control")
        .and_then(|control| control.dyn_into::<HtmlSelectElement>().ok())
    {
        control.set_value(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::monitoring_fixture;

    #[test]
    fn ordering_permutes_the_same_keys() {
        let incidents = monitoring_fixture(0xD0_51_05).incidents;
        let keys = |order| {
            ordered(&incidents, order)
                .iter()
                .map(|incident| incident.id.clone())
                .collect::<Vec<_>>()
        };
        let newest = keys(QueueOrder::Newest);
        let oldest = keys(QueueOrder::Oldest);
        let severity = keys(QueueOrder::Severity);

        assert_eq!(newest.len(), 48);
        assert_ne!(newest, oldest);
        assert_ne!(newest, severity);
        for permutation in [&oldest, &severity] {
            let mut sorted = permutation.clone();
            let mut reference = newest.clone();
            sorted.sort();
            reference.sort();
            assert_eq!(sorted, reference, "reordering must not change the key set");
        }
    }

    #[test]
    fn ordering_respects_the_chosen_axis() {
        let incidents = monitoring_fixture(0xD0_51_05).incidents;

        let newest = ordered(&incidents, QueueOrder::Newest);
        assert!(newest
            .windows(2)
            .all(|pair| pair[0].opened_minutes_ago <= pair[1].opened_minutes_ago));

        let oldest = ordered(&incidents, QueueOrder::Oldest);
        assert!(oldest
            .windows(2)
            .all(|pair| pair[0].opened_minutes_ago >= pair[1].opened_minutes_ago));

        let severity = ordered(&incidents, QueueOrder::Severity);
        assert!(severity
            .windows(2)
            .all(|pair| pair[0].severity >= pair[1].severity));
        assert_eq!(severity[0].severity, IncidentSeverity::Critical);
    }

    #[test]
    fn queue_controls_round_trip_through_their_tokens() {
        for severity in [
            IncidentSeverity::Low,
            IncidentSeverity::Medium,
            IncidentSeverity::High,
            IncidentSeverity::Critical,
        ] {
            assert_eq!(parse_severity(severity_token(severity)), Some(severity));
        }
        assert_eq!(parse_severity("any"), None);
        assert_eq!(parse_service("any"), None);
        assert_eq!(parse_service("svc-04"), Some("svc-04".to_string()));
        assert_eq!(parse_state("open"), Acknowledgement::Open);
        assert_eq!(parse_state("anything else"), Acknowledgement::Any);
        for order in [QueueOrder::Newest, QueueOrder::Oldest, QueueOrder::Severity] {
            assert_eq!(QueueOrder::parse(order.token()), order);
        }
    }

    #[test]
    fn the_count_reads_as_a_sentence() {
        assert_eq!(describe_count(0), "No incident matches these filters");
        assert_eq!(describe_count(1), "1 incident");
        assert_eq!(describe_count(12), "12 incidents");
    }
}
