# openpql-pql-parser

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)

A parser implementation for Poker Query Language (PQL).

## Example

```rust
use openpql_pql_parser::parse_query;

// Parse a PQL query
let query = "select equity from hero='AA', villain='KK', board='AhKh2c', game='holdem'";
let ast = parse_pql(query)?;

// Use the AST to execute the query
// (execution logic is provided by openpql-runner)
```
