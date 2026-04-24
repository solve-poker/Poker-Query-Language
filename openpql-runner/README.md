# openpql-runner

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)

PQL query executor — library and `opql` CLI.

## CLI

```bash
opql --run "select equity from hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'"
```

## Library

```rust,ignore
use std::io;
use openpql_runner::PQLRunner;

PQLRunner::run(
    "select equity from hero='AhKh', villain='QQ+', board='', game='holdem'",
    &mut io::stdout(),
    &mut io::stderr(),
)?;
```

## Links

- [Repo](https://github.com/solve-poker/Poker-Query-Language) · [Docs](https://pql-docs.solve.poker) · [Playground](https://pql-playground.solve.poker/)

MIT — see [LICENSE](../LICENSE).
