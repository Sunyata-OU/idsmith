# Benchmarks

This directory contains the core Rust benchmarking suite and the isolated environments used for cross-ecosystem verification.

## Core Rust Benchmarks

These are integrated with Cargo and measure the performance of the underlying Rust library.

**To run:**
```bash
cargo run --release --bin bench_idsmith
```

## Virtual Environments

The following directories contain isolated environments with all necessary dependencies to run the scripts in the `/scripts` directory. These folders are ignored by git.

- **`venv_python/`**: A Python virtual environment containing `python-stdnum` and the local `idsmith` Python bindings.
- **`venv_node/`**: A Node.js environment containing `ibantools`, `card-validator`, and the local `idsmith` Node.js bindings.

## Accuracy and Extended Benchmarking

For cross-library accuracy checks and language-specific performance comparisons, please refer to the [Scripts README](../scripts/README.md).
