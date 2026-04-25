# Library Usage

Open PQL's runner can be embedded in a Rust program to evaluate PQL strings without going through the CLI.

## Add the Dependency

```toml
[dependencies]
openpql-runner = "0.1"
```

The library is exposed under the crate name `opql` (the Cargo package is `openpql-runner`, but the library target's name is `opql`).

## Run a Query — Stream Output

`PQLRunner::run` parses, compiles, and evaluates a query, streaming a human-readable report to the writers you provide:

```rust,ignore
use std::io;
use opql::PQLRunner;

fn main() -> io::Result<()> {
    let query = "select avg(equity(hero)) \
                 from game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'";

    PQLRunner::run(query, &mut io::stdout(), &mut io::stderr())
}
```

The first writer receives result rows (one per selector), the second receives parse and runtime errors.

## Run a Query — Structured Output

If you need the per-selector values for further processing, parse the query first and then call `try_run_stmt`:

```rust,ignore
use opql::PQLRunner;
use openpql_pql_parser::parse_pql;

let stmts = parse_pql(
    "select avg(equity(hero)) from game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'",
)?;

for stmt in &stmts {
    let output = PQLRunner::try_run_stmt(stmt)?;
    // output.values, output.n_succ, etc.
}
```

`try_run_stmt` is currently marked as a temporary API in the runner — see the source for the latest shape.

## Parsing Only

The parser crates can be used independently if you want to lint PQL strings, rewrite them, or generate queries programmatically:

```rust,ignore
use openpql_pql_parser::parse_pql;

let stmts = parse_pql(
    "select equity(hero) from game='holdem', hero='AA', villain='KK', board='AhKh2c'",
)?;
```

## Range Parsing

`openpql-range-parser` exposes a parser for range strings like `QQ+, AwKw, 77-55`. Useful for validating user input before passing it into a PQL query.

## API Docs

Auto-generated reference documentation lives at [API Docs](./api.md).
