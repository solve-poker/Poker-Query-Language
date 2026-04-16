# Library Usage

Open PQL's runner can be embedded in a Rust program to evaluate PQL strings without shelling out to the CLI.

## Add the Dependency

```toml
[dependencies]
openpql-runner = "0.1.0"
```

## Run a Query

```rust,ignore
use std::io;
use openpql_runner::PQLRunner;

fn main() -> io::Result<()> {
    let query = "select equity from hero='AhKh', villain='QQ+', \
                 board='Ah9s2c', game='holdem'";

    PQLRunner::run(query, &mut io::stdout(), &mut io::stderr())
}
```

`PQLRunner::run` parses, compiles, and evaluates the query, streaming a human-readable report to the provided writers. Use `PQLRunner::try_run_stmt` if you want to handle the structured output yourself.

## Parsing Only

The parser crates can be used independently:

```rust,ignore
use openpql_pql_parser::parse_pql;

let stmts = parse_pql(
    "select equity from hero='AA', villain='KK', board='AhKh2c', game='holdem'"
)?;
```

Use this if you want to lint PQL strings in an editor, rewrite them, or generate queries programmatically.

## Range Parsing

`openpql-range-parser` exposes a parser for range strings like `QQ+, AwKw, 77-55`. Useful for validating user input before passing it into a PQL query.

## API Docs

Auto-generated API documentation lives at [API Docs](./api.md).
