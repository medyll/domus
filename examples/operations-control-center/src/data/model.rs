#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceStatus {
    Operational,
    Degraded,
    Outage,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub status: ServiceStatus,
    pub latency_ms: u32,
    pub error_rate: f64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Incident {
    pub id: String,
    pub service_id: String,
    pub title: String,
    pub severity: IncidentSeverity,
    pub acknowledged: bool,
    pub opened_minutes_ago: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Metric {
    pub service_id: String,
    pub minute: u32,
    pub requests_per_second: u32,
    pub error_rate: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MonitoringData {
    pub services: Vec<Service>,
    pub incidents: Vec<Incident>,
    pub metrics: Vec<Metric>,
}

impl MonitoringData {
    pub fn service(&self, id: &str) -> Option<&Service> {
        self.services.iter().find(|service| service.id == id)
    }

    pub fn open_incident_count(&self) -> usize {
        self.incidents
            .iter()
            .filter(|incident| !incident.acknowledged)
            .count()
    }
}
