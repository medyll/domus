use super::model::{Incident, IncidentSeverity, Metric, MonitoringData, Service, ServiceStatus};

const SERVICE_NAMES: [&str; 6] = [
    "Gateway",
    "Identity",
    "Billing",
    "Search",
    "Events",
    "Reporting",
];

/// Produce the same monitoring data for a given seed on every platform.
pub fn monitoring_fixture(seed: u64) -> MonitoringData {
    let mut random = DeterministicRandom::new(seed);
    let services = SERVICE_NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| Service {
            id: format!("svc-{:02}", index + 1),
            name: (*name).to_string(),
            status: match index {
                2 => ServiceStatus::Outage,
                1 | 4 => ServiceStatus::Degraded,
                _ => ServiceStatus::Operational,
            },
            latency_ms: 35 + random.range(180),
            error_rate: f64::from(random.range(700)) / 100.0,
        })
        .collect::<Vec<_>>();

    let incidents = (0..48)
        .map(|index| Incident {
            id: format!("inc-{:03}", index + 1),
            service_id: services[index % services.len()].id.clone(),
            title: format!("{} threshold breach", services[index % services.len()].name),
            severity: match random.range(10) {
                0 => IncidentSeverity::Critical,
                1..=2 => IncidentSeverity::High,
                3..=5 => IncidentSeverity::Medium,
                _ => IncidentSeverity::Low,
            },
            acknowledged: index % 4 == 0,
            opened_minutes_ago: 3 + random.range(720),
        })
        .collect();

    let mut metrics = Vec::with_capacity(services.len() * 60);
    for service in &services {
        for minute in 0..60 {
            metrics.push(Metric {
                service_id: service.id.clone(),
                minute,
                requests_per_second: 200 + random.range(2_000),
                error_rate: f64::from(random.range(500)) / 100.0,
            });
        }
    }

    MonitoringData {
        services,
        incidents,
        metrics,
    }
}

struct DeterministicRandom {
    state: u64,
}

impl DeterministicRandom {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn range(&mut self, upper_bound: u32) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.state >> 32) as u32) % upper_bound
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_is_repeatable_for_the_same_seed() {
        assert_eq!(monitoring_fixture(7), monitoring_fixture(7));
        assert_ne!(monitoring_fixture(7), monitoring_fixture(8));
    }

    #[test]
    fn fixture_exercises_filtering_pagination_and_charts() {
        let data = monitoring_fixture(7);

        assert_eq!(data.services.len(), 6);
        assert_eq!(data.incidents.len(), 48);
        assert_eq!(data.metrics.len(), 360);
        assert!(data.open_incident_count() > 24);
        assert!(data
            .incidents
            .iter()
            .all(|incident| data.service(&incident.service_id).is_some()));
    }
}
