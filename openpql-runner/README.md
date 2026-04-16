# openpql-runner

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)

Library and CLI (`opql`) for executing PQL queries.

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

## License

MIT — see [LICENSE](../LICENSE).

## Links

- [Project repo](https://github.com/solve-poker/Poker-Query-Language)
- [PQL docs](https://pql-docs.solve.poker)
- [Playground](https://pql-playground.solve.poker/)
