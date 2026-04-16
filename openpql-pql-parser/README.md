# openpql-pql-parser

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)

Parser for Poker Query Language (PQL) syntax. Execution lives in `openpql-runner`.

## Example

```rust,ignore
use openpql_pql_parser::parse_pql;

let stmts = parse_pql(
    "select equity from hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'"
)?;
```

## License

MIT — see [LICENSE](../LICENSE).
