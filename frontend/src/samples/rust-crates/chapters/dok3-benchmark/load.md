# Module dok3-benchmark::load

## Overview

Concurrent search load with per-worker warmup and nearest-rank latency percentiles.

## Items

- `run_load` — Repeats the same request using the supplied principal on every worker. Timed attempts, including errors, contribute to throughput and latency.

### fn dok3-benchmark::load::run_load

```rust
pub fn run_load(
    service: Arc<SearchService>,
    principal: &Principal,
    request: &SearchRequest,
    config: LoadConfig,
) -> Result<LoadReport, Dok3Error>
```

Repeats the same request using the supplied principal on every worker.
Timed attempts, including errors, contribute to throughput and latency.

### struct dok3-benchmark::load::LoadConfig

```rust
pub struct LoadConfig {
    pub threads: usize,
    pub iterations_per_thread: usize,
    pub warmup_iterations: usize,
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct dok3-benchmark::load::LoadReport

```rust
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
```

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`
