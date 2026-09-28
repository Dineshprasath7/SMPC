# SMPC Framework

A Rust implementation of secure multi-party computation primitives,
currently focusing on Shamir secret sharing and protocol infrastructure.

## Features

- Shamir secret sharing
- Secret reconstruction
- Finite-field arithmetic
- Serialization/deserialization
- Criterion benchmarks
- Unit and integration tests

## Input

The secret is entered as a hexadecimal string. Since it is decoded into bytes, the hex input must contain an even number of characters.

For example:

```text
Secret (hex): 48656c6c6f
```

This represents `Hello`.

During reconstruction, each share consists of an `x` value and its corresponding share data. The `x` value is entered as a decimal number, while the share data is entered as hexadecimal.

## Tech Stack

- Rust
- Serde
- Criterion
- Clap
- Docker

## Benchmark

Secret splitting averages approximately **13.195 µs per iteration**
for the benchmark configuration.

Reconstruct Secret averages approximately **2.1027 µs per iteration**
for the benchmark configuration

[View Criterion Benchmark Report](target/criterion/report/index.html)

## Running

```bash
cargo bench
cargo run -- split
cargo run -- reconstruct
```bash

## Resource-Constrained Execution

The application was tested under different CPU and memory constraints:

| Configuration | CPU   | Memory |
|---------------|-------|--------|
| Minimal       | 1 CPU | 512 MB |
| Low           | 2 CPUs| 1 GB   |
| Medium        | 4 CPUs| 2 GB   |
| High          | 8 CPUs| 4 GB   |

Example:

```bash
docker run --rm --cpus="2" --memory="1g" smpc
