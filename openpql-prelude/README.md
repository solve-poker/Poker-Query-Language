# openpql-prelude

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)

Core library for card handling and evaluation of Hold'em and Short Deck poker.

## Overview

`openpql-prelude` is the foundational library for poker hand evaluation and card manipulation. It provides efficient data structures and algorithms for working with poker cards, hands, boards, and various game variants.

## Features

- **Card Representation**: Efficient bit-packed card representations
- **Hand Evaluation**: Fast hand strength evaluation for Hold'em and Short Deck
- **Game Variants**: Support for Texas Hold'em, Short Deck (6+), and Omaha
- **Range Operations**: Hand range parsing and manipulation (openpql-range-parser)
- **Equity Calculations**: Efficient equity computations
- **Python Bindings**: Optional Python FFI through PyO3 (with `python` feature)
- **Property Testing**: QuickCheck integration for robust testing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openpql-prelude = "0.1.0"
```

### Optional Features

- `rand` (default): Random number generation for Monte Carlo simulations
- `python`: Python bindings via PyO3
- `quickcheck`: QuickCheck property testing support

## Supported Game Variants

- **Texas Hold'em**: Standard 52-card deck
- **Short Deck (6+)**: 36-card deck (6 through Ace)
- **Omaha**: Four-card hands with 5-card boards

## Performance

This library is optimized for high-performance poker calculations:
- Bit manipulation for card operations
- SIMD-friendly data structures where applicable
- Minimal allocations in hot paths

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Links

- [Main Project Repository](https://github.com/solve-poker/Poker-Query-Language)
- [PQL Documentation](https://pql-docs.solve.poker)
