# ProofOfConcept

## Issue
Most companies develop their backend in languages like Javascript, Typescript, or PHP. The problem is that these environments have limitations in raw performance. To counter this, companies must rent powerful servers or set up reverse proxies. This generates significant infrastructure costs that will continue to grow.

## Solution
Replace a part or the entirety of a legacy backend with a new service written in Rust. Rust is a highly optimized compiled language offering strong memory safety guarantees and zero-cost abstractions.

## Benchmarks

### What to test ?
There are 3 endpoints for each language:
1. A simple hello world to test native HTTP routing speed.
2. A CPU-bound loop with one million iterations.
3. A real-world business logic example (JSON parsing, strict validation, data processing, and serialization).

### How to test ?
We can't simply use a simple request to make a benchmark, we need a real confition, so i'm using [oha](https://crates.io/crates/oha) which is a beautiful tool to create stress-test

## Real-World Scenario Benchmark

*Test conditions: 100 concurrent connections, 100,000 total requests processing a complex JSON payload with strict validation and data aggregation.*

| Language | Framework | Requests/sec | Avg Latency | p95 Latency | p99 Latency |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Rust** | Axum | 86,730 | 1.14 ms | 1.80 ms | 2.60 ms |
| **TypeScript** | Bun + Fastify | 14,634 | 6.82 ms | 8.30 ms | 8.97 ms |
| **PHP** | *TBD* | *TBD* | *TBD* | *TBD* | *TBD* |
