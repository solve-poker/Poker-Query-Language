# openpql-range-parser

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)

Parser for poker range notation.

## Syntax

Variable-based, not the classic `AKs`/`AKo` shorthand. Suit variables `w`, `x`, `y`, `z` — same letter = same suit.

| Notation | Meaning |
| --- | --- |
| `AsKh` | Exact two cards |
| `AwKw` | Suited AK |
| `AxKy` | Offsuit AK |
| `AK` | Any AK |
| `QQ+` | Pocket pairs QQ+ |
| `88-55` | Pairs 88 down to 55 |
| `AwJw+` | Suited aces AJ+ |
| `[2,4,6,8,T]A` | A2, A4, A6, A8, AT |
| `*` | Any two cards |

Combine with commas: `AA, KK, AwKw, 77-55`.

## Links

- [Repo](https://github.com/solve-poker/Poker-Query-Language) · [Docs](https://pql-docs.solve.poker)

MIT — see [LICENSE](../LICENSE).
