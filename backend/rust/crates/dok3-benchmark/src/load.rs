//! Concurrent search load with per-worker warmup and nearest-rank latency percentiles.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Barrier, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use dok3_core::Dok3Error;
use dok3_domain::{Principal, SearchRequest};
use dok3_search::SearchService;
use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub struct LoadConfig {
    pub threads: usize,
    pub iterations_per_thread: usize,
    /// Number of untimed requests per worker.
    pub warmup_iterations: usize,
}

#[derive(Debug, Default, Serialize)]
pub struct LoadReport {
    pub total_requests: usize,
    pub error_count: usize,
    pub elapsed_ms: f64,
    pub qps: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub max_ms: f64,
}

/// Repeats the same request using the supplied principal on every worker.
/// Timed attempts, including errors, contribute to throughput and latency.
pub fn run_load(
    service: Arc<SearchService>,
    principal: &Principal,
    request: &SearchRequest,
    config: LoadConfig,
) -> Result<LoadReport, Dok3Error> {
    if config.threads == 0 {
        return Err(Dok3Error::InvalidInput("threads must be positive".into()));
    }
    let total_requests = config
        .threads
        .checked_mul(config.iterations_per_thread)
        .ok_or_else(|| Dok3Error::InvalidInput("total requests exceed usize".into()))?;
    if total_requests == 0 {
        return Ok(LoadReport::default());
    }

    let launch = OnceLock::<bool>::new();
    let warmed_up = Barrier::new(config.threads);
    let (mut latencies, error_count, elapsed) = thread::scope(|scope| {
        let mut workers = Vec::new();
        for _ in 0..config.threads {
            let worker = thread::Builder::new().spawn_scoped(scope, || {
                if !*launch.wait() {
                    return None;
                }
                // A panicking search must still release the other workers.
                let warmup = catch_unwind(AssertUnwindSafe(|| {
                    for _ in 0..config.warmup_iterations {
                        let _ = service.search(principal, request);
                    }
                }));
                warmed_up.wait();
                warmup.ok()?;

                let started = Instant::now();
                let mut latencies = Vec::new();
                let mut errors = 0;
                for _ in 0..config.iterations_per_thread {
                    let before = Instant::now();
                    errors += usize::from(service.search(principal, request).is_err());
                    latencies.push(before.elapsed());
                }
                Some((started, Instant::now(), latencies, errors))
            });
            match worker {
                Ok(handle) => workers.push(handle),
                Err(error) => {
                    let _ = launch.set(false);
                    return Err(Dok3Error::Internal(format!(
                        "cannot start load worker: {error}"
                    )));
                }
            }
        }
        let _ = launch.set(true);

        let mut latencies = Vec::new();
        let mut errors = 0;
        let mut started = Instant::now();
        let mut finished = started;
        for worker in workers {
            let (worker_start, worker_end, samples, worker_errors) =
                worker.join().ok().flatten().ok_or_else(|| {
                    Dok3Error::Internal("load worker stopped unexpectedly".into())
                })?;
            if latencies.is_empty() {
                started = worker_start;
                finished = worker_end;
            } else {
                started = started.min(worker_start);
                finished = finished.max(worker_end);
            }
            latencies.extend(samples);
            errors += worker_errors;
        }
        Ok((latencies, errors, finished.duration_since(started)))
    })?;

    latencies.sort_unstable();
    let seconds = elapsed.as_secs_f64();
    Ok(LoadReport {
        total_requests,
        error_count,
        elapsed_ms: millis(elapsed),
        qps: if elapsed.is_zero() {
            0.0
        } else {
            total_requests as f64 / seconds
        },
        p50_ms: nearest_rank_ms(&latencies, 50),
        p95_ms: nearest_rank_ms(&latencies, 95),
        p99_ms: nearest_rank_ms(&latencies, 99),
        max_ms: millis(latencies[latencies.len() - 1]),
    })
}

fn millis(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn nearest_rank_ms(sorted: &[Duration], percentile: u128) -> f64 {
    let rank = (sorted.len() as u128 * percentile).div_ceil(100) as usize;
    millis(sorted[rank - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_nearest_rank_without_interpolation() {
        let sorted: Vec<_> = (1..=20).map(Duration::from_millis).collect();
        assert_eq!(nearest_rank_ms(&sorted, 50), 10.0);
        assert_eq!(nearest_rank_ms(&sorted, 95), 19.0);
        assert_eq!(nearest_rank_ms(&sorted, 99), 20.0);
    }
}
