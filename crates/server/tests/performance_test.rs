//! Performance and SLA benchmark test suite validating p95 latency and concurrent task execution.

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::TestHarness;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use std::time::Instant;

#[tokio::test]
async fn test_perf_latency_sla_p95() {
    let harness = TestHarness::new().await;
    let iterations = 200;
    let mut latencies_micros = Vec::with_capacity(iterations);

    // Warm-up requests
    for _ in 0..10 {
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let _ = harness.send_request(req).await;
    }

    // Benchmark loop
    for _ in 0..iterations {
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let start = Instant::now();
        let resp = harness.send_request(req).await;
        let duration = start.elapsed();

        assert_eq!(resp.status(), StatusCode::OK);
        latencies_micros.push(duration.as_micros());
    }

    latencies_micros.sort_unstable();

    let p50_micros = latencies_micros[(iterations as f64 * 0.50) as usize];
    let p95_micros = latencies_micros[(iterations as f64 * 0.95) as usize];
    let p99_micros = latencies_micros[(iterations as f64 * 0.99) as usize];

    println!(
        "Benchmark (/health): p50 = {} µs, p95 = {} µs, p99 = {} µs",
        p50_micros, p95_micros, p99_micros
    );

    let sla_p95_limit_micros = 50_000;
    assert!(
        p95_micros < sla_p95_limit_micros,
        "p95 latency ({} µs) exceeds SLA limit ({} µs)",
        p95_micros,
        sla_p95_limit_micros
    );
}

#[tokio::test]
async fn test_perf_concurrent_load_50_workers() {
    let harness = Arc::new(TestHarness::new().await);
    let worker_count = 50;
    let mut join_handles = Vec::with_capacity(worker_count);

    let start_total = Instant::now();

    for i in 0..worker_count {
        let h = Arc::clone(&harness);
        join_handles.push(tokio::spawn(async move {
            let req = Request::builder()
                .method("GET")
                .uri("/health")
                .header("x-request-id", format!("concurrent_worker_{i}"))
                .body(Body::empty())
                .unwrap();

            let resp = h.send_request(req).await;
            assert_eq!(resp.status(), StatusCode::OK);
        }));
    }

    for handle in join_handles {
        handle.await.expect("Worker task panicked");
    }

    let elapsed = start_total.elapsed();
    println!(
        "Concurrent load: 50 workers completed in {} ms ({:.2} ms per worker)",
        elapsed.as_millis(),
        elapsed.as_secs_f64() * 1000.0 / (worker_count as f64)
    );

    assert!(
        elapsed.as_millis() < 1000,
        "Concurrency execution exceeded 1s: {:?}",
        elapsed
    );
}
