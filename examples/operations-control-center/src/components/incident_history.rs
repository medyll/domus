use std::collections::HashMap;
use std::rc::Rc;

use domius_web::components::data::table::{
    Column, ColumnAlign, DataTable, DataTableProps, RowData,
};
use domius_web::components::navigation::pagination::{Pagination, PaginationProps};
use web_sys::Element;

use crate::data::{Incident, IncidentSeverity};

pub struct IncidentHistoryProps {
    pub incidents: Vec<Incident>,
    pub page_size: usize,
    pub filterable: bool,
    pub selectable: bool,
}

pub fn incident_history(props: IncidentHistoryProps) -> Element {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let root = document
        .create_element("section")
        .expect("create incident history");
    root.set_class_name("incident-history");
    let table_host = document
        .create_element("div")
        .expect("create incident table host");
    table_host.set_class_name("incident-history-table");
    let pagination_host = document
        .create_element("div")
        .expect("create incident pagination host");
    pagination_host.set_class_name("incident-history-pagination");
    root.append_child(&table_host).expect("append table host");
    root.append_child(&pagination_host)
        .expect("append pagination host");

    let incidents = Rc::new(props.incidents);
    let page_size = props.page_size.max(1);
    render_page(
        &table_host,
        incidents.as_ref(),
        1,
        page_size,
        props.filterable,
        props.selectable,
    );

    let callback_host = table_host.clone();
    let callback_incidents = Rc::clone(&incidents);
    let (pagination, _) = Pagination::create(PaginationProps {
        total_items: incidents.len(),
        page_size,
        on_page_change: Some(Box::new(move |page| {
            render_page(
                &callback_host,
                callback_incidents.as_ref(),
                page,
                page_size,
                props.filterable,
                props.selectable,
            );
        })),
        ..Default::default()
    });
    pagination_host
        .append_child(&pagination)
        .expect("append incident pagination");
    root
}

fn render_page(
    host: &Element,
    incidents: &[Incident],
    page: usize,
    page_size: usize,
    filterable: bool,
    selectable: bool,
) {
    host.set_text_content(None);
    let start = page.saturating_sub(1) * page_size;
    let rows = incidents
        .iter()
        .skip(start)
        .take(page_size)
        .map(incident_row)
        .collect();
    let table = DataTable::create(DataTableProps {
        columns: incident_columns(),
        data: rows,
        filterable,
        selectable,
        ..Default::default()
    });
    host.append_child(&table).expect("append incident table");
}

fn incident_columns() -> Vec<Column> {
    vec![
        column("id", "Incident", true, true, ColumnAlign::Left),
        column("service", "Service", true, true, ColumnAlign::Left),
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
        ("service".into(), incident.service_id.clone()),
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

pub fn severity_label(severity: IncidentSeverity) -> &'static str {
    match severity {
        IncidentSeverity::Low => "Low",
        IncidentSeverity::Medium => "Medium",
        IncidentSeverity::High => "High",
        IncidentSeverity::Critical => "Critical",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::monitoring_fixture;

    #[test]
    fn rows_expose_fields_used_by_sorting_and_filters() {
        let incident = monitoring_fixture(7).incidents.remove(0);
        let row = incident_row(&incident);
        assert_eq!(row.get("id"), Some(&incident.id));
        assert_eq!(row.get("service"), Some(&incident.service_id));
        assert!(row.contains_key("severity"));
        assert!(row.contains_key("status"));
    }
}
