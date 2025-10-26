# Open PQL (Poker Query Language)

> Made with ♥️

> ⚠️ **Work in Progress**: This project is currently under active development and is not yet ready for production use.

> 🌐 **Try it online**: An interactive demo is available at https://pql-playground.solve.poker/

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)

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
openpql-runner = "0.1.0"
```

### CLI Usage

The `opql` command-line tool provides direct access to PQL functionality:

```bash
# Calculate average board suit count
opql --run "select avg(boardsuitcount(river)) from hero='As9s', villain='*', board='2s3sJh', game='holdem'"

# Analyze equity in specific scenarios
opql --run "select equity from hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'"
```

### WebAssembly Demo

Try Open PQL in your browser with the WebAssembly demo:

```bash
# Install trunk (if not already installed)
cargo install trunk

# Run the WASM demo
trunk serve --config ./open-pql-wasm/Trunk.toml

# Open http://localhost:8080 in your browser
```

## Architecture

This workspace contains the following crates:

- **`openpql-prelude`**: Core poker library for card handling and evaluation of Hold'em and Short Deck poker
- **`openpql-range-parser`**: Parser for poker range notation (e.g., "AA-TT", "89+")
- **`openpql-pql-parser`**: Parser implementation for Poker Query Language (PQL) syntax
- **`openpql-runner`**: Main library and CLI tool (`opql`) for executing PQL queries
- **`openpql-macro`**: Internal procedural macros

## Documentation

- **PQL Guide & Tutorial**: [pql-docs.solve.poker](https://pql-docs.solve.poker)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Special thanks to the original Odds Oracle (propokertools.com) team for pioneering the PQL concept and providing inspiration for this Rust implementation.
