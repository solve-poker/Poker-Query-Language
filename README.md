# Open PQL (Poker Query Language)

> ⚠️ **Work in Progress**: This project is currently under active development and is not yet ready for production use.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

A high-performance Rust implementation of Poker Query Language (PQL), enabling SQL-like queries for poker analysis and calculations. This project is a spiritual successor to the original Java implementation developed by Odds Oracle.

## Overview

Open PQL provides a powerful query language for poker analysis, allowing users to perform complex calculations on poker scenarios using familiar SQL-like syntax. The library supports various poker variants and offers comprehensive hand evaluation, equity calculations, and range analysis capabilities.

## Features

- **Comprehensive Poker Library**: Full-featured poker card and hand evaluation system
- **SQL-like Query Language**: Intuitive PQL syntax for poker analysis
- **High Performance**: Optimized Rust implementation for fast calculations
- **Multi-game Support**: Texas Hold'em and other poker variants
- **Range Analysis**: Advanced hand range evaluation and filtering
- **Equity Calculations**: Precise equity computations for different scenarios

## Quick Start

### Installation

Add Open PQL to your `Cargo.toml`:

```toml
[dependencies]
open-pql = "0.0.3"
```

### CLI Usage

The `opql` command-line tool provides direct access to PQL functionality:

```bash
# Calculate average board suit count
opql -c "select avg(boardsuitcount(river)) from hero='As9s', villain='*', board='2s3sJh', game='holdem'"

# Analyze equity in specific scenarios
opql -c "select equity from hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'"
```

### Library Usage (WIP)

```rust
use open_pql::*;

// Example: Basic poker hand evaluation
let hand = parse_hand("AsKs")?;
let board = parse_board("AhKh2c")?;
let equity = calculate_equity(&hand, &board)?;
```

## Architecture

This workspace contains three main crates:

- **`open-pql`**: Core library with poker logic and PQL implementation
- **`open-pql-macro`**: Procedural macros for compile-time optimizations
- **`opql`**: Command-line interface for interactive PQL queries

## Development

### Building

```bash
# Build all workspace members
cargo build --workspace

# Build with optimizations
cargo build --workspace --release
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run with coverage
./scripts/coverage.sh
```

### Benchmarking

```bash
# Run benchmarks
cargo bench --features benchmark
```

## Documentation

- **API Documentation**: [docs.rs/open-pql](https://docs.rs/open-pql)
- **PQL Guide & Tutorial**: [pql-docs.solve.poker](https://pql-docs.solve.poker)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Special thanks to the original Odds Oracle (propokertools.com) team for pioneering the PQL concept and providing inspiration for this Rust implementation.


