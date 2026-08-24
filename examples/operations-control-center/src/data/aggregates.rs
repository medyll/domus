//! Aggregations over a monitoring window, shared by the pages that show them.
//!
//! These return plain numbers rather than view types, so a page decides how to
//! draw them and two pages cannot drift apart on what a figure means.

use std::collections::BTreeMap;

use super::model::{Metric, Service};

/// Fleet throughput per minute of the window, in requests per second.
pub fn throughput_by_minute(metrics: &[Metric]) -> Vec<(u32, u32)> {
    let mut totals = BTreeMap::<u32, u32>::new();
    for metric in metrics {
        *totals.entry(metric.minute).or_default() += metric.requests_per_second;
    }
    totals.into_iter().collect()
}

/// Average error rate over the whole window, per service, in display order.
pub fn error_rate_by_service(services: &[Service], metrics: &[Metric]) -> Vec<(String, f64)> {
    services
        .iter()
        .map(|service| {
            let sample = metrics
                .iter()
                .filter(|metric| metric.service_id == service.id)
                .collect::<Vec<_>>();
            let average = sample.iter().map(|metric| metric.error_rate).sum::<f64>()
                / sample.len().max(1) as f64;
            (service.name.clone(), average)
        })
        .collect()
}

/// One averaged cell per service and window of `minutes`.
///
/// The tuple is `(window index, service index, average)`, which is the shape a
/// heatmap and a scatter plot both want.
pub fn error_rate_windows(
    services: &[Service],
    metrics: &[Metric],
    minutes: u32,
) -> Vec<(usize, usize, f64)> {
    windows(services, metrics, minutes, |sample| {
        sample.iter().map(|metric| metric.error_rate).sum::<f64>() / sample.len().max(1) as f64
    })
}

/// Average throughput per service and window of `minutes`, same shape as above.
pub fn throughput_windows(
    services: &[Service],
    metrics: &[Metric],
    minutes: u32,
) -> Vec<(usize, usize, f64)> {
    windows(services, metrics, minutes, |sample| {
        sample
            .iter()
            .map(|metric| f64::from(metric.requests_per_second))
            .sum::<f64>()
            / sample.len().max(1) as f64
    })
}

/// How many windows of `minutes` the fixture spans.
pub fn window_count(metrics: &[Metric], minutes: u32) -> usize {
    let minutes = minutes.max(1);
    metrics
        .iter()
        .map(|metric| metric.minute / minutes)
        .max()
        .map_or(0, |last| last as usize + 1)
}

/// Label a window by the minutes it covers, e.g. `00-09 min`.
pub fn window_label(window: usize, minutes: u32) -> String {
    let start = window as u32 * minutes;
    format!("{:02}-{:02} min", start, start + minutes - 1)
}

fn windows<F>(
    services: &[Service],
    metrics: &[Metric],
    minutes: u32,
    summarise: F,
) -> Vec<(usize, usize, f64)>
where
    F: Fn(&[&Metric]) -> f64,
{
    let minutes = minutes.max(1);
    let count = window_count(metrics, minutes);
    services
        .iter()
        .enumerate()
        .flat_map(|(y, service)| {
            let summarise = &summarise;
            (0..count).map(move |x| {
                let sample = metrics
                    .iter()
                    .filter(|metric| {
                        metric.service_id == service.id && metric.minute / minutes == x as u32
                    })
                    .collect::<Vec<_>>();
                (x, y, summarise(&sample))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::monitoring_fixture;
    use super::*;

    #[test]
    fn throughput_covers_every_minute_of_the_window() {
        let data = monitoring_fixture(7);
        let throughput = throughput_by_minute(&data.metrics);

        assert_eq!(throughput.len(), 60);
        assert_eq!(throughput.first().map(|point| point.0), Some(0));
        assert_eq!(throughput.last().map(|point| point.0), Some(59));
        assert!(throughput.iter().all(|point| point.1 > 0));
    }

    #[test]
    fn error_rates_are_reported_once_per_service() {
        let data = monitoring_fixture(7);
        let rates = error_rate_by_service(&data.services, &data.metrics);

        assert_eq!(rates.len(), 6);
        assert_eq!(
            rates.iter().map(|rate| rate.0.clone()).collect::<Vec<_>>(),
            data.services
                .iter()
                .map(|service| service.name.clone())
                .collect::<Vec<_>>()
        );
        assert!(rates.iter().all(|rate| rate.1 >= 0.0));
    }

    #[test]
    fn windows_tile_the_hour_without_gaps() {
        let data = monitoring_fixture(7);

        assert_eq!(window_count(&data.metrics, 10), 6);
        assert_eq!(window_count(&data.metrics, 20), 3);
        assert_eq!(window_count(&[], 10), 0);

        let cells = error_rate_windows(&data.services, &data.metrics, 10);
        assert_eq!(cells.len(), 36);
        assert_eq!(
            cells
                .iter()
                .map(|cell| (cell.0, cell.1))
                .collect::<Vec<_>>()
                .len(),
            36
        );
        assert!(cells.iter().all(|cell| cell.2 >= 0.0));

        let throughput = throughput_windows(&data.services, &data.metrics, 10);
        assert_eq!(throughput.len(), 36);
        assert!(throughput.iter().all(|cell| cell.2 > 0.0));
    }

    #[test]
    fn window_labels_read_as_ranges() {
        assert_eq!(window_label(0, 10), "00-09 min");
        assert_eq!(window_label(5, 10), "50-59 min");
        assert_eq!(window_label(2, 20), "40-59 min");
    }
}
