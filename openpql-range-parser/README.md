# openpql-range-parser

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)

Parser for poker range notation.

## Range Syntax

Open PQL uses a **generic, variable-based** syntax — not the classic `AKs`/`AKo` shorthand.

Suit variables: `w`, `x`, `y`, `z`. Same letter = same suit, different letters = different suits.

| Notation | Meaning |
| --- | --- |
| `AsKh` | Exact two cards |
| `AwKw` | Suited AK |
| `AxKy` | Offsuit AK |
| `AK` | Any AK |
| `TT` | Any pocket tens |
| `QQ+` | Pocket pairs QQ+ |
| `88-55` | Pairs from 88 down to 55 |
| `AwJw+` | Suited aces from AJ up |
| `[2,4,6,8,T]A` | A2, A4, A6, A8, AT |
| `*` | Any two cards |

Combine with commas: `AA, KK, AwKw, 77-55`.

## License

MIT — see [LICENSE](../LICENSE).

## Links

- [Project repo](https://github.com/solve-poker/Poker-Query-Language)
- [PQL docs](https://pql-docs.solve.poker)
