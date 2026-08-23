use domius_core::signal::{signal, Signal};
use domius_web::context::{has_context, provide_context, use_context};

use crate::data::{monitoring_fixture, MonitoringData};

#[derive(Clone)]
pub struct MonitoringContext {
    pub data: Signal<MonitoringData>,
}

impl MonitoringContext {
    pub fn seeded(seed: u64) -> Self {
        Self {
            data: signal(monitoring_fixture(seed)),
        }
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
    use super::*;
    use domius_web::context::{clear_all_contexts, remove_context};

    #[test]
    fn monitoring_data_round_trips_through_context() {
        clear_all_contexts();
        MonitoringContext::seeded(21).provide();

        assert!(MonitoringContext::is_available());
        assert_eq!(
            MonitoringContext::current()
                .unwrap()
                .data
                .get()
                .services
                .len(),
            6
        );

        remove_context::<MonitoringContext>();
        assert!(!MonitoringContext::is_available());
    }
}
