# ProofOfConcept

## Issue
Most companies develop their backend in languages like Javascript, Typescript, or PHP. The problem is that these environments have limitations in raw performance. To counter this, companies must rent powerful servers or set up reverse proxies. This generates significant infrastructure costs that will continue to grow.

## Solution
Replace a part or the entirety of a legacy backend with a new service written in Rust. Rust is a highly optimized compiled language offering strong memory safety guarantees and zero-cost abstractions.

## What to test?
There are 3 endpoints for each language:
1. A simple hello world to test native HTTP routing speed.
2. A CPU-bound loop with one million iterations.
3. A real-world business logic example (JSON parsing, strict validation, data processing, and serialization).

## How to test?
We can't simply use a single request to make a valid benchmark, we need to simulate real-world conditions. We use [oha](https://crates.io/crates/oha), which is an excellent tool to create HTTP stress tests.

After running the server you want to test, you can simply run `oha` by adapting the port.

| Language | Port |
| :--- | ---: |
| **Rust** | 3000 |
| **TypeScript** | 3001 |
| **PHP** | 8080 |

Run the `oha` test with the following command:
```bash
oha -m POST -H "Content-Type: application/json" -d '{"client_id":"f47ac10b-58cc-4372-a567-0e02b2c3d479","email":"contact@entreprise.com","is_active":true,"appointments":[{"id":1,"date":"2026-09-10T10:00:00Z","duration_minutes":60,"rate":50.0}]}' -c 100 -n 100000 [http://127.0.0.1:3000/process](http://127.0.0.1:3000/process)
```
*(Note for Windows PowerShell users: use -d "@payload_example.json" instead of the raw string to avoid escaping issues, or escape double quotes properly).*

### Rust

Install [Rust](https://rust-lang.org/), which includes the package manager `cargo`. Then head to the rust folder:
```bash
cd rust/
```

Run the server in highly optimized release mode:
```bash
cargo run --release
```

### TypeScript

Install [Bun](https://bun.sh/). It is a powerful JavaScript runtime and package manager designed as a faster drop-in replacement for Node.js. Head to the typescript folder:
```bash
cd typescript
```

Install the dependencies and start the server:
```bash
bun install
bun run start
```

### PHP 8.4

To avoid complex local setups, we use [Docker](https://www.docker.com/) to run the PHP environment with Symfony. Ensure Docker is installed. Head to the PHP folder:
```bash
cd php
```

Run the PHP server using Docker with parallel workers enable:
```bash
docker run --rm -p 8080:8000 -e PHP_CLI_SERVER_WORKERS=16 -v ${PWD}:/app -w /app php:8.4-cli php -S 0.0.0.0:8000 -t public
```
*(Note for Linux/macOS users: replace ${PWD} with $(pwd) in the docker command).*

## Real-World Scenario Benchmark

*Test conditions: 100 concurrent connections, complex JSON payload parsing, strict validation, and data aggregation.*

 | Language | Framework | Requests/sec | Avg Latency | p95 Latency | p99 Latency |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Rust** | Axum | 86,730 | 1.14 ms | 1.80 ms | 2.60 ms |
| **TypeScript** | Bun + Fastify | 14,634 | 6.82 ms | 8.30 ms | 8.97 ms |
| **PHP 8.4** | Symfony (Built-in) | 30 | 3,357.80 ms | 3,747.60 ms | 4,164.70 ms |
