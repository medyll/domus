use domius_core::{batch, computed, signal, Computed, Signal};
use domius_web::context::{has_context, provide_context, use_context};

use crate::data::{Incident, IncidentSeverity, MonitoringData};

/// Which side of the acknowledgement line an operator wants to look at.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Acknowledgement {
    #[default]
    Any,
    Open,
    Acknowledged,
}

impl Acknowledgement {
    pub fn token(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Open => "open",
            Self::Acknowledged => "acknowledged",
        }
    }
}

/// Does this incident survive the three filters?
pub fn matches(
    incident: &Incident,
    severity: Option<IncidentSeverity>,
    service: Option<&str>,
    acknowledgement: Acknowledgement,
) -> bool {
    if let Some(severity) = severity {
        if incident.severity != severity {
            return false;
        }
    }
    if let Some(service) = service {
        if incident.service_id != service {
            return false;
        }
    }
    match acknowledgement {
        Acknowledgement::Any => true,
        Acknowledgement::Open => !incident.acknowledged,
        Acknowledgement::Acknowledged => incident.acknowledged,
    }
}

/// The incident filters shared by every view, and the values derived from them.
///
/// Views read `matching` rather than filtering for themselves, so the table,
/// the feed and the counters can never disagree about what is on screen.
#[derive(Clone)]
pub struct FilterContext {
    pub severity: Signal<Option<IncidentSeverity>>,
    pub service: Signal<Option<String>>,
    pub acknowledgement: Signal<Acknowledgement>,
    pub matching: Computed<Vec<Incident>>,
    pub matching_count: Computed<usize>,
}

impl FilterContext {
    /// Derive filters over a monitoring window.
    pub fn over(data: Signal<MonitoringData>) -> Self {
        let severity = signal(None::<IncidentSeverity>);
        let service = signal(None::<String>);
        let acknowledgement = signal(Acknowledgement::default());

        let (source, chosen_severity, chosen_service, chosen_acknowledgement) = (
            data,
            severity.clone(),
            service.clone(),
            acknowledgement.clone(),
        );
        let matching = computed(move || {
            let severity = chosen_severity.get();
            let service = chosen_service.get();
            let acknowledgement = chosen_acknowledgement.get();
            source
                .get()
                .incidents
                .into_iter()
                .filter(|incident| matches(incident, severity, service.as_deref(), acknowledgement))
                .collect::<Vec<_>>()
        });

        let counted = matching.clone();
        let matching_count = computed(move || counted.get().len());
        Self {
            severity,
            service,
            acknowledgement,
            matching,
            matching_count,
        }
    }

    /// Apply every filter at once, so dependants recompute a single time.
    pub fn apply(
        &self,
        severity: Option<IncidentSeverity>,
        service: Option<String>,
        acknowledgement: Acknowledgement,
    ) {
        batch(|| {
            self.severity.set(severity);
            self.service.set(service);
            self.acknowledgement.set(acknowledgement);
        });
    }

    /// Drop every filter in one update.
    pub fn clear(&self) {
        self.apply(None, None, Acknowledgement::Any);
    }

    /// Keys of the matching incidents, in display order.
    pub fn matching_keys(&self) -> Vec<String> {
        self.matching
            .get()
            .iter()
            .map(|incident| incident.id.clone())
            .collect()
    }

    pub fn provide(self) {
        provide_context(self);
    }

    pub fn current() -> Option<Self> {
        use_context::<Self>()
    }

    pub fn is_available() -> bool {
        has_context::<Self>()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use domius_core::create_effect;

    use super::*;
    use crate::data::monitoring_fixture;

    fn filters() -> FilterContext {
        FilterContext::over(signal(monitoring_fixture(0xD0_51_05)))
    }

    #[test]
    fn filters_narrow_the_shared_incident_set() {
        let filters = filters();
        assert_eq!(filters.matching_count.get(), 48);

        filters.apply(None, None, Acknowledgement::Open);
        assert_eq!(filters.matching_count.get(), 36);
        assert!(filters
            .matching
            .get()
            .iter()
            .all(|incident| !incident.acknowledged));

        filters.apply(Some(IncidentSeverity::Critical), None, Acknowledgement::Any);
        let critical = filters.matching.get();
        assert!(!critical.is_empty());
        assert!(critical
            .iter()
            .all(|incident| incident.severity == IncidentSeverity::Critical));

        filters.apply(None, Some("svc-01".to_string()), Acknowledgement::Any);
        assert!(filters
            .matching
            .get()
            .iter()
            .all(|incident| incident.service_id == "svc-01"));

        filters.clear();
        assert_eq!(filters.matching_count.get(), 48);
    }

    #[test]
    fn applying_three_filters_in_one_batch_updates_dependants_once() {
        let filters = filters();
        let observed = Rc::new(RefCell::new(Vec::new()));

        let counted = filters.matching_count.clone();
        let recorded = Rc::clone(&observed);
        create_effect(move || recorded.borrow_mut().push(counted.get()));

        // The effect runs once on creation, before any filter is applied.
        assert_eq!(*observed.borrow(), vec![48]);

        filters.apply(
            Some(IncidentSeverity::Low),
            Some("svc-02".to_string()),
            Acknowledgement::Open,
        );

        let observed = observed.borrow();
        assert_eq!(
            observed.len(),
            2,
            "three signals in one batch should produce one observable update, got {observed:?}"
        );
        assert_eq!(observed[1], filters.matching_count.get());
        assert!(observed[1] < 48);
    }

    #[test]
    fn setting_the_same_filters_one_by_one_updates_more_than_once() {
        let filters = filters();
        let observed = Rc::new(RefCell::new(0usize));

        let counted = filters.matching_count.clone();
        let recorded = Rc::clone(&observed);
        create_effect(move || {
            counted.get();
            *recorded.borrow_mut() += 1;
        });

        filters.severity.set(Some(IncidentSeverity::Low));
        filters.service.set(Some("svc-02".to_string()));
        filters.acknowledgement.set(Acknowledgement::Open);

        // This is the cost the batch above avoids.
        assert!(
            *observed.borrow() > 2,
            "unbatched writes should be observed more than once"
        );
    }

    #[test]
    fn matching_keys_follow_the_filtered_set() {
        let filters = filters();
        filters.apply(None, Some("svc-03".to_string()), Acknowledgement::Any);

        let keys = filters.matching_keys();
        assert_eq!(keys.len(), filters.matching_count.get());
        assert!(keys.iter().all(|key| key.starts_with("inc-")));
        assert_eq!(
            keys,
            filters
                .matching
                .get()
                .iter()
                .map(|incident| incident.id.clone())
                .collect::<Vec<_>>()
        );
    }
}
