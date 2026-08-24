use std::cell::Cell;
use std::rc::Rc;

use domius_core::signal::signal;
use domius_web::components::data::badge::{Badge, BadgeProps, BadgeVariant};
use domius_web::components::feedback::infinite_scroll::{InfiniteScroll, InfiniteScrollProps};
use web_sys::Element;

use crate::components::incident_history::severity_label;
use crate::data::{Incident, IncidentSeverity};

pub struct IncidentFeedProps {
    pub incidents: Vec<Incident>,
    pub batch_size: usize,
}

pub fn incident_feed(props: IncidentFeedProps) -> Element {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let list = document.create_element("ol").expect("create incident feed");
    list.set_class_name("list");
    list.set_attribute("aria-label", "Progressive incident feed")
        .expect("label incident feed");

    let incidents = Rc::new(props.incidents);
    let batch_size = props.batch_size.max(1);
    let initial_count = batch_size.min(incidents.len());
    append_incidents(&list, &incidents[..initial_count]);

    let loaded = Rc::new(Cell::new(initial_count));
    let loading = signal(false);
    let has_more = signal(initial_count < incidents.len());
    let callback_list = list.clone();
    let callback_incidents = Rc::clone(&incidents);
    let callback_loaded = Rc::clone(&loaded);
    let callback_loading = loading.clone();
    let callback_has_more = has_more.clone();
    InfiniteScroll::create(InfiniteScrollProps {
        children: list,
        has_more,
        loading,
        threshold: 160,
        on_load_more: Box::new(move || {
            let start = callback_loaded.get();
            let end = (start + batch_size).min(callback_incidents.len());
            append_incidents(&callback_list, &callback_incidents[start..end]);
            callback_loaded.set(end);
            callback_has_more.set(end < callback_incidents.len());
            callback_loading.set(false);
        }),
        ..Default::default()
    })
}

fn append_incidents(list: &Element, incidents: &[Incident]) {
    let document = list.owner_document().expect("incident feed document");
    for incident in incidents {
        let item = document.create_element("li").expect("create incident item");
        item.set_class_name("list-item");
        item.set_attribute("data-key", &incident.id)
            .expect("set incident key");
        let article = document
            .create_element("article")
            .expect("create incident article");
        let title = document
            .create_element("h3")
            .expect("create incident title");
        title.set_text_content(Some(&incident.title));
        article.append_child(&title).expect("append incident title");
        let metadata = document
            .create_element("p")
            .expect("create incident metadata");
        metadata.set_text_content(Some(&format!(
            "{} · {} minutes ago · {}",
            incident.service_id,
            incident.opened_minutes_ago,
            if incident.acknowledged {
                "acknowledged"
            } else {
                "open"
            }
        )));
        article
            .append_child(&metadata)
            .expect("append incident metadata");
        article
            .append_child(&Badge::create(BadgeProps {
                children: severity_label(incident.severity).to_string(),
                variant: severity_variant(incident.severity),
                dot: true,
                ..Default::default()
            }))
            .expect("append incident severity");
        item.append_child(&article)
            .expect("append incident article");
        list.append_child(&item).expect("append incident item");
    }
}

fn severity_variant(severity: IncidentSeverity) -> BadgeVariant {
    match severity {
        IncidentSeverity::Low => BadgeVariant::Neutral,
        IncidentSeverity::Medium => BadgeVariant::Info,
        IncidentSeverity::High => BadgeVariant::Warning,
        IncidentSeverity::Critical => BadgeVariant::Error,
    }
}

#[cfg(test)]
mod tests {
    use crate::data::monitoring_fixture;

    #[test]
    fn feed_fixture_spans_multiple_complete_batches() {
        let incidents = monitoring_fixture(7).incidents;
        assert_eq!(incidents.len(), 48);
        assert_eq!(incidents.chunks(10).count(), 5);
        assert_eq!(incidents.chunks(10).last().unwrap().len(), 8);
    }
}
