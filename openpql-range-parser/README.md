# openpql-range-parser

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)

Parser for poker range notation.

## Overview

`openpql-range-parser` provides parsing and validation for poker hand range notation commonly used in poker analysis. It converts range strings into concrete sets of hands that can be used for equity calculations and analysis.

## Features

- **Range Notation Support**: Parse generic poker range syntax
- **Pair Ranges**: `AA`, `KK-JJ`, `99+`
- **Suited/Offsuit**: `AwKw`, `AxKy`, `AK` (both)
- **Span Notation**: `22+`, `AT+`
- **Board Parsing**: Parse 3 flop cards along with fixed turn and river cards
- **Compact Representation**: Efficient storage of range data

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openpql-range-parser = "0.1.0"
```

## Range Notation Syntax

### Pairs
- `AA` - Pocket aces
- `KK-JJ` - Kings through jacks
- `99+` - Nines or better
- `22-66` - Deuces through sixes

### Suited/Offsuit Hands
- `AwKw` - Ace-king suited
- `AxKy` - Ace-king offsuit
- `AK` - Both suited and offsuit

### Lists
- `[2,4,6,8,T]A` - Represents A2, A4, A6, A8, AT

### Wildcards
- `*` - All possible hands

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Links

- [Main Project Repository](https://github.com/solve-poker/Poker-Query-Language)
- [PQL Documentation](https://pql-docs.solve.poker)
