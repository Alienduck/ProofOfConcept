# ProofOfConcept

## Issue
Most companies develop their backend in languages like Javascript, Typescript, or PHP. The problem is that these environments have limitations in raw performance. To counter this, companies must rent powerful servers or set up reverse proxies. This generates significant infrastructure costs that will continue to grow.

## Solution
Replace a part or the entirety of a legacy backend with a new service written in Rust. Rust is a highly optimized compiled language offering strong memory safety guarantees and zero-cost abstractions.

## Benchmarks
There are 3 endpoints for each language:
1. A simple hello world to test native HTTP routing speed.
2. A CPU-bound loop with one million iterations.
3. A real-world business logic example (JSON parsing, strict validation, data processing, and serialization).
