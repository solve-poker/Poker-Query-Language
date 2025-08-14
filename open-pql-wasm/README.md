# Open PQL WebAssembly Interface

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-enabled-blue.svg)](https://webassembly.org/)

A web-based interface for Open PQL (Poker Query Language) built with Rust, WebAssembly, and Yew. This package provides a browser-based query editor and execution environment for PQL, enabling poker analysis directly in the browser.

> **Development Preview**: This interface is under active development and may not be suitable for production use.

## Overview

`open-pql-wasm` compiles the Open PQL library to WebAssembly, providing a web-based query interface that runs entirely in the browser. The interface includes a query editor, example queries, and real-time result display.

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [Trunk](https://trunkrs.dev/) for building and serving the WebAssembly application

```bash
# Install trunk
cargo install trunk
```

### Development Server

```bash
# Clone the repository and navigate to the WASM package
cd open-pql-wasm

# Start the development server
trunk serve

# The interface will be available at http://localhost:8080
```

### Building for Production

```bash
# Build optimized WebAssembly bundle
trunk build --release

# Output files will be in the dist/ directory
```

## Usage Examples

The web interface supports all standard PQL queries. Here are some examples you can try:

### Equity Calculations
```pql
SELECT avg(equity(hero, river))
FROM game='holdem', hero='TsAc', villain='JsQs', board='2s3s4s'
```

### Hand Analysis
```pql
SELECT max(flopHandCategory(hero))
FROM game='holdem', hero='7h Ah', board='7s 8h Tc'
```

### Range Analysis
```pql
SELECT COUNT(*)
FROM game='holdem', hero='AA-22', villain='*', board='As9s2c'
```

