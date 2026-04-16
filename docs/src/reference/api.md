# API Docs

Rustdoc-generated reference documentation for the workspace crates is built separately and published alongside this book.

- `openpql-runner` — main library and CLI entry points
- `openpql-prelude` — core card and hand types
- `openpql-pql-parser` — PQL grammar and AST
- `openpql-range-parser` — range-notation parser

## Building Locally

```bash
cargo doc --no-deps --workspace --open
```

This generates HTML under `target/doc/`. Open `target/doc/openpql_runner/index.html` to explore the runner's public API.

## Why Separate?

This book is written for humans reading top-to-bottom, while rustdoc is a reference derived from the source. Both are useful; when they disagree, rustdoc is authoritative because it's regenerated from the code on every commit.
