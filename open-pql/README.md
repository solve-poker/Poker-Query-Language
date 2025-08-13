# open-pql

> ⚠️ **Work in Progress**: This project is currently under active development and is not yet ready for production use.

[![crates.io](https://img.shields.io/crates/v/open-pql.svg)](https://crates.io/crates/open-pql)
[![Documentation](https://docs.rs/open-pql/badge.svg)](https://docs.rs/open-pql)

The core library for Open PQL (Poker Query Language) - a high-performance Rust implementation for poker analysis and calculations.

## Overview

This crate provides the fundamental building blocks for poker analysis, including:

- **Card and Hand Representation**: Efficient data structures for poker cards, hands, and boards
- **Hand Evaluation**: Fast algorithms for determining hand strength and rankings
- **Range Analysis**: Tools for working with poker hand ranges and filtering
- **Equity Calculations**: Precise equity computations for various poker scenarios
- **PQL Parser**: SQL-like query language parser for poker analysis
- **Game Support**: Texas Hold'em, Omaha, and other poker variants

## Features

- `benchmark` - Enables benchmarking functionality
- `x86` - Optimizations for x86 architecture

## Quick Start

```rust
use open_pql::*;

// Parse and evaluate poker hands
let hero_hand = parse_hand("AsKs")?;
let villain_range = parse_range("QQ+,AK")?;
let board = parse_board("Ah9s2c")?;

// Calculate equity
let equity = calculate_equity_vs_range(&hero_hand, &villain_range, &board)?;
println!("Hero equity: {:.2}%", equity * 100.0);

// Use PQL queries
let query = "select avg(boardsuitcount(river)) from hero='As9s', villain='*', board='2s3sJh', game='holdem'";
let result = execute_pql_query(query)?;
```

## Architecture

The library is organized into several key modules:

- **`base`**: Core poker primitives (cards, ranks, suits, hands)
- **`functions`**: PQL function implementations for analysis
- **`pql_parser`**: Parser for PQL query language
- **`pql_type`**: Type system for PQL values
- **`range_parser`**: Parser for poker hand ranges
- **`vm`**: Virtual machine for executing PQL queries
- **`runner`**: Query execution engine

## Performance

Open PQL is designed for high performance:

- Optimized data structures using bit manipulation
- SIMD instructions for hand evaluation (with `x86` feature)
- Efficient memory layouts for cache performance
- Compile-time optimizations using procedural macros

## Development

### Building

```bash
# Standard build
cargo build

# With all features
cargo build --features "benchmark,x86"

# Release build with optimizations
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run with property-based testing
cargo test --features quickcheck
```

### Benchmarking

```bash
# Run benchmarks (requires benchmark feature)
cargo bench --features benchmark
```

## License

Licensed under the MIT License. See [LICENSE](../LICENSE) for details.
